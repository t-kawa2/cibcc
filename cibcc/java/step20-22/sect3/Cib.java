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
		Function prog = parser.parse();

		for (Function fn = prog; fn != null; fn = fn.next) {

			int offset = 0;
			for (VarList vl = fn.locals; vl != null; vl = vl.next) {
				Var va = vl.va;
				offset += Type.size_of(va.ty);
				va.offset = offset;
			}
			fn.stack_size = offset;
		}

		Type typeAnalyzer = new Type();
		typeAnalyzer.add_type(prog);

		Codegen codegen = new Codegen();
		codegen.codegen(prog);
	}
}

