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
	ND_RETURN,
	ND_EXPR_STMT,
	ND_LVAR,
	ND_NUM,
}

class Node {
	NodeKind kind;
	Node next;
	Node lhs;
	Node rhs;
	String name;
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

	Node(NodeKind kind, String name) {
		this.kind = kind;
		this.name = name;
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


	public Node parse() {
		Node head = stmt();
		Node cur = head;

		while (!isEOF()) {
			cur.next = stmt();
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
			return new Node(NodeKind.ND_LVAR, tok.str);
		}

		return new Node(NodeKind.ND_NUM, expectNumber());
	}
}

