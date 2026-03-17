import java.util.ArrayList;
import java.util.List;

enum TokenType {
	TK_RESERVED,
	TK_NUM,
	TK_EOF,
}

class Token {
	TokenType type;
	String str;
	int val;
	int len;

	Token(TokenType type, String str) {
		this.type = type;
		this.str = str;
	}

	Token(TokenType type, String str, int len, int val) {
		this.type = type;
		this.str = str;
		this.len = len;
		this.val = val;
	}
}

public class Tokenize {

	private String input;

	public Tokenize(String input) {
		this.input = input;
	}

	public List<Token> tokenize() {
		List<Token> tokens = new ArrayList<>();
		int i = 0;

		while (i < input.length()) {

			if (Character.isWhitespace(input.charAt(i))) {
				i++;
				continue;
			}

			if (input.charAt(i) == '+' || input.charAt(i) == '-' ||
				input.charAt(i) == '*' || input.charAt(i) == '/' ||
				input.charAt(i) == '(' || input.charAt(i) == ')') {
				String s = String.valueOf(input.charAt(i));
				tokens.add(new Token(TokenType.TK_RESERVED, s));
				i++;
				continue;
			}

			if (Character.isDigit(input.charAt(i))) {
				int start = i;
				while (i < input.length() && Character.isDigit(input.charAt(i))) {
					i++;
				}
				String numStr = input.substring(start, i);
				int val = Integer.parseInt(numStr);
				tokens.add(new Token(TokenType.TK_NUM, numStr, numStr.length(), val));
				continue;
			}

			throw new RuntimeException("トークン化できません: " + input.substring(i));
		}

		tokens.add(new Token(TokenType.TK_EOF, "EOF"));
		return tokens;
	}
}	

