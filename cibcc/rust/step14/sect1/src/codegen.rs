use crate::parser::Program;
use crate::parser::Node;
use crate::parser::NodeKind;
use std::fmt::Write;

pub fn codegen(prog: Program) -> String {
	let mut output = String::new();
	let mut labelseq: i64 = 0;

	output.push_str(".intel_syntax noprefix\n");
	output.push_str(".global main\n");
	output.push_str("main:\n");
	output.push_str("  push rbp\n");
	output.push_str("  mov rbp, rsp\n");
	writeln!(output, "  sub rsp, {}", prog.stack_size).expect("Failed to write to output");

	let mut var = prog.node;
	while let Some(node) = var {
		gen(&node, &mut output, &mut labelseq);
		var = node.next;
	}

	output.push_str(".Lreturn:\n");
	output.push_str("  mov rsp, rbp\n");
	output.push_str("  pop rbp\n");
	output.push_str("  ret\n");
	output
}

fn gen(node: &Node, output: &mut String, labelseq: &mut i64) {
	match node.kind {
		NodeKind::Num(val) => {
			writeln!(output, "  push {}", val).unwrap();
		}
		NodeKind::ExprStmt => {
			gen(node.lhs.as_ref().unwrap(), output, labelseq);
			output.push_str("  pop rax\n");
		}
		NodeKind::Var => {
			gen_addr(node, output);
			output.push_str("  pop rax\n");
			output.push_str("  mov rax, [rax]\n");
			output.push_str("  push rax\n");
		}
		NodeKind::Assign => {
			gen_addr(node.lhs.as_ref().unwrap(), output);
			gen(node.rhs.as_ref().unwrap(), output, labelseq);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");
			output.push_str("  mov [rax], rdi\n");
			output.push_str("  push rdi\n");
		}
		NodeKind::If => {
			let seq = *labelseq;
			*labelseq += 1;
			if let Some(cond) = &node.cond {
				gen(cond, output, labelseq);
			}
			writeln!(output, "  pop rax").unwrap();
			writeln!(output, "  cmp rax, 0").unwrap();

			if let Some(els) = &node.els {
				writeln!(output, "  je .Lelse{}", seq).unwrap();
				if let Some(then) = &node.then {
					gen(then, output, labelseq);
				}
				writeln!(output, "  jmp .Lend{}", seq).unwrap();
				writeln!(output, ".Lelse{}:", seq).unwrap();
				gen(els, output, labelseq);
			} else {
				writeln!(output, "  je .Lend{}",seq).unwrap();
				if let Some(then) = &node.then {
					gen(then, output, labelseq);
				}
			}
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::While => {
			let seq = *labelseq;
			*labelseq += 1;
			writeln!(output, ".Lbegin{}:", seq).unwrap();
			gen(node.cond.as_ref().unwrap(), output, labelseq);
			writeln!(output, "  pop rax").unwrap();
			writeln!(output, "  cmp rax, 0").unwrap();
			writeln!(output, "  je .Lend{}", seq).unwrap();
			gen(node.then.as_ref().unwrap(), output, labelseq);
			writeln!(output, "  jmp .Lbegin{}", seq).unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::For => {
			let seq = *labelseq;
			*labelseq += 1;
			if let Some(init) = &node.init {
				gen(init, output, labelseq);
			}
			writeln!(output, ".Lbegin{}:", seq).unwrap();
			if let Some(cond) = &node.cond {
				gen(cond, output, labelseq);
				writeln!(output, "  pop rax").unwrap();
				writeln!(output, "  cmp rax, 0").unwrap();
				writeln!(output, "  je .Lend{}", seq).unwrap();
			}
			if let Some(then) = &node.then {
				gen(then, output, labelseq);
			}
			if let Some(inc) = &node.inc {
				gen(inc, output, labelseq);
			}
			writeln!(output, "  jmp .Lbegin{}", seq).unwrap();
			writeln!(output, ".Lend{}:", seq).unwrap();
		}
		NodeKind::Block => {
			let mut var = &node.body;
			while let Some(node) = var {
				gen(&node, output, labelseq);
				var = &node.next
			}
		}
		NodeKind::Funcall => {
			writeln!(output, "  call {}", node.funcname).unwrap();
			writeln!(output, "  push rax").unwrap();
		}
		NodeKind::Return => {
			if let Some(ref lhs) = node.lhs {
				gen(lhs, output, labelseq);
			}
			output.push_str("  pop rax\n");
			output.push_str("  jmp .Lreturn\n");
		}
		NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div |
		NodeKind::EQ  | NodeKind::NE  | NodeKind::LT  | NodeKind::LE  => {
			gen(node.lhs.as_ref().unwrap(), output, labelseq);
			gen(node.rhs.as_ref().unwrap(), output, labelseq);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");

			match node.kind {
				NodeKind::Add => output.push_str("  add rax, rdi\n"),
				NodeKind::Sub => output.push_str("  sub rax, rdi\n"),
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

fn gen_addr(node: &Node, output: &mut String) {
	if let NodeKind::Var = node.kind {
		if let Some(var) = &node.var {
			writeln!(output, "  lea rax, [rbp-{}]", var.offset).unwrap();
		}
		output.push_str("  push rax\n");
	} else {
		panic!("代入の左辺値が変数ではありません");
	}
}

