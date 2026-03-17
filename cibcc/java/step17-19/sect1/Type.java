public class Type {

	enum TypeKind {
		TY_INT,
		TY_PTR,
	}

	TypeKind kind;
	Type base;

	Type(TypeKind kind, Type base) {
		this.kind = kind;
		this.base = base;
	}

	Type(TypeKind kind) {
		this.kind = kind;
	}

	public Type() {}

	public void add_type(Function prog) {
		for (Function fn = prog; fn != null; fn = fn.next) {
			for (Node node = fn.node; node != null; node = node.next) {
				visit(node);
			}
		}
	}

	private void visit(Node node) {
		if (node == null) {
			return;
		}

		visit(node.lhs);
		visit(node.rhs);
		visit(node.cond);
		visit(node.then);
		visit(node.els);
		visit(node.init);
		visit(node.inc);

	for (Node n = node.body; n != null; n = n.next) {
			visit(n);
	}
	for (Node n = node.args; n != null; n = n.next) {
			visit(n);
	}

	switch (node.kind) {
		case ND_MUL:
		case ND_DIV:
		case ND_EQ:
		case ND_NE:
		case ND_LT:
		case ND_LE:
		case ND_VAR:
		case ND_FUNCALL:
		case ND_NUM:
			node.ty = int_type();
			return;
		case ND_ADD:
			if (node.rhs.ty == null) {
				System.err.println("Error: right hand side or ADD has no type info. kind: " + node.rhs.kind);
				return;
			}
			if (node.rhs.ty.kind == TypeKind.TY_PTR) {
				Node tmp = node.lhs;
				node.lhs = node.rhs;
				node.rhs = tmp;
			}
			if (node.rhs.ty.kind == TypeKind.TY_PTR) {
				System.out.println("invalid pointer arithmetic operands");
			}
			node.ty = node.lhs.ty;
			return;
		case ND_SUB:
			if (node.rhs.ty.kind == TypeKind.TY_PTR) {
				System.out.println("invalid pointer arithmetic operands");
			}
			node.ty = node.lhs.ty;
			return;
		case ND_RETURN:
			return;
		case ND_ASSIGN:
			node.ty = node.lhs.ty;
			return;
		case ND_ADDR:
			node.ty = pointer_to(node.lhs.ty);
			return;
		case ND_DEREF:
			if (node.lhs.ty.kind == TypeKind.TY_PTR) {
				node.ty = node.lhs.ty.base;
			} else {
				node.ty = int_type();
			}
			return;
		}
	}

	private Type int_type() {
		Type ty = new Type(TypeKind.TY_INT);
		return ty;
	}

	private Type pointer_to(Type base) {
		Type ty = new Type(TypeKind.TY_PTR, base);
		ty.base = base;
		return ty;
	}
}


