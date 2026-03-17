import java.util.List;
import java.util.ListIterator;

enum NodeKind {
	ND_ADD,
	ND_SUB,
	ND_MUL,
	ND_DIV,
	ND_EQ,
	ND_NE,
	ND_LT,
	ND_LE,
	ND_ASSIGN,
	ND_ADDR,
	ND_DEREF,
	ND_RETURN,
	ND_IF,
	ND_WHILE,
	ND_FOR,
	ND_SIZEOF,
	ND_BLOCK,
	ND_FUNCALL,
	ND_EXPR_STMT,
	ND_VAR,
	ND_NUM,
	ND_NULL,
}

class Node {
	NodeKind kind;
	Node next;
	Type ty;
	Token tok;
	Node lhs;
	Node rhs;
	Node cond;
	Node then;
	Node els;
	Node init;
	Node inc;
	Node body;
	String funcname;
	Node args;
	Var va;
	int val;

	Node(NodeKind kind, Node lhs, Node rhs) {
		this.kind = kind;
		this.lhs = lhs;
		this.rhs = rhs;
	}

	Node(NodeKind kind, Node lhs) {
		this.kind = kind;
		this.lhs = lhs;
	}

	Node(NodeKind kind, int val) {
		this.kind = kind;
		this.val = val;
	}

	Node(NodeKind kind, Var va) {
		this.kind = kind;
		this.va = va;
	}

	Node(NodeKind kind) {
		this.kind = kind;
	}
}

class Var {
	String name;
	Type ty;
	boolean is_local;
	int offset;
	String contents;
	int cont_len;

	Var(String name) {
		this.name = name;
	}
}

class VarList {
	VarList next;
	Var va;

	VarList head;

	VarList(Var va) {
		this.va = va;
	}
}

class Function {
	Function next;
	String name;
	VarList params;
	Node node;
	VarList locals;
	int stack_size;

	Function(String name, Node node, VarList locals) {
		this.name = name;
		this.node = node;
		this.locals = locals;
	}
}

class Program {
	VarList globals;
	VarList locals;
	Function fns;

	Program(VarList globals, Function fns) {
		this.globals = globals;
		this.fns = fns;
	}
}

public class Parser {
	private Token token;
	private List<Token> tokens;
	private Function fn;

	private ListIterator<Token> iterator;

	public Parser(List<Token> tokens) {
		this.tokens = tokens;
		this.iterator = tokens.listIterator();

		if (iterator.hasNext()) {
			token = iterator.next();
		} else {
			throw new RuntimeException("No tokens provided to parser.");
		}
	}

	VarList locals = null;

	private boolean consume(String op) {
		if (token.type != TokenType.TK_RESERVED || !token.str.equals(op)) {
			return false;
		}
		if (iterator.hasNext()) {
			token = iterator.next();
		} else {
			token = null;
		}
		return true;
	}

	private Token consume_tok(String name) {
		if (token.type != TokenType.TK_RESERVED && token.str != name) {
			return null;
		}

		Token tok = token;

		if (iterator.hasNext()) {
			token = iterator.next();
		} else {
			token = null;
		}
		return tok;
	}

	private void expect(String op) {
		if (!consume(op)) {
			throw new RuntimeException("Expected token '" + op + "'");
		}
	}

	private int expectNumber() {
		if (token.type != TokenType.TK_NUM) {
			throw new RuntimeException("Expected a Number, but get something else");
		}

		int val = token.val;

		if (iterator.hasNext()) {
			token = iterator.next();
		} else {
			token = null;
		}
		return val;
	}

	private boolean isEOF() {
		return token == null || token.type == TokenType.TK_EOF;
	}

	private Token consume_ident() {
		if (token.type != TokenType.TK_IDENT) {
			return null;
		}

		Token tok = token;

		if (iterator.hasNext()) {
			token = iterator.next();
		} else {
			token = null;
		}
		return tok;
	}

	private String expect_ident() {
		if (token.type != TokenType.TK_IDENT) {
			throw new RuntimeException("識別子(identifier)が期待されています。");
		}

		String name = token.str;

		if (iterator.hasNext()) {
			token = iterator.next();
		} else {
			token = null;
		}
		return name;
	}

	private Token peek(String op) {
		if (token.type != TokenType.TK_RESERVED || !token.str.equals(op)) {
			return null;
		}
		return token;
	}

	VarList globals = null;

	private Node newVar(Var va, Token tok) {
		Node node = new Node(NodeKind.ND_VAR, va);
		node.tok = tok;
		return node;
	}

