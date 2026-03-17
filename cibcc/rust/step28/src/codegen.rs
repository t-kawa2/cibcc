use crate::parser::Function;
use crate::parser::Node;
use crate::parser::NodeKind;
use crate::parser::Program;
use crate::r_type::TypeKind;
use std::fmt::Write;

pub fn codegen(prog: Program) -> String {
	let mut output = String::new();
	let mut labelseq: i64 = 0;
	output.push_str(".intel_syntax noprefix\n");
	emit_data(&prog, &mut output);
	emit_text(&prog, &mut output, &mut labelseq);

	output
}

fn emit_data(prog: &Program, output: &mut String) {
	output.push_str(".data\n");
	let mut g_ptr = prog.globals.as_ref();
	while let Some(vl) = g_ptr {
		let v = &vl.var.borrow();
		output.push_str(&format!("{}:\n", v.name));
		if v.contents.is_empty() {
			output.push_str(&format!("  .zero {}\n", v.ty.size()));
		} else {
			for i in 0..v.cont_len {
				let b = v.contents.as_bytes()[i as usize];
				output.push_str(&format!("  .byte {}\n", b));
			}
		}
		g_ptr = vl.next.as_deref();
	}
}

fn emit_text(prog: &Program, output: &mut String, labelseq: &mut i64) {
	output.push_str(".text\n");
	let mut f_ptr = prog.fns.as_ref();

	while let Some(f) = f_ptr {
		output.push_str(&format!(".global {}\n", f.name));
		output.push_str(&format!("{}:\n", f.name));

		output.push_str("  push rbp\n");
		output.push_str("  mov rbp, rsp\n");
		output.push_str(&format!("  sub rsp, {}\n", f.stack_size));

		let argreg8 = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];
		let argreg1 = ["dil", "sil", "dl", "cl", "r8b", "r9b"];

		let mut params_vec = Vec::new();
		let mut p_ptr = f.params.as_deref();
		while let Some(vl) = p_ptr {
			params_vec.push(&vl.var);
			p_ptr = vl.next.as_deref();
		}

		for (i, var_rc) in params_vec.iter().enumerate() {
			if i >= 6 { break; }

			let v = var_rc.borrow();

			if v.ty.size() == 8 {
				writeln!(output, "  mov [rbp-{}], {}", v.offset, argreg8[i]).unwrap();
			} else if v.ty.size() == 1 {
				writeln!(output, "  mov [rbp-{}], {}", v.offset, argreg1[i]).unwrap();
			}
		}
		if let Some(ref node) = f.node {
			gen(node, prog, f, output, labelseq);
		}

		output.push_str(&format!(".Lreturn.{}:\n", f.name));
		output.push_str("  mov rsp, rbp\n");
		output.push_str("  pop rbp\n");
		output.push_str("  ret\n");
		f_ptr = f.next.as_deref();
	}
}

