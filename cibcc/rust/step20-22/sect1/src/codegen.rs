use crate::parser::Function;
use crate::parser::Node;
use crate::parser::NodeKind;
use crate::parser::VarList;
use crate::r_type::TypeKind;
use std::fmt::Write;

pub fn codegen(prog: Function) -> String {
	let mut output = String::new();
	let mut labelseq: i64 = 0;
	output.push_str(".intel_syntax noprefix\n");

	let mut curr_fn = Some(Box::new(prog));
	while let Some(mut f) = curr_fn {
		assign_var_offset(&mut f);
		writeln!(output, ".global {}", f.name).unwrap();
		writeln!(output, "{}:", f.name).unwrap();

		output.push_str("  push rbp\n");
		output.push_str("  mov rbp, rsp\n");
		writeln!(output, "  sub rsp, {}", f.stack_size).unwrap();

		let argreg = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
		let mut cur_param = f.params.as_ref();
		let mut i = 0;

		while let Some(param_list) = cur_param {
			if let Some(ref var) = param_list.var {
				writeln!(output, "  mov [rbp-{}], {}", var.offset, argreg[i]).unwrap();
				i += 1;
			}
			cur_param = param_list.next.as_ref();
		}
		
		let mut curr_node = f.node.clone();
		while let Some(node) = curr_node {
			gen(&node, &f, &mut output, &mut labelseq);
			curr_node = node.next;
		}

		writeln!(output, ".Lreturn.{}:", f.name).unwrap();
		writeln!(output, "  mov rsp, rbp").unwrap();
		writeln!(output, "  pop rbp").unwrap();
		writeln!(output, "  ret").unwrap();
		curr_fn = f.next.take();
	}
	output
}