	private Var findVar(String str) {
		for (VarList vl = locals; vl != null; vl = vl.next) {
			Var va = vl.va;
			if (str.equals(va.name)) {
				return va;
			}
		}
		for (VarList vl = globals; vl != null; vl = vl.next) {
			Var va = vl.va;
			if (str.equals(va.name)) {
				return va;
			}
		}
		return null;
	}

	private Var pushVar(String str, Type ty, boolean is_local) {
		Var va = new Var(str);
		va.ty = ty;
		va.is_local = is_local;

		VarList vl = new VarList(va);
		if (is_local) {
			if (locals == null) {
				va.offset = 8;
			} else {
				va.offset = locals.va.offset + 8;
			}
			vl.next = locals;
			locals = vl;
		} else {
			vl.next = globals;
			globals = vl;
		}
		return va;
	}

	private Node read_expr_stmt() {
		return new Node(NodeKind.ND_EXPR_STMT, expr());
	}

	private Node func_args() {
		if (consume(")")) {
			return null;
		}

		Node head = assign();
		Node cur = head;
		while (consume(",")) {
			cur.next = assign();
			cur = cur.next;
		}
		expect(")");
		return head;
	}

	private VarList read_func_params() {
		if (consume(")")) {
			return null;
		}

		VarList head = read_func_param();
		VarList cur = head;

		while (!consume(")")) {
			expect(",");
			cur.next = read_func_param();
			cur = cur.next;
		}
		return head;
	}

	private VarList read_func_param() {
		Type ty = basetype();
		String name = expect_ident();
		ty = read_type_suffix(ty);

		Var va = pushVar(name, ty, true);
		VarList vl =new VarList(va);
		return vl;
	}

	private Node declaration() {
		Type ty = basetype();
		String name = expect_ident();
		ty = read_type_suffix(ty);
		Var va = pushVar(name, ty, true);

		if (consume(";")) {
			return new Node(NodeKind.ND_NULL);
		}

		expect("=");
		Node lhs = new Node(NodeKind.ND_VAR);
		lhs.va = va;
		Node rhs = expr();
		expect(";");
		Node node = new Node(NodeKind.ND_ASSIGN, lhs, rhs);
		return new Node(NodeKind.ND_EXPR_STMT, node);
	}

	private Type basetype() {
		Type ty = new Type();
		if (consume("char")) {
			ty = Type.char_type();
		} else {
			expect("int");
			ty = Type.int_type();
		}
		while (consume("*")) {
			ty = Type.pointer_to(ty);
		}
		return ty;
	}

	private Type read_type_suffix(Type base) {
		if (!consume("[")) {
			return base;
		}
		int sz = expectNumber();
		expect("]");
		base = read_type_suffix(base);
		return Type.array_of(base, sz);
	}

	private void global_var() {
		Type ty = basetype();
		String name =expect_ident();
		ty = read_type_suffix(ty);
		expect(";");
		pushVar(name, ty, false);
	}






	private Function function(Type ty, String name) {
		locals = null;

		Function fn = new Function(name, null, null);
		fn.params = read_func_params();
		expect("{");

		Node head = stmt();
		head.next = null;
		Node cur = head;

		while (!consume("}")) {
			cur.next = stmt();
			cur = cur.next;
		}

		fn.name = name;
		fn.node = head;
		fn.locals = locals;
		return fn;
	}


	public Program parse() {
		Function head = null;
		Function cur = null;
		globals = null;
		locals = null;

		while (!isEOF()) {
			Type ty = basetype();
			String name = expect_ident();
			ty = read_type_suffix(ty);

			if (consume("(")) {
				locals = null;
				Function newFunc = function(ty, name);
				if (head == null) {
					head = newFunc;
					cur = newFunc;
				} else {
					cur.next = newFunc;
					cur = cur.next;
				}
			
			} else {
				pushVar(name, ty, false);
				expect(";");
			}
		}

		Program prog = new Program(globals, head);
		return prog;
	}

