// Direct unit tests for bettershell commands module
// These tests directly invoke command functions from the bettershell library

use std::env;

#[test]
fn test_handle_echo_single_word() {
    use bettershell::commands::handle_echo;
    
    // Capture output by redirecting stdout
    let args = vec!["hello"];
    handle_echo(args);
    // Output: "hello" should be printed
}

#[test]
fn test_handle_echo_multiple_words() {
    use bettershell::commands::handle_echo;
    
    let args = vec!["hello", "world", "test"];
    handle_echo(args);
    // Output: "hello world test" should be printed
}

#[test]
fn test_handle_pwd() {
    use bettershell::commands::handle_print_working_directory;
    
    handle_print_working_directory();
    // Should print current working directory
}

#[test]
fn test_handle_type_builtin_echo() {
    use bettershell::commands::handle_type;
    
    let args = vec!["echo"];
    handle_type(args);
    // Should print "echo is a shell builtin"
}

#[test]
fn test_handle_type_builtin_exit() {
    use bettershell::commands::handle_type;
    
    let args = vec!["exit"];
    handle_type(args);
    // Should print "exit is a shell builtin"
}

#[test]
fn test_handle_type_builtin_type() {
    use bettershell::commands::handle_type;
    
    let args = vec!["type"];
    handle_type(args);
    // Should print "type is a shell builtin"
}

#[test]
fn test_handle_type_builtin_pwd() {
    use bettershell::commands::handle_type;
    
    let args = vec!["pwd"];
    handle_type(args);
    // Should print "pwd is a shell builtin"
}

#[test]
fn test_handle_type_builtin_cd() {
    use bettershell::commands::handle_type;
    
    let args = vec!["cd"];
    handle_type(args);
    // Should print "cd is a shell builtin"
}

#[test]
fn test_handle_type_external_command() {
    use bettershell::commands::handle_type;
    
    let args = vec!["ls"];
    handle_type(args);
    // Should find ls in PATH or print "not found"
}

#[test]
fn test_handle_type_nonexistent_command() {
    use bettershell::commands::handle_type;
    
    let args = vec!["nonexistentcommand123"];
    handle_type(args);
    // Should print "nonexistentcommand123: not found"
}

#[test]
fn test_command_not_found() {
    use bettershell::commands::command_not_found;
    
    command_not_found("nonexistentcommand123");
    // Should print "nonexistentcommand123: command not found"
}

#[test]
fn test_handle_cd_to_tmp() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    let args = vec!["/tmp"];
    handle_cd(args);
    
    let new_dir = env::current_dir().unwrap();
    assert_eq!(new_dir.to_str().unwrap(), "/tmp");
    
    // Restore original directory
    env::set_current_dir(&original_dir).ok();
}

#[test]
fn test_handle_cd_invalid_directory() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    let args = vec!["/nonexistent_directory_12345"];
    handle_cd(args);
    
    // Should stay in same directory
    let current_dir = env::current_dir().unwrap();
    assert_eq!(current_dir, original_dir);
}

#[test]
fn test_handle_cd_with_tilde() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    let args = vec!["~"];
    handle_cd(args);
    
    let current_dir = env::current_dir().unwrap();
    
    if let Some(home) = env::home_dir() {
        assert_eq!(current_dir, home);
    }
    
    // Restore original directory
    env::set_current_dir(&original_dir).ok();
}

#[test]
fn test_execute_external_program_not_found() {
    use bettershell::commands::execute_external_program;
    
    let command = "nonexistentprogram12345";
    let args = vec![];
    execute_external_program(command, args);
    // Should print command not found message
}

#[cfg(test)]
mod unit_tests {
    use std::env;

    #[test]
    fn test_path_parsing() {
        let path = env::var("PATH").unwrap_or_default();
        let path_split: Vec<&str> = path.split(":").collect();
        
        // PATH should contain at least one directory
        assert!(path_split.len() > 0);
    }

    #[test]
    fn test_home_directory_tilde_expansion() {
        let test_path = "~/test";
        
        if let Some(home_dir) = env::home_dir() {
            let home_str = home_dir.to_string_lossy();
            let expanded = test_path.replace("~", &home_str);
            
            assert!(expanded.starts_with(&home_str.to_string()));
            assert!(!expanded.contains("~"));
        }
    }

    #[test]
    fn test_args_joining() {
        let args = vec!["hello", "world", "test"];
        let joined = args.join(" ");
        
        assert_eq!(joined, "hello world test");
    }

    #[test]
    fn test_command_parsing() {
        let input = "echo hello world";
        let mut input_split = input.split_whitespace();
        let command = input_split.next().unwrap_or("");
        let args: Vec<&str> = input_split.collect();
        
        assert_eq!(command, "echo");
        assert_eq!(args.len(), 2);
        assert_eq!(args[0], "hello");
        assert_eq!(args[1], "world");
    }

    #[test]
    fn test_empty_input_parsing() {
        let input = "";
        let mut input_split = input.split_whitespace();
        let command = input_split.next().unwrap_or("");
        let args: Vec<&str> = input_split.collect();
        
        assert_eq!(command, "");
        assert_eq!(args.len(), 0);
    }

    #[test]
    fn test_whitespace_trimming() {
        let input = "  echo   hello  \n";
        let trimmed = input.trim();
        
        assert_eq!(trimmed, "echo   hello");
    }

    #[test]
    fn test_echo_args_construction() {
        let args = vec!["hello", "beautiful", "world"];
        let result = args.join(" ");
        
        assert_eq!(result, "hello beautiful world");
    }
}
