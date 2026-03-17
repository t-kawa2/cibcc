import java.util.List;
import java.util.ListIterator;

enum NodeKind {
	ND_ADD,
	ND_SUB,
	ND_NUM,
}

class Node {
	NodeKind kind;
	Node lhs;
	Node rhs;
	int val;

	Node(NodeKind kind, Node lhs, Node rhs) {
		this.kind = kind;
		this.lhs = lhs;
		this.rhs = rhs;
	}

	Node(NodeKind kind, int val) {
		this.kind = kind;
		this.val = val;
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

	private Node expr() {
		Node node = primary();

		while (true) {
			if (consume("+")) {
				node = new Node(NodeKind.ND_ADD, node, primary());
			} else if (consume("-")) {
				node = new Node(NodeKind.ND_SUB, node, primary());
			} else {
				return node;
			}
		}
	}

	private Node primary() {

		return new Node(NodeKind.ND_NUM, expectNumber());

	}

	public Node parse() {
		if (iterator.hasNext()) iterator.next();
		iterator.previous();

		Node ast = expr();

		if (!isEOF()) {
			throw new RuntimeException("Extra Token remainingafter parsing expression.");
		}
		return ast;
	}
}

