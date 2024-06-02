use colored::Colorize;
use gethostname::gethostname;
use regex::Regex;
use std::env::{self};
use std::fs;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use users::{get_current_uid, get_user_by_uid};

fn main() {
    println!(
        "Welcome to {}, the rust shell :)
use \"help\" to get info about the available commands",
        "rush".blue()
    );
    loop {
        // print user, hostname, prompt
        whoami();
        print!("@");
        hostname();
        pwd();
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let re = Regex::new(r#"(".*?"|\S+)"#).unwrap();
        let parts: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();

        let command = parts[0];

        let args = &parts[1..];

        match command {
            "cat" => cat(args),
            "ls" => ls(args),
            "cd" => cd(args),
            "rm" => rm(args),
            "touch" => touch(args),
            "echo" => echo(args),
            "whoami" => whoami(),
            "pwd" => pwd(),
            "hostname" => hostname(),
            "help" => help(),
            "exit" => break,
            _ => println!("Not a recognized command!"),
        }
        match command {
            "cd" => {}
            "help" => {}
            "cat" => {}
            _ => println!(),
        }
    }
}
fn cat(array: &[&str]) {
    let file_path = array[0];
    match fs::read_to_string(file_path) {
        Ok(contents) => print!("{contents}"),
        Err(e) => print!("Error: {e}"),
    }
}
fn ls(array: &[&str]) {
    if array.len() != 0 {
        let file_path = array[0];
        let paths = fs::read_dir(file_path).unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let gnu_path = path.to_str().unwrap(); //gnu path would look like /home/test_user/project/test.txt
            let filename = derive_filename(gnu_path, file_path).unwrap();


            match check_path_type(gnu_path) {
                PathType::File => print!("{}  ", filename.truecolor(61, 255, 71)),
                PathType::Directory => print!("{}  ", filename.truecolor(61, 148, 255)),
                PathType::NotFound => {
                    println!("Identified {filename} as exisitng but lost it... uh...  ")
                }
                PathType::Other => print!("{}  ", filename.truecolor(255, 168, 61)),
            }
        }
    } else {
        if let Ok(path) = env::current_dir() {
            let paths = fs::read_dir(path).unwrap();
            let file_path_raw = env::current_dir().unwrap();
            let file_path = file_path_raw.to_str().unwrap();
            for path in paths {
                let path = path.unwrap().path();
                let gnu_path = path.to_str().unwrap(); 
                let filename = derive_filename(gnu_path, file_path).unwrap();


                match check_path_type(gnu_path) {
                    PathType::File => print!("{}  ", filename.truecolor(61, 255, 71)), //green btw
                    PathType::Directory => print!("{}  ", filename.truecolor(61, 148, 255)), //blue btw
                    PathType::NotFound => {
                        println!("Identified {filename} as exisitng but lost it... uh...  ")
                    }
                    PathType::Other => print!("{}  ", filename.truecolor(255, 168, 61)),
                }
            }
        } else {
            eprint!("Can't get current directory");
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
        let path = path.to_str().unwrap();
        print!("{}", path.truecolor(61, 148, 255));
    } else {
        eprint!("Can't get current directory");
    }
}
fn echo(array: &[&str]) {
    let content = array[0];
    print!("{content}");
}
fn whoami() {
    let user = get_user_by_uid(get_current_uid()).unwrap();
    print!("{}", user.name().to_str().unwrap().truecolor(255, 61, 245));
}
fn rm(array: &[&str]) {
    let path = &array[0];
    fs::remove_file(path).unwrap();
    match check_path_type(path) {
        PathType::File => print!("Unable to remove {path}"),
        PathType::Directory => print!("Unable to remove {path}"),
        PathType::NotFound => print!("Removed {path}"),
        PathType::Other => print!("Unable to remove {path}"),
    }
}
fn touch(array: &[&str]) {
    let path = &array[0];
    File::create(path).unwrap();
    match check_path_type(path) {
        PathType::File => print!("Created File {path}"),
        PathType::Directory => print!("How in the world? {path} is a directory. Huh."),
        PathType::NotFound => print!("Unable to create {path}"),
        PathType::Other => {
            print!("How in the world? {path} was created but was unrecognized metadata. Huh. ")
        }
    }
}enum PathType {
    File,
    Directory,
    NotFound,
    Other,
}

fn check_path_type(path: &str) -> PathType {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_file() {
                PathType::File
            } else if metadata.is_dir() {
                PathType::Directory
            } else {
                PathType::Other
            }
        }
        Err(_) => PathType::NotFound,
    }
}
fn hostname() {
    print!(
        "{} ",
        gethostname().to_str().unwrap().truecolor(61, 245, 255)
    );
}
fn help() {
    println!(
        "cat - concatenates (prints) a file
ls - lists files in the specificed directory
cd - changes directory
rm - removes a specified file
touch - creates a specified file
echo - echoes given text 
whoami - lists current user
pwd - prints the current directory
hostname - prints the hostname
help - gives a blurb on all commands
exit - leave the shell"
    )
}

fn derive_filename<'a>(full_path: &'a str, base_path: &'a str) -> Option<&'a str> {
    if full_path.starts_with(base_path) {
        Some(&full_path[base_path.len()..].trim_start_matches('/'))
    } else {
        None
    }
}
