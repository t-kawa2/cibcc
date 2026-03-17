import java.io.InputStreamReader;

class Token {
	private String source;
	private int start = 0;
	private int current = 0;

	Token(String source) {
		this.source = source;
	}

	public String number() {
		start = current;
		while (isDigit(peek())) advance();
		return source.substring(start, current);
	}

	public boolean isDigit(char c) {
		return c >= '0' && c <= '9';
	}

	public char peek() {
		if (isAtEnd()) return '\0';
		return source.charAt(current);
	}

	public char advance() {
		return source.charAt(current++);
	}

	public boolean isAtEnd() {
		return current >= source.length();
	}
}

public class Cib {

	public static void main(String[] args) {

		String source = args[0];

		if (args.length != 1) {
			System.out.println("引数の個数が正しくありません");
			return;
		}

		System.out.println(".intel_syntax noprefix");
		System.out.println(".global main");
		System.out.println("main:");

		Token token = new Token(source);

		System.out.println("  mov rax, " + token.number());

		while (!token.isAtEnd()) {
			if (token.peek() == '+') {
				token.advance();
				System.out.println("  add rax, " + token.number());
			} else if (token.peek() == '-') {
				token.advance();
				System.out.println("  sub rax, " + token.number());
			}
		}

		System.out.println("  ret");
	}
}

