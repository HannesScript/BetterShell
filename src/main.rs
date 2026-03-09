#[allow(unused_imports)]
use std::io::{self, Write};
use bettershell::commands; // Use the library module

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
    let mut input_split: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    // Quotation logic
    let quotes = ["\"", "'"];
    let mut start_item_index: Option<usize> = None;
    let mut end_item_index: Option<usize> = None;
    let mut QUOTE_TYPE: Option<&str> = None;

    // First pass: find start
    for (index, element) in input.split_whitespace().enumerate() {
        if let Some(quote) = quotes.iter().find(|&&q| element.starts_with(q)) {
            start_item_index = Some(index);
            QUOTE_TYPE = Some(quote);
            break;
        }
    }

    // Only continue if we have found a quote
    if "" != QUOTE_TYPE.unwrap_or_default() {
        // Second pass: find end (create new iterator)
        for (index, element) in input.split_whitespace().enumerate() {
            if let Some(quote) = quotes.iter().find(|&&q| element.ends_with(q)) {
                end_item_index = Some(index);
                if QUOTE_TYPE == Some(quote) {
                    break;
                }
            }
        }

        // Remove the quotes and combine the elements to one
        if let Some(index) = start_item_index {
            if let Some(element) = input_split.get_mut(index) {
                if let Some(first_char) = element.chars().next() {
                    *element = (&element[first_char.len_utf8()..]).to_string();
                }
            }
        }
        
        // and the last element
        if let Some(index) = end_item_index {
            if let Some(element) = input_split.get_mut(index) {
                element.pop(); // removes the last char
            }
        }
    }

    // Get first element (command)
    let command: &str = input_split.get(0).map(|s| s.as_str()).unwrap_or("");
    // extract remaining parts to args
    let args: Vec<&str> = input_split.iter().skip(1).map(|s| s.as_str()).collect();

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
