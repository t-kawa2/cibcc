labelseq = 0
argreg = ['rdi', 'rsi', 'rdx', 'rcx', 'r8', 'r9']
def gen(node):
	global labelseq
	global argreg
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
	elif node.kind == 'ADDR':
		gen_addr(node.lhs)
		return
	elif node.kind == 'DEREF':
		gen(node.lhs)
		load()
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
	elif node.kind == 'FOR':
		seq = labelseq
		labelseq += 1
		if node.init:
			gen(node.init)
		print(f".Lbegin{seq}:")
		if node.cond:
			gen(node.cond)
			print("  pop rax")
			print("  cmp rax, 0")
			print(f"  je .Lend{seq}")
		gen(node.then)
		if node.inc:
			gen(node.inc)
		print(f"  jmp .Lbegin{seq}")
		print(f".Lend{seq}:")
		return
	elif node.kind == 'BLOCK':
		n = node.body
		while n:
			gen(n)
			n = n.next
		return
	elif node.kind == 'FUNCALL':
		nargs = 0
		arg = node.args
		while arg:
			gen(arg)
			nargs += 1
			arg = arg.next
		for i in range(nargs - 1, -1, -1):
			print(f"  pop {argreg[i]}")
		seq = labelseq
		labelseq += 1
		print("  mov rax, rsp")
		print("  and rax, 15")
		print(f"  jnz .Lcall{seq}")
		print("  mov rax, 0")
		print(f"  call {node.funcname}")
		print(f"  jmp .Lend{seq}")
		print(f".Lcall{seq}:")
		print("  sub rsp, 8")
		print("  mov rax, 0")
		print(f"  call {node.funcname}")
		print("  add rsp, 8")
		print(f".Lend{seq}:")
		print("  push rax")
		return
	elif node.kind == 'RETURN':
		if node.lhs:
			gen(node.lhs)
			print("  pop rax")
		print(f"  jmp .Lreturn.{current_fn_name}")
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

current_fn_name = ""
def codegen(prog):
	global current_fn_name
	print(".intel_syntax noprefix")
	fn = prog
	while fn:
		print(f".global {fn.name}")
		print(f"{fn.name}:")
		current_fn_name = fn.name

		print("  push rbp")
		print("  mov rbp, rsp")
		print(f"  sub rsp, {fn.stack_size}")

		i = 0
		vl = fn.params
		while vl:
			var = vl.var
			print(f"  mov [rbp-{var.offset}], {argreg[i]}")
			i += 1
			vl = vl.next

		node = fn.node
		while node:
			gen(node)
			node = node.next

		print(f".Lreturn.{current_fn_name}:")
		print("  mov rsp, rbp")
		print("  pop rbp")
		print("  ret")

		fn = fn.next

def gen_addr(node):
	if node.kind == 'VAR':
		print(f"  lea rax, [rbp-{node.var.offset}]")
		print("  push rax")
		return
	elif node.kind == 'DEREF':
		gen(node.lhs)
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

