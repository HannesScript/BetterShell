use std::fs;
use std::env;
use std::str;
use std::process;
use std::os::unix::process::CommandExt;
use std::path::*;

pub fn command_not_found(command: &str) {
    // Called when the command doesn't exist
    println!("{}: command not found", command);
    return;
}


pub fn handle_type(args: Vec<&str>) {
    let cmd: &str = args[0];   

    match cmd {
        // "" => println!(""),
        "exit" => println!("exit is a shell builtin"),
        "echo" => println!("echo is a shell builtin"),
        "type" => println!("type is a shell builtin"),
        "pwd" => println!("pwd is a shell builtin"),
        "cd" => println!("cd is a shell builtin"),
        _ => type_non_builtin(cmd),
    }
}

fn type_non_builtin(cmd: &str) {
    let path: String = env::var("PATH").unwrap_or_default();
    let path_split: str::Split<'_, &str> = path.split(":");

    let elem_found_at: String = String::new();

    for folder in path_split {                                      
        let full_path = format!("{}/{}", folder, cmd);              
        let path_obj = std::path::Path::new(&full_path);            
                                                                    
        if path_obj.exists() {                                      
            use std::os::unix::fs::PermissionsExt;                  
            let metadata = fs::metadata(&full_path);                
            if let Ok(meta) = metadata {                            
                let permissions = meta.permissions();               
                let mode = permissions.mode();                      
                if mode & 0o111 != 0 {                              
                    println!("{} is {}", cmd, full_path);           
                    return;                                         
                }                                                  
            }                                                       
        }                                                           
    }

    if elem_found_at != "" {
        println!("{} is {}", cmd, elem_found_at);
        return;
    }

    println!("{}: not found", cmd)
}


pub fn handle_exit() {
    process::exit(0); // exit
    // TODO: Implement better exit (stop running commands etc.)
}

pub fn handle_echo(args: Vec<&str>) {
    let text_to_print: String = args.join(" ");
    println!("{}", text_to_print);
    return;
}

pub fn execute_external_program(command: &str, args: Vec<&str>) {
    let path: String = env::var("PATH").unwrap_or_default();
    let path_split: str::Split<'_, &str> = path.split(":");

    let mut elem_found_at: String = String::new();

    for folder in path_split {                                      
        let full_path = format!("{}/{}", folder, command);              
        let path_obj = std::path::Path::new(&full_path);            
                                                                    
        if path_obj.exists() {                                      
            use std::os::unix::fs::PermissionsExt;                  
            let metadata = fs::metadata(&full_path);                
            if let Ok(meta) = metadata {                            
                let permissions = meta.permissions();               
                let mode = permissions.mode();                      
                if mode & 0o111 != 0 {                              
                    elem_found_at = full_path;        
                }                                                  
            }                                                       
        }                                                           
    }

    if elem_found_at != "" {
        // Execute the program
        let output = process::Command::new(&elem_found_at)
            .arg0(command)
            .args(&args)
            .current_dir(env::current_dir().unwrap_or_default())
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        } else {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }

        return;
    }

    // Fallback
    command_not_found(command);
}

pub fn handle_print_working_directory() {
    let binding = env::current_dir().unwrap_or_default();
    let working_dir = binding.display();

    println!("{}", working_dir);
}

pub fn handle_cd(args: Vec<&str>) {
    let mut path_str: String = args[0].to_string();

    // Handle home directory paths
    if path_str.starts_with("~") {
        let home_dir = env::home_dir().unwrap_or_default();
        let home_dir_str = home_dir.to_string_lossy();
        path_str = path_str.replace("~", &home_dir_str);
    }

    let path: &Path = Path::new(&path_str);
    if !env::set_current_dir(&path).is_ok() {
        println!("cd: {}: No such file or directory", path_str);
    }
}
