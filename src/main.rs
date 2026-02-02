#[allow(unused_imports)]
use std::io::{self, Write};
mod commands; // this line includes commands.rs

fn main() {
    // Just loops the REPL (read-evaluate-print loop)
    loop {
        read_eval_print_cycle();
    }
}

fn read_eval_print_cycle() {
    // Print the iconic shell prompt
    print!("TODO (USER) : {} $ ", std::env::current_dir().unwrap_or_default().to_string_lossy());
    io::stdout().flush().unwrap();

    let mut input = String::new(); // Initialize the input variable
    io::stdin().read_line(&mut input).unwrap(); // Read input

    // Trim the input to remove newline characters
    let input: &str = input.trim();

    // Split by white spaces
    let mut input_split: std::str::SplitWhitespace<'_> = input.split_whitespace();
    // Get first element (command)
    let command: &str = input_split.next().unwrap_or("");
    // extract remaining parts to args
    let args: Vec<&str> = input_split.collect();

    // match the commands
    command_matcher(command, args);
}

/**
    This matches the command and calls according function (from commands.rs)
*/
fn command_matcher(command: &str, args: Vec<&str>) {
    match command {
        "type" => commands::handle_type(args),
        "echo" => commands::handle_echo(args),
        "cd" => commands::handle_cd(args),
        "pwd" => commands::handle_print_working_directory(),
        "exit" => commands::handle_exit(),
        _ => commands::execute_external_program(command, args),
    }
}
