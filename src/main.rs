extern crate git_status_line;

use std::process::Command;
use std::str;

use git_status_line::Status;

fn main() {
    let output = Command::new("git")
        .args(&["status", "--porcelain=2", "--branch"])
        .output()
        .expect("failed to execute process");
    if output.status.success() {
        let output = match str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8: {}", e),
        };
        let status = match Status::new(output) {
            Ok(v) => v,
            Err(e) => panic!("{}", e),
        };
        print!("{} ", status);
    }
}
