labelseq = 0
def gen(node):
	global labelseq
	if node.kind == 'NUM':
		print(f"  push {node.val}")
		return
	elif node.kind == 'EXPR_STMT':
		gen(node.lhs)
		print("  add rsp, 8")
		return
	elif node.kind == 'VAR':
		gen_addr(node)
		load()
		return
	elif node.kind == 'ASSIGN':
		gen_addr(node.lhs)
		gen(node.rhs)
		store()
		return
	elif node.kind == 'IF':
		seq = labelseq
		labelseq += 1
		if node.els:
			gen(node.cond)
			print("  pop rax")
			print("  cmp rax, 0")
			print(f"  je .Lelse{seq}")
			gen(node.then)
			print(f"  jmp .Lend{seq}")
			print(f".Lelse{seq}:")
			gen(node.els)
			print(f".Lend{seq}:")
		else:
			gen(node.cond)
			print("  pop rax")
			print("  cmp rax, 0")
			print(f"  je .Lend{seq}")
			gen(node.then)
			print(f".Lend{seq}:")
		return
	elif node.kind == 'WHILE':
		seq = labelseq
		labelseq += 1
		print(f".Lbegin{seq}:")
		gen(node.cond)
		print("  pop rax")
		print("  cmp rax, 0")
		print(f"  je .Lend{seq}")
		gen(node.then)
		print(f"  jmp .Lbegin{seq}")
		print(f".Lend{seq}:")
		return
	elif node.kind == 'RETURN':
		gen(node.lhs)
		print("  pop rax")
		print("  jmp .Lreturn")
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

def codegen(prog):
	print(".intel_syntax noprefix")
	print(".global main")
	print("main:")

	print("  push rbp")
	print("  mov rbp, rsp")
	print(f"  sub rsp, {prog.stack_size}")

	cur = prog.node
	while cur:
		gen(cur)
		cur = cur.next

	print(".Lreturn:")
	print("  mov rsp, rbp")
	print("  pop rbp")
	print("  ret")

def gen_addr(node):
	if node.kind == 'VAR':
		print(f"  lea rax, [rbp-{node.var.offset}]")
		print("  push rax")
		return
	raise Exception("not an lvalue")  

def load():
	print("  pop rax")
	print("  mov rax, [rax]")
	print("  push rax")

def store():
	print("  pop rdi")
	print("  pop rax")
	print("  mov [rax], rdi")
	print("  push rdi")

