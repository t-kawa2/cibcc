use crate::parser::Node;
use crate::parser::NodeKind;
use std::fmt::Write;

pub fn codegen(head: Box<Node>) -> String {
	let mut output = String::new();

	output.push_str(".intel_syntax noprefix\n");
	output.push_str(".global main\n");
	output.push_str("main:\n");
	output.push_str("  push rbp\n");
	output.push_str("  mov rbp, rsp\n");
	output.push_str("  sub rsp, 208\n");

	let mut cur = Some(head);
	while let Some(n) = cur {
		gen(&n, &mut output);
		output.push_str("  pop rax\n");
		cur = n.next;
	}

	output.push_str(".Lreturn:\n");
	output.push_str("  mov rsp, rbp\n");
	output.push_str("  pop rbp\n");
	output.push_str("  ret\n");
	output
}

fn gen(node: &Node, output: &mut String) {
	match node.kind {
		NodeKind::Num(val) => {
			writeln!(output, "  push {}", val).unwrap();
		}
		NodeKind::ExprStmt => {
			if let Some(ref lhs) = node.lhs {
				gen(lhs, output);
				output.push_str("  pop rax\n");
			}
		}
		NodeKind::LVar => {
			gen_addr(node, output);
			output.push_str("  pop rax\n");
			output.push_str("  mov rax, [rax]\n");
			output.push_str("  push rax\n");
		}
		NodeKind::Assign => {
			gen_addr(node.lhs.as_ref().unwrap(), output);
			gen(node.rhs.as_ref().unwrap(), output);
			output.push_str("  pop rdi\n");
			output.push_str("  pop rax\n");
			output.push_str("  mov [rax], rdi\n");
			output.push_str("  pop rdi\n");
		}
		NodeKind::Return => {
			if let Some(ref lhs) = node.lhs {
				gen(lhs, output);
			}
			output.push_str("  pop rax\n");
			output.push_str("  jmp .Lreturn\n");
		}
		NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div |
		NodeKind::EQ  | NodeKind::NE  | NodeKind::LT  | NodeKind::LE  => {
			gen(node.lhs.as_ref().unwrap(), output);
			gen(node.rhs.as_ref().unwrap(), output);
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
	if let NodeKind::LVar = node.kind {
		let first_char = node.name.as_bytes()[0];
		let offset = (first_char - b'a' + 1) as i32 * 8;

		writeln!(output, "  lea rax, [rbp-{}]", offset).unwrap();
		output.push_str("  push rax\n");
	} else {
		panic!("代入の左辺値が変数ではありません");
	}
}

