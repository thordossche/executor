use std::{fs, io, io::{Read, BufReader, BufRead}, str, process::{Command, Stdio}};
use is_executable::IsExecutable;
use inquire::{error::InquireError, Select};

fn read_and_print_file() {
    let greeting_file_result = fs::File::open("Cargo.toml");

    let mut greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut buffer = Vec::new();
    greeting_file.read_to_end(&mut buffer).expect("Failed to read file");
    
    let s = match str::from_utf8(&buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    println!("result: {}", s);
}

fn main() {
    let entries = fs::read_dir(".").expect("Failed to read dir");

    let mut executables = Vec::new();
    for entry in entries {
        let dir_entry = entry.expect("Failed to read directory entry");
        let path = dir_entry.path();
        // println!("{:?}: {:?}", path, path.is_executable());

        if path.is_executable() {
            if let Some(path_string) = path.to_str(){
                executables.push(path_string.to_owned());
            }
        }

        // if let Ok(entry) = entry {
        //     // Here, `entry` is a `DirEntry`.
        //     if let Ok(metadata) = entry.metadata() {
        //         // Now let's show our entry's permissions!
        //         println!("{:?}: {:?}", entry.path(), metadata.permissions());
        //     } else {
        //         println!("Couldn't get metadata for {:?}", entry.path());
        //     }
        // }
    }

    // for (i, executable) in executables.iter().enumerate() {
    //     println!("{}) {:?}", i, executable);
    // }

    let ans: Result<String, InquireError> = Select::new("Which executable would you like to execute?", executables).prompt();

    let executable;
    match ans {
        Ok(choice) => executable = choice,
        Err(_) => panic!("An error occured..."),
    }

    let mut child = Command::new("bash")
        .arg("-e")
        .arg(executable)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run the executable");

    if let Some(stdout) = child.stdout.take() {
        let mut bufread = BufReader::new(stdout);
        let mut buf = String::new();

        while let Ok(n) = bufread.read_line(&mut buf) {
            if n > 0 {
                println!("{}", buf.trim());
                buf.clear();
            } else {
                match child.try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => continue,
                    Err(err) => {
                        println!("Child process errord with {}", err);
                        break;
                    }
                }
            }
        }

    } else {
        panic!("Could not take the stdout");
    }
}
