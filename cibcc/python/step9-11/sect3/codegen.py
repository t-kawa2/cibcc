def gen(node):
	if node.kind == 'NUM':
		print(f"  push {node.val}")
		return
	elif node.kind == 'EXPR_STMT':
		gen(node.lhs)
		print("  add rsp, 8")
		return
	elif node.kind == 'RETURN':
		gen(node.lhs)
		print("  pop rax")
		print("  ret")
		return

	gen(node.lhs)
	gen(node.rhs)

	print("  pop rdi")
	print("  pop rax")

	if node.kind == 'ADD':
		print("  add rax, rdi")
	elif node.kind == 'SUB':
		print("  sub rax, rdi")
	elif node.kind == 'MUL':
		print("  imul rax, rdi")
	elif node.kind == 'DIV':
		print("  cqo")
		print("  idiv rdi")
	elif node.kind == 'EQ':
		print("  cmp rax, rdi")
		print("  sete al")
		print("  movzb rax, al")
	elif node.kind == 'NE':
		print("  cmp rax, rdi")
		print("  setne al")
		print("  movzb rax, al")
	elif node.kind == 'LT':
		print("  cmp rax, rdi")
		print("  setl al")
		print("  movzb rax, al")
	elif node.kind == 'LE':
		print("  cmp rax, rdi")
		print("  setle al")
		print("  movzb rax, al")

	print("  push rax")

def codegen(node):
	print(".intel_syntax noprefix")
	print(".global main")
	print("main:")

	n = node
	while n:
		gen(n)
		n = n.next

	print("  ret")
