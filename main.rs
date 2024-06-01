use std::io::{self, Write};
use std::env::{self};
use std::fs;
use std::path::Path;
use regex::Regex;
use users::{get_user_by_uid, get_current_uid};

fn main() {
    
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
         
        let re = Regex::new(r#"(".*?"|\S+)"#).unwrap();
        let parts: Vec<&str> = re.find_iter(&input)
                                 .map(|m| m.as_str())
                                 .collect();
    
        
        let command = parts[0];

        let args=&parts[1..];

        match command {
            "cat" => cat(args),
            "ls" => ls(args),
            "cd" => cd(args), 
            "rm" => rm(args),
            "whoami" => whoami(),
            "pwd" => pwd(),
            "echo" => echo(args),
            "exit" => break,
            _ => println!("Not a recognized command!")
        }
    }
}


fn off_quot(text_with_quotes: &str) -> Option<String> {
    let re = Regex::new(r#""([^"]*)""#).unwrap();
    
    if let Some(captured) = re.captures(text_with_quotes) {
        if let Some(inner) = captured.get(1) {
            return Some(inner.as_str().to_string());
        }
    }
    None
}



fn cat(array: &[&str]) {
    let file_path = array[0];
    match fs::read_to_string(file_path) {
        Ok(contents) => println!("{contents}"),
        Err(e) => println!("Error: {e}"),
    }
    
}
fn ls(array: &[&str]) {

    
    if array.len() != 0 {
        let file_path = array[0];
        let paths = fs::read_dir(file_path).unwrap();
    
        for path in paths {
            print!("{} ", path.unwrap().path().display());
            println!("");
        }

    } else {
        if let Ok(path) = env::current_dir() {
            let paths = fs::read_dir(path).unwrap();
    
            for path in paths {
                print!("{} ", path.unwrap().path().display());
                println!("");
        }
        } else {
            eprintln!("Can't get current directory");
        }
        
    }
    
}
fn cd(array: &[&str]) {
    let path = array[0];
    let root = Path::new(path);
    assert!(env::set_current_dir(&root).is_ok());
}
fn pwd() {
    if let Ok(path) = env::current_dir() {
        println!("{:?}", path);
    } else {
        eprintln!("Can't get current directory");
    }
}
fn echo(array: &[&str]) {
    let content = array[0];
    println!("{content}");
}
fn whoami() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    println!("{:#?}", user.name());
}

fn rm(array: &[&str]) {
    let path = &array[0];
    fs::remove_file(path);
}




