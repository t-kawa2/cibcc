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
		Program prog = parser.parse();

		int offset = 0;
		for (Var va = prog.locals; va != null; va = va.next) {
			offset += 8;
			va.offset = offset;
		}
		prog.stack_size = offset;

		Codegen codegen = new Codegen();
		codegen.codegen(prog);
	}
}

