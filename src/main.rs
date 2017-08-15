use std::process;
use std::process::Command;

extern crate git_status_line;
use git_status_line::Status;

fn main() {
    let (status_txt, success) = capture_stdout("git", &["status", "--porcelain=2", "--branch"]);
    if !success {
        process::exit(1);
    }
    let status = Status::new(&status_txt).unwrap();
    println!("{}", status);
}

fn capture_stdout(prog: &str, args: &[&str]) -> (String, bool) {
    let result = Command::new(prog)
        .args(args)
        .output()
        .unwrap();
    (String::from_utf8(result.stdout).unwrap(), result.status.success())
}
