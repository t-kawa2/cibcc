import java.util.List;

public class Cib {

	public static void main(String[] args) {

		if (args.length != 1) {
			System.out.println("引数の個数が正しくありません");
			return;
		}

		String code = args[0];

		Tokenize tokenize = new Tokenize(code);
		List<Token> tokens = tokenize.tokenize();

		Parser parser = new Parser(tokens);
		Node node = parser.parse();

		Codegen codegen = new Codegen();
		codegen.codegen(node);
	}
}

