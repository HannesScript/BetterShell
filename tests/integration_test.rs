// Integration tests for BetterShell - testing commands module directly
use std::env;

#[test]
fn test_echo_command_with_single_arg() {
    use bettershell::commands::handle_echo;
    
    let args = vec!["hello"];
    handle_echo(args);
    // Prints: hello
}

#[test]
fn test_echo_command_with_multiple_args() {
    use bettershell::commands::handle_echo;
    
    let args = vec!["hello", "beautiful", "world"];
    handle_echo(args);
    // Prints: hello beautiful world
}

#[test]
fn test_echo_command_empty_args() {
    use bettershell::commands::handle_echo;
    
    let args: Vec<&str> = vec![];
    handle_echo(args);
    // Prints: (empty line)
}

#[test]
fn test_pwd_command() {
    use bettershell::commands::handle_print_working_directory;
    
    handle_print_working_directory();
    // Should print current working directory
}

#[test]
fn test_type_command_all_builtins() {
    use bettershell::commands::handle_type;
    
    let builtins = vec!["echo", "exit", "type", "pwd", "cd"];
    
    for builtin in builtins {
        let args = vec![builtin];
        handle_type(args);
        // Each should print: "<builtin> is a shell builtin"
    }
}

#[test]
fn test_type_command_external() {
    use bettershell::commands::handle_type;
    
    // Test with 'cat' which should exist on most Unix systems
    let args = vec!["cat"];
    handle_type(args);
    // Should print path to cat or "not found"
}

#[test]
fn test_type_command_nonexistent() {
    use bettershell::commands::handle_type;
    
    let args = vec!["this_command_does_not_exist_xyz"];
    handle_type(args);
    // Should print: "this_command_does_not_exist_xyz: not found"
}

#[test]
fn test_cd_to_tmp_directory() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    // Change to /tmp
    let args = vec!["/tmp"];
    handle_cd(args);
    
    // Verify we're in /tmp
    let current = env::current_dir().unwrap();
    assert_eq!(current.to_str().unwrap(), "/tmp");
    
    // Restore original directory
    env::set_current_dir(&original_dir).ok();
}

#[test]
fn test_cd_to_home_with_tilde() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    // Change to home directory
    let args = vec!["~"];
    handle_cd(args);
    
    // Verify we're in home directory
    if let Some(home) = env::home_dir() {
        let current = env::current_dir().unwrap();
        assert_eq!(current, home);
    }
    
    // Restore original directory
    env::set_current_dir(&original_dir).ok();
}

#[test]
fn test_cd_to_invalid_directory() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    // Try to change to non-existent directory
    let args = vec!["/this_directory_absolutely_does_not_exist_xyz"];
    handle_cd(args);
    
    // Should remain in original directory
    let current = env::current_dir().unwrap();
    assert_eq!(current, original_dir);
}

#[test]
fn test_cd_relative_path() {
    use bettershell::commands::handle_cd;
    
    let original_dir = env::current_dir().unwrap();
    
    // Change to parent directory
    let args = vec![".."];
    handle_cd(args);
    
    // Verify we moved up one directory
    let parent = env::current_dir().unwrap();
    assert_ne!(parent, original_dir);
    assert_eq!(original_dir.parent().unwrap(), parent.as_path());
    
    // Restore original directory
    env::set_current_dir(&original_dir).ok();
}

#[test]
fn test_command_not_found_message() {
    use bettershell::commands::command_not_found;
    
    command_not_found("fake_command_xyz");
    // Should print: "fake_command_xyz: command not found"
}

#[test]
#[ignore] // Ignore by default as it spawns external processes
fn test_execute_external_ls() {
    use bettershell::commands::execute_external_program;
    
    let command = "ls";
    let args = vec!["/tmp"];
    execute_external_program(command, args);
    // Should execute ls /tmp
}

#[test]
fn test_execute_external_not_found() {
    use bettershell::commands::execute_external_program;
    
    let command = "this_program_does_not_exist_xyz";
    let args: Vec<&str> = vec![];
    execute_external_program(command, args);
    // Should print: "this_program_does_not_exist_xyz: command not found"
}

#[test]
#[ignore] // Ignore by default as it spawns external processes
fn test_execute_external_echo_via_path() {
    use bettershell::commands::execute_external_program;
    
    let command = "echo";
    let args = vec!["test", "message"];
    execute_external_program(command, args);
    // Should execute /bin/echo test message
}

#[cfg(test)]
mod integration_scenarios {
    use std::env;
    use bettershell::commands::*;

