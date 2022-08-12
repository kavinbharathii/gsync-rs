// gsync app
use std::process::Command; 
use std::io::Write;
use std::io;
use colored::*;

fn main() {
    print!("commit message >>> ");
    io::stdout().flush().unwrap(); 

    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read message"); 
    let _ = Command::new("git").arg("init").output();
    let _ = Command::new("git").args(["add", "."]).output();
    let status = Command::new("git").args(["commit", "-m", &message]).status().expect("failed to commit");

    match status.code() {
        Some(code) => match code {
            1 => {
                println!("no changes to commit, {}", "process terminated".red());
            },
            _ => {
                println!("{}, ::commit: {}", "Committed successfully".green(), message.blue());
            }
        },
        None => println!("process terminated")
    }
}
