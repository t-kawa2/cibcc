import java.io.InputStreamReader;

public class Cib {

	public static void main(String[] args) {
		int val = 0;

		if (args.length != 1) {
			System.out.println("引数の個数が正しくありません");
			return;
		}

		System.out.println(".intel_syntax noprefix");
		System.out.println(".global main");
		System.out.println("main:");

		val = Integer.parseInt(args[0]);
		System.out.println("  mov rax, " + val);

		System.out.println("  ret");
	}
}

