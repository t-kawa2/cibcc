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
	ND_BLOCK,
	ND_FUNCALL,
	ND_EXPR_STMT,
	ND_VAR,
	ND_NUM,
}

class Node {
	NodeKind kind;
	Node next;
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
	Var next;
	String name;
	int offset;

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

public class Parser {
	private List<Token> tokens;

	private ListIterator<Token> iterator;

	public Parser(List<Token> tokens) {
		this.tokens = tokens;
		this.iterator = tokens.listIterator();
	}

	private boolean consume(String op) {
		if (!iterator.hasNext()) return false;
		Token current = iterator.next();
		if (current.type != TokenType.TK_RESERVED || !current.str.equals(op)) {
			iterator.previous();
			return false;
		}
		return true;
	}

	private void expect(String op) {
		if (!consume(op)) {
			throw new RuntimeException("Expected token '" + op + "'");
		}
	}

	private int expectNumber() {
		if (!iterator.hasNext()) throw new RuntimeException("Expected a Number");
		Token current = iterator.next();
		if (current.type != TokenType.TK_NUM) {
			throw new RuntimeException("Expected a Number, but get something else");
		}
		return current.val;
	}

	private boolean isEOF() {
		if (!iterator.hasNext()) return false;
		Token current = iterator.next();
		iterator.previous();
		return current.type == TokenType.TK_EOF;
	}

	private Token consume_ident() {
		Token current = iterator.next();
		iterator.previous();
		if (current.type != TokenType.TK_IDENT) {
			return null;
		} else {
			Token tok = current;
			current = iterator.next();
			return tok;
		}
	}

	private String expect_ident() {
		Token current = iterator.next();
		iterator.previous();
		if (current.type != TokenType.TK_IDENT) {
			return null;
		} else {
			Token tok = current;
			current = iterator.next();
			return tok.str;
		}
	}

	private Node newVar(Var va) {
		return new Node(NodeKind.ND_VAR, va);
	}

	private Var findVar(String str) {
		for (VarList vl = locals; vl != null; vl = vl.next) {
			Var va = vl.va;
			if (str.equals(va.name)) {
				return va;
			}
		}
		return null;
	}

	private Var pushVar(String str) {
		Var va = new Var(str);
		VarList vl = new VarList(va);
		vl.next = locals;
		locals = vl;
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

		VarList head = new VarList(pushVar(expect_ident()));
		VarList cur = head;

		while (!consume(")")) {
			expect(",");
			cur.next = new VarList(pushVar(expect_ident()));
			cur = cur.next;
		}
		return head;
	}






	VarList locals;

	private Function function() {
		locals = null;

		Function fn = new Function(null, null, null);
		fn.name = expect_ident();
		expect("(");
		fn.params = read_func_params();
		expect("{");

		Node head = new Node(null);
		head.next = null;
		Node cur = head;
		while (!consume("}")) {
			cur.next = stmt();
			cur = cur.next;
		}

		fn.node = head.next;
		fn.locals = locals;
		return fn;
	}


	public Function parse() {
		locals = null;

		Function head = function();
		head.next = null;
		Function cur = head;

		while (!isEOF()) {
			cur.next = function();
			cur = cur.next;
		}
		return head;
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
		return primary();
	}

	private Node primary() {
		if (consume("(")) {
			Node node = expr();
			expect(")");
			return node;
		}

		Token tok = consume_ident();
		if (tok != null) {
			if (consume("(")) {
				Node node = new Node(NodeKind.ND_FUNCALL);
				node.funcname = tok.str;
				node.args = func_args();
				return node;
			}

			Var va = findVar(tok.str);
			if (va == null) {
				va = pushVar(tok.str);
			}
			return newVar(va);
		}

		return new Node(NodeKind.ND_NUM, expectNumber());
	}
}

