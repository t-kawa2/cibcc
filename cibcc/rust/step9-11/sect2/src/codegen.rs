use crate::parser::Node;
use crate::parser::NodeKind;

pub fn codegen(node: Box<Node>) -> String {
	let mut output = String::new();

	match node.kind {
		NodeKind::Num(val) => {
			output.push_str(&format!("  push {}\n", val));
		}
		NodeKind::Return => {
			if let Some(lhs) = node.lhs {
				output.push_str(&codegen(lhs));
			}
			output.push_str("  pop rax\n");
			output.push_str("  ret\n");
		}
		NodeKind::Add | NodeKind::Sub | NodeKind::Mul | NodeKind::Div |
		NodeKind::EQ  | NodeKind::NE  | NodeKind::LT  | NodeKind::LE  => {
			if let (Some(lhs), Some(rhs)) = (node.lhs, node.rhs) {
				output.push_str(&codegen(lhs));
				output.push_str(&codegen(rhs));

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
					_ => unreachable!(),
				}
				output.push_str("  push rax\n");
			}
		}
	}
	output
}

