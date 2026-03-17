import java.util.ArrayList;
import java.util.List;

enum TokenType {
	TK_RESERVED,
	TK_IDENT,
	TK_STR,
	TK_NUM,
	TK_EOF,
}

class Token {
	TokenType type;
	Token next;
	String str;
	int val;
	int len;
	String contents;
	int cont_len;

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

			if (input.charAt(i) == 'i' && input.charAt(i+1) == 'f') {
				String s = input.substring(i, i+2);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 2));
				i += 2;
				continue;
			}

			if (input.charAt(i) == 'e' && input.charAt(i+1) == 'l' &&
				input.charAt(i+2) == 's' && input.charAt(i+3) == 'e') {
				String s = input.substring(i, i+4);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 4));
				i += 4;
				continue;
			}

			if (input.charAt(i) == 'w' && input.charAt(i+1) == 'h' &&
				input.charAt(i+2) == 'i' && input.charAt(i+3) == 'l' &&
				input.charAt(i+4) == 'e') {
				String s = input.substring(i, i+5);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 5));
				i += 5;
				continue;
			}

			if (input.charAt(i) == 'f' && input.charAt(i+1) == 'o' &&
				input.charAt(i+2) == 'r') {
				String s = input.substring(i, i+3);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 3));
				i += 3;
				continue;
			}

			if (input.charAt(i) == 'i' && input.charAt(i+1) == 'n' &&
				input.charAt(i+2) == 't') {
				String s = input.substring(i, i+3);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 3));
				i += 3;
				continue;
			}

			if (input.charAt(i) == 's' && input.charAt(i+1) == 'i' &&
				input.charAt(i+2) == 'z' && input.charAt(i+3) == 'e' &&
				input.charAt(i+4) == 'o' && input.charAt(i+5) == 'f') {
				String s = input.substring(i, i+6);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 6));
				i += 6;
				continue;
			}

			if (input.charAt(i) == 'c' && input.charAt(i+1) == 'h' &&
				input.charAt(i+2) == 'a' && input.charAt(i+3) == 'r') {
				String s = input.substring(i, i+4);
				tokens.add(new Token(TokenType.TK_RESERVED, s, 4));
				i += 4;
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
				input.charAt(i) == ';' || input.charAt(i) == '=' ||
				input.charAt(i) == '{' || input.charAt(i) == '}' ||
				input.charAt(i) == ',' || input.charAt(i) == '&' ||
				input.charAt(i) == '[' || input.charAt(i) == ']') {
				String s = String.valueOf(input.charAt(i));
				tokens.add(new Token(TokenType.TK_RESERVED, s, 1));
				i++;
				continue;
			}

			if (Character.isAlphabetic(input.charAt(i))) {
				int start = i;
				while (Character.isLetterOrDigit(input.charAt(i)) || (input.charAt(i) == '_')) {
					i++;
				}
				String s = input.substring(start, i);
				tokens.add(new Token(TokenType.TK_IDENT, s, i-start));
				continue;
			}

			if (input.charAt(i) == '"') {
				int start = i;
				i++;
				while (i < input.length() && input.charAt(i) != '"') {
					i++;
				}

				if (i >= input.length()) {
					throw new RuntimeException("文字列が閉じられていません");
				}

				i++;
				String s = input.substring(start, i);
				String content = s.substring(1, s.length()-1);
				StringBuilder sb = new StringBuilder();
				for (int j = 0; j < content.length(); j++) {
					if (content.charAt(j) == '\\' && j+1 < content.length()) {
						j++;
						switch (content.charAt(j)) {
							case 'a': sb.append((char)7); break;
							case 'b': sb.append('\b'); break;
							case 't': sb.append('\t'); break;
							case 'n': sb.append('\n'); break;
							case 'v': sb.append((char)11); break;
							case 'f': sb.append('\f'); break;
							case 'r': sb.append('\r'); break;
							case 'e': sb.append((char)27); break;
							case '0': sb.append((char)0); break;
							default: sb.append(content.charAt(j));
						}
					} else {
						sb.append(content.charAt(j));
					}
				}

				Token tok = new Token(TokenType.TK_STR, s, i-start);
				tok.contents = sb.toString();
				tok.cont_len = tok.contents.length() + 1;
				tokens.add(tok);
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

