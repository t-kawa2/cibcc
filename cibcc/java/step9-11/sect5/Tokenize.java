import java.util.ArrayList;
import java.util.List;

enum TokenType {
	TK_RESERVED,
	TK_IDENT,
	TK_NUM,
	TK_EOF,
}

class Token {
	TokenType type;
	String str;
	int val;
	int len;

	Token(TokenType type, String str, int len) {
		this.type = type;
		this.str = str;
		this.len = len;
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

			if (input.charAt(i) == 'r' && input.charAt(i+1) == 'e' &&
				input.charAt(i+2) == 't' && input.charAt(i+3) == 'u' &&
				input.charAt(i+4) == 'r' && input.charAt(i+5) == 'n') {
				String s = input.substring(i, i+6);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 6));
				i += 6;
				continue;
			}

			if ((input.charAt(i) == '=' && input.charAt(i+1) == '=') ||
				(input.charAt(i) == '!' && input.charAt(i+1) == '=') ||
				(input.charAt(i) == '<' && input.charAt(i+1) == '=') ||
				(input.charAt(i) == '>' && input.charAt(i+1) == '=')) {
				String s = input.substring(i, i+2);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 2));
				i += 2;
				continue;
			}

			if (input.charAt(i) == '+' || input.charAt(i) == '-' ||
				input.charAt(i) == '*' || input.charAt(i) == '/' ||
				input.charAt(i) == '(' || input.charAt(i) == ')' ||
				input.charAt(i) == '<' || input.charAt(i) == '>' ||
				input.charAt(i) == ';' || input.charAt(i) == '=') {
				String s = String.valueOf(input.charAt(i));
				tokens.add(new Token(TokenType.TK_RESERVED, s, 1));
				i++;
				continue;
			}

			if (Character.isAlphabetic(input.charAt(i))) {
				int start = i;
				while (Character.isLetterOrDigit(input.charAt(i))) {
					i++;
				}
				String s = input.substring(start, i);
				tokens.add(new Token(TokenType.TK_IDENT, s, i-start));
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

		tokens.add(new Token(TokenType.TK_EOF, "EOF", 0));
		return tokens;
	}
}	

