use std::process::Command;
use std::str;

fn main() {
    println!("Hello, world!");
}

fn run() {
    let output = Command::new("/usr/bin/git")
        .args(&["status", "--porcelain=2", "-b"])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let output = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8: {}", e),
        };
        let lines: Vec<&str> = output.lines().collect();
        println!("{:?}", lines);
    }
}