fn gen(node: &Node, prog: &Program, f: &Function, output: &mut String, labelseq: &mut i64) {
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
			if let Some(lhs) = &node.lhs {
				gen(lhs, prog, f, output, labelseq);
			}

			return;
		}
		NodeKind::Var => {
			gen_addr(node, prog, &f, output, labelseq);
			if let Some(ty) = &node.ty {
				if ty.kind != TypeKind::Array {
					if ty.size() == 8 {
						output.push_str("  mov rax, [rax]\n");
					} else if ty.size() == 1 {
						output.push_str("  movsx rax, byte ptr [rax]\n");
					}
				}
			}
			output.push_str("  push rax\n");
			return;
		}
		NodeKind::Assign => {
			gen_addr(node.lhs.as_ref().unwrap(), prog, f, output, labelseq);
			output.push_str("  push rax\n");
			gen(node.rhs.as_ref().unwrap(), prog, f, output, labelseq);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");

			let ty_opt = node.ty.as_ref().or_else(|| { 
				node.lhs.as_ref().and_then(|lhs| lhs.ty.as_ref())
			});

			if let Some(ty) = ty_opt {
				if ty.size() == 8 {
					output.push_str("  mov [rax], rdi\n");
				} else if ty.size() == 1 {
					output.push_str("  mov [rax], dil\n");
				}
			} else {
				output.push_str("  mov [rax], rdi\n");
			}
			output.push_str("  push rdi\n");
			return;
		}
		NodeKind::Addr => {
			gen_addr(node.lhs.as_ref().unwrap(), prog, &f, output, labelseq);
			output.push_str("  push rax\n");
			return;
		}
		NodeKind::Deref => {
			gen(node.lhs.as_ref().unwrap(), prog, &f, output, labelseq);
			output.push_str("  pop rax\n");
			if let Some(ref ty) = node.ty {
				if let TypeKind::Array = ty.kind {
				} else {
					if ty.size() == 8 {
						output.push_str("  mov rax, [rax]\n");
					} else if ty.size() == 1 {
						output.push_str("  movsx rax, byte ptr [rax]\n");
					}
				}
			} else {
				eprintln!("Deref node has no Type!");
			}
			output.push_str("  push rax\n");
			return;
		}
		NodeKind::If => {
			let seq = *labelseq;
			*labelseq += 1;
			if let Some(cond) = &node.cond {
				gen(cond, prog, &f, output, labelseq);
			}
			writeln!(output, "  pop rax").unwrap();
			writeln!(output, "  cmp rax, 0").unwrap();

			if let Some(els) = &node.els {
				writeln!(output, "  je .Lelse{}", seq).unwrap();
				if let Some(then) = &node.then {
					gen(then, prog, &f, output, labelseq);
				}
				writeln!(output, "  jmp .Lend{}", seq).unwrap();
				writeln!(output, ".Lelse{}:", seq).unwrap();
				gen(els, prog, &f, output, labelseq);
			} else {
				writeln!(output, "  je .Lend{}",seq).unwrap();
				if let Some(then) = &node.then {
					gen(then, prog, &f, output, labelseq);
				}
			}
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::While => {
			let seq = *labelseq;
			*labelseq += 1;
			writeln!(output, ".Lbegin{}:", seq).unwrap();
			gen(node.cond.as_ref().unwrap(), prog, &f, output, labelseq);
			writeln!(output, "  pop rax").unwrap();
			writeln!(output, "  cmp rax, 0").unwrap();
			writeln!(output, "  je .Lend{}", seq).unwrap();
			gen(node.then.as_ref().unwrap(), prog, &f, output, labelseq);
			writeln!(output, "  jmp .Lbegin{}", seq).unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::For => {
			let seq = *labelseq;
			*labelseq += 1;
			if let Some(init) = &node.init {
				gen(init, prog, &f, output, labelseq);
			}
			writeln!(output, ".Lbegin{}:", seq).unwrap();
			if let Some(cond) = &node.cond {
				gen(cond, prog, &f, output, labelseq);
				writeln!(output, "  pop rax").unwrap();
				writeln!(output, "  cmp rax, 0").unwrap();
				writeln!(output, "  je .Lend{}", seq).unwrap();
			}
			if let Some(then) = &node.then {
				gen(then, prog, &f, output, labelseq);
			}
			if let Some(inc) = &node.inc {
				gen(inc, prog, &f, output, labelseq);
			}
			writeln!(output, "  jmp .Lbegin{}", seq).unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::Block | NodeKind::StmtExpr => {
			let mut cur = node.body.as_deref();
			while let Some(n) = cur {
				let is_last = n.next.is_none();
				gen(n, prog, f, output, labelseq);

				let keep_value = (node.kind == NodeKind::StmtExpr) && is_last;

				if !keep_value {
					if n.kind == NodeKind::ExprStmt {
						writeln!(output, "  add rsp, 8").unwrap();
					}
				}
				cur = n.next.as_deref();
			}
			return;
		}
		NodeKind::Funcall => {
			let mut nargs = 0;
			let argreg = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

			let mut cur = &node.args;
			while let Some(n) = cur {
				if NodeKind::ExprStmt == n.kind {
					if let Some(lhs) = &n.lhs {
						gen(lhs, prog, &f, output, labelseq);
					}
				} else {
					gen(n, prog, &f, output, labelseq);
				}
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
			return;
		}
		NodeKind::Return => {
			if let Some(ref lhs) = node.lhs {
				gen(lhs, prog, f, output, labelseq);
				output.push_str("  pop rax\n");
			}
			output.push_str(&format!("  jmp .Lreturn.{}\n", f.name));
			return;
		}
		NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div |
		NodeKind::EQ  | NodeKind::NE  | NodeKind::LT  | NodeKind::LE  => {
			gen(node.lhs.as_ref().unwrap(), prog, &f, output, labelseq);
			gen(node.rhs.as_ref().unwrap(), prog, &f, output, labelseq);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");

			match &node.kind {
				NodeKind::Add => {
					let lhs_ty = node.lhs.as_ref().and_then(|n| n.ty.as_ref());
					let rhs_ty = node.rhs.as_ref().and_then(|n| n.ty.as_ref());

					if let Some(lty) = lhs_ty {
						if lty.kind == TypeKind::Ptr || lty.kind == TypeKind::Array {
//						if lty.kind == TypeKind::Ptr {
							if let Some(ref base) = lty.base {
								writeln!(output, "  imul rdi, {}", base.size()).unwrap();
							}
						}
					} else if let Some(rty) = rhs_ty {
						if rty.kind == TypeKind::Ptr || rty.kind == TypeKind::Array {
//						if rty.kind == TypeKind::Ptr {
							if let Some(ref base) = rty.base {
								writeln!(output, "  imul rax, {}", base.size()).unwrap();
							}
						}
					}

					output.push_str("  add rax,rdi\n");
					output.push_str("  push rax\n");
					return;
				}
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
			return;
		}
	}
}

fn gen_addr(node: &Node, prog: &Program, f: &Function, output: &mut String, labelseq: &mut i64) {
	match &node.kind {
		NodeKind::Var => {
			let v_rc = node.var.as_ref().expect("Variable info missing");
			let v = v_rc.borrow();
			if v.is_local {
				writeln!(output, "  lea rax, [rbp-{}]", v.offset).unwrap();
			} else {
				writeln!(output, "  lea rax, {}[rip]", v.name).unwrap();
			}
		}
		NodeKind::Deref => {
			gen(node.lhs.as_ref().unwrap(), prog, f, output, labelseq);
			output.push_str("  pop rax\n");
		}
		_ => panic!("Not an lvalue"),
	}
}

fn _gen_lvar(node: &Node, prog: &Program, f: &Function, output: &mut String, labelseq: &mut i64) {
	if let Some(node_ty) = node.ty.as_ref() {
		if node_ty.kind == TypeKind::Array {
			eprintln!("not an ivalue");
		}
	}
	gen_addr(node, prog, f, output, labelseq);
}