fn gen(node: &Node, f: &Function, output: &mut String, labelseq: &mut i64) {
	match &node.kind {
		NodeKind::Null => {
			return;
		}
		NodeKind::Num(val) => {
			writeln!(output, "  mov rax, {}", val).unwrap();
			output.push_str("  push rax\n");
			return;
		}
		NodeKind::ExprStmt => {
			gen(node.lhs.as_ref().unwrap(), &f, output, labelseq);
			output.push_str("  pop rax\n");
			return;
		}
		NodeKind::Var => {
			gen_addr(node, &f, output, labelseq);
			output.push_str("  pop rax\n");
			if let Some(ty) = &node.ty {
				if ty.kind != TypeKind::Array {
					output.push_str("  mov rax, [rax]\n");
				}
			}
			output.push_str("  push rax\n");
			return;
		}
		NodeKind::Assign => {
			gen_addr(node.lhs.as_ref().unwrap(), &f, output, labelseq);
			gen(node.rhs.as_ref().unwrap(), &f, output, labelseq);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");
			output.push_str("  mov [rax], rdi\n");
			output.push_str("  push rdi\n");
			return;
		}
		NodeKind::Addr => {
			gen_addr(node.lhs.as_ref().unwrap(), &f, output, labelseq);
			return;
		}
		NodeKind::Deref => {
			gen(node.lhs.as_ref().unwrap(), &f, output, labelseq);
			output.push_str("  pop rax\n");
			if let Some(ref ty) = node.ty {
				if ty.kind != TypeKind::Array {
					output.push_str("  mov rax, [rax]\n");
				}
			}
			output.push_str("  push rax\n");
			return;
		}
		NodeKind::If => {
			let seq = *labelseq;
			*labelseq += 1;
			if let Some(cond) = &node.cond {
				gen(cond, &f, output, labelseq);
			}
			writeln!(output, "  pop rax").unwrap();
			writeln!(output, "  cmp rax, 0").unwrap();

			if let Some(els) = &node.els {
				writeln!(output, "  je .Lelse{}", seq).unwrap();
				if let Some(then) = &node.then {
					gen(then, &f, output, labelseq);
				}
				writeln!(output, "  jmp .Lend{}", seq).unwrap();
				writeln!(output, ".Lelse{}:", seq).unwrap();
				gen(els, &f, output, labelseq);
			} else {
				writeln!(output, "  je .Lend{}",seq).unwrap();
				if let Some(then) = &node.then {
					gen(then, &f, output, labelseq);
				}
			}
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::While => {
			let seq = *labelseq;
			*labelseq += 1;
			writeln!(output, ".Lbegin{}:", seq).unwrap();
			gen(node.cond.as_ref().unwrap(), &f, output, labelseq);
			writeln!(output, "  pop rax").unwrap();
			writeln!(output, "  cmp rax, 0").unwrap();
			writeln!(output, "  je .Lend{}", seq).unwrap();
			gen(node.then.as_ref().unwrap(), &f, output, labelseq);
			writeln!(output, "  jmp .Lbegin{}", seq).unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::For => {
			let seq = *labelseq;
			*labelseq += 1;
			if let Some(init) = &node.init {
				gen(init, &f, output, labelseq);
			}
			writeln!(output, ".Lbegin{}:", seq).unwrap();
			if let Some(cond) = &node.cond {
				gen(cond, &f, output, labelseq);
				writeln!(output, "  pop rax").unwrap();
				writeln!(output, "  cmp rax, 0").unwrap();
				writeln!(output, "  je .Lend{}", seq).unwrap();
			}
			if let Some(then) = &node.then {
				gen(then, &f, output, labelseq);
			}
			if let Some(inc) = &node.inc {
				gen(inc, &f, output, labelseq);
			}
			writeln!(output, "  jmp .Lbegin{}", seq).unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::Block => {
			let mut var = &node.next;
			while let Some(node) = var {
				gen(&node, &f, output, labelseq);
				var = &node.next
			}
		}
		NodeKind::Funcall => {
			let mut nargs = 0;
			let argreg = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

			let mut cur = &node.args;
			while let Some(n) = cur {
				gen(n, &f, output, labelseq);
				nargs += 1;
				cur = &n.next
			}

			for i in (0..nargs).rev() {
				writeln!(output, "  pop {}", argreg[i]).unwrap();
			}

			let seq = *labelseq;
			*labelseq += 1;
			writeln!(output, "  mov rax, rsp").unwrap();
			writeln!(output, "  and rax, 15").unwrap();
			writeln!(output, "  jnz .Lcall{}", seq).unwrap();
			writeln!(output, "  mov rax, 0").unwrap();
			writeln!(output, "  call {}", node.funcname).unwrap();
			writeln!(output, "  jmp .Lend{}", seq).unwrap();
			writeln!(output, ".Lcall{}:", seq).unwrap();
			writeln!(output, "  sub rsp, 8").unwrap();
			writeln!(output, "  mov rax, 0").unwrap();
			writeln!(output, "  call {}", node.funcname).unwrap();
			writeln!(output, "  add rsp, 8").unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
			writeln!(output, "  push rax").unwrap();
		}
		NodeKind::Return => {
			gen(node.lhs.as_ref().unwrap(), f, output, labelseq);
			output.push_str("  pop rax\n");
			output.push_str(&format!("  jmp .Lreturn.{}\n", f.name));
			return;
		}
		NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div |
		NodeKind::EQ  | NodeKind::NE  | NodeKind::LT  | NodeKind::LE  => {
			gen(node.lhs.as_ref().unwrap(), &f, output, labelseq);
			gen(node.rhs.as_ref().unwrap(), &f, output, labelseq);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");

			match &node.kind {
				NodeKind::Add => {
					let lhs_ty = node.lhs.as_ref().and_then(|n| n.ty.as_ref());
					let rhs_ty = node.rhs.as_ref().and_then(|n| n.ty.as_ref());

					if let Some(lty) = lhs_ty {
						if let Some(ref base) = lty.base {
							writeln!(output, "  imul rdi, {}", base.size()).unwrap();
						}
					}

					if let Some(rty) = rhs_ty {
						if let Some(ref base) = rty.base {
							writeln!(output, "  imul rax, {}", base.size()).unwrap();
						}
					}

					output.push_str("  add rax, rdi\n");
					output.push_str("  push rax\n");
					return;
				},
				NodeKind::Sub => {
					if let Some(ref ty) = node.ty {
						if let Some(ref base) = ty.base {
							writeln!(output, "  imul rdi, {}", base.size()).unwrap();
						}
					}

					output.push_str("  sub rax, rdi\n");
					output.push_str("  push rax\n");
					return;
				},
				NodeKind::Mul => output.push_str("  imul rax, rdi\n"),
				NodeKind::Div => {
								output.push_str("  cqo\n");
								output.push_str("  idiv rdi\n");
						},
				NodeKind::EQ => { output.push_str("  cmp rax, rdi\n");
								output.push_str("  sete al\n");
								output.push_str("  movzb rax, al\n")
						},
				NodeKind::NE => { output.push_str("  cmp rax, rdi\n");
								output.push_str("  setne al\n");
								output.push_str("  movzb rax, al\n")
						},
				NodeKind::LT => { output.push_str("  cmp rax, rdi\n");
								output.push_str("  setl al\n");
								output.push_str("  movzb rax, al\n")
						},
				NodeKind::LE => { output.push_str("  cmp rax, rdi\n");
								output.push_str("  setle al\n");
								output.push_str("  movzb rax, al\n")
						},
				_ => {}
			}
			output.push_str("  push rax\n");
		}
	}
}

fn gen_addr(node: &Node, f: &Function, output: &mut String, labelseq: &mut i64) {
	match &node.kind {
		NodeKind::Var => {
			if let Some(node_var) = &node.var {
				let mut offset = 0;
				let mut cur = &f.locals;
				while let Some(vl) = cur {
					if let Some(ref v) = vl.var {
						if v.name == node_var.name {
							offset = v.offset;
							break;
						}
					}
					cur = &vl.next;
				}
				writeln!(output, "  lea rax, [rbp-{}]", offset).unwrap();
			}
			output.push_str("  push rax\n");
		}
		NodeKind::Deref => {
			if let Some(ref lhs) = node.lhs {
				gen(lhs, f, output, labelseq);
			}
		}
		_ => panic!("代入の左辺値がアドレスを持たない値です"),
	}
}

fn assign_var_offset(f: &mut Function) {
	let mut offset = 0;

	let mut cur = f.params.as_mut();
	while let Some(vl) = cur {
		if let Some(ref mut v) = vl.var {
			offset += 8;
			v.offset = offset;
		}
		cur = vl.next.as_mut();
	}

	let mut cur = f.locals.as_mut();
	while let Some(vl) = cur {
		if let Some(ref mut v) = vl.var {
			let already_assigned = find_param_offset(&f.params, &v.name);
			if let Some(off) = already_assigned {
				v.offset = off;
			} else {
				offset += v.ty.size() as i64;
				v.offset = offset;
			}
		}
		cur = vl.next.as_mut();
	}
	f.stack_size = (offset + 15) / 16 * 16;
}

fn find_param_offset(params: &Option<Box<VarList>>, name: &str) -> Option<i64> {
	let mut cur = params.as_ref();
	while let Some(vl) = cur {
		if let Some(ref v) = vl.var {
			if v.name == name && v.offset != 0 {
				return Some(v.offset);
			}
		}
		cur = vl.next.as_ref();
	}
	None
}

