use std::{fs, io, io::{BufReader, BufRead}, process::{Command, Stdio, Child}};
use is_executable::IsExecutable;
use inquire::{MultiSelect, Select};

fn run_executable(executable: &String) -> Result<Child, io::Error> {
    let child = Command::new("bash")
        .arg("-e")
        .arg(executable)
        .stdout(Stdio::piped())
        .spawn();
    child
}

fn show_stdout_of_child(mut child: Child) {
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
                        println!("Something went wrong: {}", err);
                        break;
                    }
                }
            }
        }
    } else {
        println!("Something went wrong when getting stdout");
    }
}

fn main() {
    let entries = fs::read_dir(".").expect("Failed to read dir");

    let mut executables = Vec::new();
    for entry in entries {
        let dir_entry = entry.expect("Failed to read directory entry");
        let path = dir_entry.path();

        if path.is_executable() {
            if let Some(path_string) = path.to_str(){
                executables.push(path_string.to_owned());
            }
        }

    }

    let mut selector = MultiSelect::new("Which executable(s) would you like to launch?", executables);
    selector.vim_mode = true;

    let executables = match selector.prompt() {
        Ok(selection) => selection,
        Err(_) => panic!("Something went wrong in the selection"),
    };

    let mut children = Vec::new();
    let mut running_executables = Vec::new();
    for executable in executables {
        match run_executable(&executable){
            Ok(child) => {
                children.push(child);
                running_executables.push(executable.clone());
            },
            Err(_) => continue,
        }
    }

    let mut selector = Select::new("From which process would you like to see the stdout", running_executables.clone());
    selector.vim_mode = true;

    let selection = match selector.prompt() {
        Ok(selection) => selection,
        Err(_) => panic!("Something went wrong in the selection"),
    };

    if let Some(index) = running_executables.iter().position(|exe| exe == &selection) {
        let child = children.swap_remove(index);
        show_stdout_of_child(child);
    }
}
