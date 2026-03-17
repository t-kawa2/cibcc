import java.util.List;
import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.nio.charset.Charset;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.charset.StandardCharsets;

public class Cib {

	static boolean hadError = false;

	public static void main(String[] args) throws IOException {

		if (args.length != 1) {
			System.out.println("引数の個数が正しくありません");
			return;
		}

		byte[] bytes = Files.readAllBytes(Paths.get(args[0]));
		if (hadError) System.exit(65);
		String code = new String(bytes, StandardCharsets.UTF_8) + "\0";

		Tokenize tokenize = new Tokenize(code);
		List<Token> tokens = tokenize.tokenize();

		Parser parser = new Parser(tokens);
		Program prog = parser.parse();

		for (Function fn = prog.fns; fn != null; fn = fn.next) {

			int maxOffset = 0;
			for (VarList vl = fn.locals; vl != null; vl = vl.next) {
				if (vl.va.offset > maxOffset) {
					maxOffset = vl.va.offset;
				}
			}
			fn.stack_size = align_to(maxOffset, 16);
		}
		
		Type typeAnalyzer = new Type();
		typeAnalyzer.add_type(prog);

		Codegen codegen = new Codegen();
		codegen.codegen(prog);
	}

	private static int align_to(int n, int align) {
		return (n + align - 1)& ~(align - 1);
	}
}
