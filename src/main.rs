// gsync app

// required crates
use std::process::Command; 
use std::io::Write;
use std::io;
use colored::*;

fn main() {
    // user prompt
    print!("commit message >>> ");
    io::stdout().flush().unwrap(); 

    // user input for commit message
    let mut message = String::new();
    io::stdin().read_line(&mut message).expect("Failed to read message"); 
    
    // git initialize, add files and commit
    let _ = Command::new("git").arg("init").output();
    let _ = Command::new("git").args(["add", "."]).output();

    // capture the status of the commit action to determine if 
    // a change was commited or not
    let status = Command::new("git").args(["commit", "-m", &message]).status().expect("failed to commit");

    match status.code() {
        Some(code) => match code {
            // if the status code is 1, then there is no changes to commit
            1 => {
                println!("{} {} {}", "push failed,".blue(), "no changes".red(), "to commit".blue());
            },

            // else commit the changes with the given message
            _ => {
                println!("{}, ::commit: {}", "Committed successfully".green(), message.blue());
            }
        },
        None => println!("process terminated")
    }
}
