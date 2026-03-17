use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		eprintln!("引数の個数が正しくありません");
		return;
	}

	println!(".intel_syntax noprefix");
	println!(".global main");
	println!("main:");
	println!("  mov rax, {}", args[1].parse::<usize>().unwrap());
	println!("  ret");
}