	private Node stmt() {
		if (consume("return")) {
			Node node = expr();
			expect(";");

			node = new Node(NodeKind.ND_RETURN, node);
			return node;
		} else if (consume("if")) {
			Node node = new Node(NodeKind.ND_IF);
			expect("(");
			node.cond = expr();
			expect(")");
			node.then = stmt();
			if (consume("else")) {
				node.els = stmt();
			}
			return node;
		} else if (consume("while")) {
			Node node = new Node(NodeKind.ND_WHILE);
			expect("(");
			node.cond = expr();
			expect(")");
			node.then = stmt();
			return node;
		} else if (consume("for")) {
			Node node = new Node(NodeKind.ND_FOR);
			expect("(");
			if (!consume(";")) {
				node.init = read_expr_stmt();
				expect(";");
			}
			if (!consume(";")) {
				node.cond = expr();
				expect(";");
			}
			if (!consume(")")) {
				node.inc = read_expr_stmt();
				expect(")");
			}
			node.then = stmt();
			return node;
		} else if (consume("{")) {
			Node head = stmt();
			head.next = null;
			Node cur = head;

			while (!consume("}")) {
				cur.next = stmt();
				cur = cur.next;
			}

			Node node = new Node(NodeKind.ND_BLOCK);
			node.body = head.next;
			return node;
		} else if (peek("int") != null || peek("char") != null) {
			return declaration();
		} else {
			Node node = new Node(NodeKind.ND_EXPR_STMT, expr());
			expect(";");
			return node;
		}
	}

	private Node expr() {
		return assign();
	}

	private Node assign() {
		Node node = equality();
		if (consume("=")) {
			node = new Node(NodeKind.ND_ASSIGN, node, assign());
		}
		return node;
	}

	private Node equality() {
		Node node = relational();

		while (true) {
			if (consume("==")) {
				node = new Node(NodeKind.ND_EQ, node, relational());
			} else if (consume("!=")) {
				node = new Node(NodeKind.ND_NE, node, relational());
			} else {
				return node;
			}
		}
	}

	private Node relational() {
		Node node = add();

		while (true) {
			if (consume("<")) {
				node = new Node(NodeKind.ND_LT, node, add());
			} else if (consume("<=")) {
				node = new Node(NodeKind.ND_LE, node, add());
			} else if (consume(">")) {
				node = new Node(NodeKind.ND_LT, add(), node);
			} else if (consume(">=")) {
				node = new Node(NodeKind.ND_LE, add(), node);
			} else {
				return node;
			}
		}
	}

	private Node add() {
		Node node = mul();

		while (true) {
			if (consume("+")) {
				node = new Node(NodeKind.ND_ADD, node, mul());
			} else if (consume("-")) {
				node = new Node(NodeKind.ND_SUB, node, mul());
			} else {
				return node;
			}
		}
	}

	private Node mul() {
		Node node = unary();

		while (true) {
			if (consume("*")) {
				node = new Node(NodeKind.ND_MUL, node, unary());
			} else if (consume("/")) {
				node = new Node(NodeKind.ND_DIV, node, unary());
			} else {
				return node;
			}
		}
	}

	private Node unary() {
		if (consume("+")) {
			return unary();
		}
		if (consume("-")) {
			return new Node(NodeKind.ND_SUB, new Node(NodeKind.ND_NUM, 0), unary());
		}
		if (consume("&")) {
			return new Node(NodeKind.ND_ADDR, unary());
		}
		if (consume("*")) {
			return new Node(NodeKind.ND_DEREF, unary());
		}
		return postfix();
	}

	private Node postfix() {
		Node node = primary();

		while (consume("[")) {
			Node exp = new Node(NodeKind.ND_ADD, node, expr());
			expect("]");
			node = new Node(NodeKind.ND_DEREF, exp);
		}
		return node;
	}

	private Node primary() {
		if (consume("(")) {
			Node node = expr();
			expect(")");
			return node;
		}

		Token tok = consume_tok("sizeof");
		if (tok != null) {
			return new Node(NodeKind.ND_SIZEOF, unary());
		}

		tok = consume_ident();
		if (tok != null) {
			if (consume("(")) {
				Node node = new Node(NodeKind.ND_FUNCALL);
				node.funcname = tok.str;
				node.args = func_args();
				return node;
			}

			Var va = findVar(tok.str);
			if (va == null) {
				System.err.println("undefined variable");
			}
			return newVar(va, tok);
		}

		if (token.type == TokenType.TK_STR) {
			Token curtok = token;

			if (iterator.hasNext()) {
				token = iterator.next();
			} else {
				token = null;
			}

			Type ty = Type.array_of(Type.char_type(), curtok.cont_len+1);
			Var va = pushVar(LabelGenerator.newLabel(), ty, false);
			va.contents = curtok.contents;
			va.cont_len = curtok.cont_len;
			return newVar(va, curtok);
		}

		tok = token;
		if (tok.type != TokenType.TK_NUM) {
			System.err.println("expected expression");
		}
		return new Node(NodeKind.ND_NUM, expectNumber());
	}
}

