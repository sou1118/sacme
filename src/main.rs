use std::io;
use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc:  usize = argv.len();
    let stderr = io::stderr();
    if argc != 2 {
        eprintln!{"{:?} {}", stderr, "引数の個数が正しくありません"}
        return;
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", argv[1].parse::<i32>().unwrap());
    println!("    ret");
}
