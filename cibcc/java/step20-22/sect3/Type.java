public class Type {

	enum TypeKind {
		TY_INT,
		TY_PTR,
		TY_ARRAY,
	}

	TypeKind kind;
	Type base;
	int array_size;

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
		case ND_FUNCALL:
		case ND_NUM:
			node.ty = int_type();
			return;
		case ND_VAR:
			if (node.va == null) {
				System.err.println("Error: ND_VAR node has no Var infomation (va is null).");
				return;
			}
			node.ty = node.va.ty;
			return;
		case ND_ADD:
			if (node.lhs.ty == null || node.rhs.ty == null) {
				System.err.println("Error: right hand side or ADD has no type info. kind: " + node.rhs.kind);
				return;
			}
			if (node.rhs.ty.base != null) {
				Node tmp = node.lhs;
				node.lhs = node.rhs;
				node.rhs = tmp;
			}
			if (node.rhs.ty.base != null) {
				System.out.println("invalid pointer arithmetic operands");
				return;
			}
			node.ty = node.lhs.ty;
			return;
		case ND_SUB:
			if (node.lhs.ty == null || node.rhs.ty == null) {
				System.err.println("Error: SUB operands lack type info.");
				return;
			}
			if (node.rhs.ty.base != null) {
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
			if (node.lhs.ty.kind == TypeKind.TY_ARRAY) {
				node.ty = pointer_to(node.lhs.ty.base);
			} else {
				node.ty = pointer_to(node.lhs.ty);
			}
			return;
		case ND_DEREF:
			if (node.lhs.ty.base == null) {
				System.out.println("invalid pointer dereference");
			}
			node.ty = node.lhs.ty.base;
			return;
		case ND_SIZEOF:
			node.kind = NodeKind.ND_NUM;
			node.ty = int_type();
			node.val = size_of(node.lhs.ty);
			node.lhs = null;
			return;
		}
	}

	public static Type int_type() {
		Type ty = new Type(TypeKind.TY_INT);
		return ty;
	}

	public static Type pointer_to(Type base) {
		Type ty = new Type(TypeKind.TY_PTR, base);
		ty.base = base;
		return ty;
	}

	public static Type array_of(Type base, int size) {
		Type ty = new Type(TypeKind.TY_ARRAY, base);
		ty.array_size = size;
		return ty;
	}

	public static int size_of(Type ty) {
		if (ty.kind == TypeKind.TY_INT || ty.kind == TypeKind.TY_PTR) {
			return 8;
		}
		assert(ty.kind == TypeKind.TY_ARRAY);
		return size_of(ty.base) * ty.array_size;
	}
}