    #[test]
    fn test_multiple_cd_commands() {
        let original_dir = env::current_dir().unwrap();
        
        // Navigate to /tmp
        handle_cd(vec!["/tmp"]);
        assert_eq!(env::current_dir().unwrap().to_str().unwrap(), "/tmp");
        
        // Navigate back up
        handle_cd(vec![".."]);
        assert_eq!(env::current_dir().unwrap().to_str().unwrap(), "/");
        
        // Restore
        env::set_current_dir(&original_dir).ok();
    }

    #[test]
    fn test_pwd_after_cd() {
        let original_dir = env::current_dir().unwrap();
        
        // Change directory
        handle_cd(vec!["/tmp"]);
        
        // Check PWD reflects the change
        let current = env::current_dir().unwrap();
        assert_eq!(current.to_str().unwrap(), "/tmp");
        
        // Restore
        env::set_current_dir(&original_dir).ok();
    }

    #[test]
    #[ignore] // Ignore by default as it spawns external processes that wait for input
    fn test_type_then_execute_command() {
        // First check if command exists
        handle_type(vec!["cat"]);
        
        // Then try to execute it (if it exists)
        execute_external_program("cat", vec![]);
        // This should either execute cat (waiting for input) or print command not found
    }

    #[test]
    fn test_echo_various_inputs() {
        // Test empty
        handle_echo(vec![]);
        
        // Test single word
        handle_echo(vec!["test"]);
        
        // Test multiple words
        handle_echo(vec!["hello", "world"]);
        
        // Test special characters
        handle_echo(vec!["hello!", "@#$%", "world?"]);
    }

    #[test]
    fn test_directory_navigation_sequence() {
        let original_dir = env::current_dir().unwrap();
        
        // Go to root
        handle_cd(vec!["/"]);
        assert_eq!(env::current_dir().unwrap().to_str().unwrap(), "/");
        
        // Go to tmp
        handle_cd(vec!["tmp"]);
        assert_eq!(env::current_dir().unwrap().to_str().unwrap(), "/tmp");
        
        // Go to home
        if let Some(_home) = env::home_dir() {
            handle_cd(vec!["~"]);
            // Should be in home directory now
        }
        
        // Restore
        env::set_current_dir(&original_dir).ok();
    }

    #[test]
    fn test_check_all_builtins_with_type() {
        let builtins = vec!["echo", "exit", "type", "pwd", "cd"];
        
        for builtin in builtins {
            handle_type(vec![builtin]);
            // Each should print that it's a builtin
        }
    }

    #[test]
    fn test_error_handling_invalid_cd() {
        let original_dir = env::current_dir().unwrap();
        
        // Try multiple invalid directories
        handle_cd(vec!["/invalid1"]);
        assert_eq!(env::current_dir().unwrap(), original_dir);
        
        handle_cd(vec!["/invalid2"]);
        assert_eq!(env::current_dir().unwrap(), original_dir);
        
        handle_cd(vec!["~/nonexistent"]);
        assert_eq!(env::current_dir().unwrap(), original_dir);
    }
}

#[cfg(test)]
mod path_tests {
    use std::env;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use std::fs;

    #[test]
    fn test_path_contains_directories() {
        let path = env::var("PATH").unwrap_or_default();
        let dirs: Vec<&str> = path.split(":").collect();
        
        assert!(dirs.len() > 0, "PATH should contain at least one directory");
        
        // Check that some directories in PATH actually exist
        let existing_dirs: Vec<&str> = dirs.into_iter()
            .filter(|d| Path::new(d).exists())
            .collect();
        
        assert!(existing_dirs.len() > 0, "At least one PATH directory should exist");
    }

    #[test]
    fn test_find_executable_in_path() {
        let path = env::var("PATH").unwrap_or_default();
        let path_dirs: Vec<&str> = path.split(":").collect();
        
        // Try to find 'ls' command
        let mut found_ls = false;
        
        for dir in path_dirs {
            let ls_path = format!("{}/ls", dir);
            let path_obj = Path::new(&ls_path);
            
            if path_obj.exists() {
                if let Ok(metadata) = fs::metadata(&ls_path) {
                    let mode = metadata.permissions().mode();
                    if mode & 0o111 != 0 {
                        found_ls = true;
                        break;
                    }
                }
            }
        }
        
        // ls should be found on most Unix systems
        assert!(found_ls, "Should find 'ls' command in PATH");
    }

    #[test]
    fn test_home_directory_expansion() {
        if let Some(home) = env::home_dir() {
            let test_path = "~/test";
            let home_str = home.to_string_lossy();
            let expanded = test_path.replace("~", &home_str);
            
            assert!(expanded.starts_with(home_str.as_ref()));
            assert!(!expanded.contains("~"));
            assert!(expanded.ends_with("/test"));
        }
    }
}
