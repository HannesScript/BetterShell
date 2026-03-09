use bettershell::commands;
#[allow(unused_imports)]
use std::io::{self, Write}; // Use the library module

fn main() {
    // Just loops the REPL (read-evaluate-print loop)
    loop {
        read_eval_print_cycle();
    }
}

fn read_eval_print_cycle() {
    // Print the iconic shell prompt
    print!(
        "TODO (USER) : {} $ ",
        std::env::current_dir()
            .unwrap_or_default()
            .to_string_lossy()
    );
    io::stdout().flush().unwrap();

    let mut input = String::new(); // Initialize the input variable
    io::stdin().read_line(&mut input).unwrap(); // Read input

    // Trim the input to remove newline characters
    let input: &str = input.trim();

    // Split by white spaces while preserving quoted segments
    let input_split: Vec<String> = split_input_preserving_quotes(input);

    // Get first element (command)
    let command: &str = input_split.get(0).map(|s| s.as_str()).unwrap_or("");
    // extract remaining parts to args
    let args: Vec<&str> = input_split.iter().skip(1).map(|s| s.as_str()).collect();

    // match the commands
    command_matcher(command, args);
}

fn split_input_preserving_quotes(input: &str) -> Vec<String> {
    let mut parts: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quote: Option<char> = None;

    for ch in input.chars() {
        match ch {
            '"' | '\'' => {
                if in_quote == Some(ch) {
                    in_quote = None;
                } else if in_quote.is_none() {
                    in_quote = Some(ch);
                } else {
                    current.push(ch);
                }
            }
            c if c.is_whitespace() && in_quote.is_none() => {
                if !current.is_empty() {
                    parts.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
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
