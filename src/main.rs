use std::{
    any::Any,
    env,
    fmt::Display,
    fs::File,
    io::prelude::*,
    path::Path,
    process::{self, ExitCode},
    str::FromStr,
};

use serde::Serialize;
use toml::*;
use toml_edit::*;

enum Action {
    Add,
    Modify,
    Delete,
    Show,
    Search,
}

enum Error {
    NotEnoughArguments,
    UnspecifiedCommand,
    CouldNotOpenFile,
    Internal,
    UnknownReason,
}

struct CommandDetails {
    action: Action,
    args: Vec<String>,
}

fn print_error_and_exit(err: Error) {
    match err {
        Error::NotEnoughArguments => {
            eprintln!(
                "Incorrect amount of arguments! You should use [foget] [action] [action parameters]\n"
            );
        }
        Error::UnspecifiedCommand => {
            eprintln!("Could not understand action. Please specify one from the list [ show, add, modify, delete ].\n");
        }
        Error::CouldNotOpenFile => {
            eprintln!("Could not open database file located in \"~/foget/descriptions/unix.toml\". Exiting now.")
        }
        Error::Internal => {
            eprintln!("Unexpected internal error.");
        }
        Error::UnknownReason => {
            eprintln!("Unknown error. Exiting.");
        }
    }

    process::exit(1);
}

fn print_help() {
    println!("Usage: $[foget] [action] [action parameters]");
    println!("\tActions:");

    println!("\t[add or a] [name of the command] [tags]");
    println!("\t\tadd a new command and tags to the datatabase");

    println!("\t[modify or mod or m] [name of the command] [new tags]");
    println!("\t\tmodify a command by adding new tags to the datatabase");

    // TODO: Other help messages

    process::exit(0);
}

fn parse_arguments() -> Result<CommandDetails, ()> {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "add" | "a" => {
            if args.len() < 4 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            return Result::Ok(CommandDetails {
                action: Action::Add,
                args: vec![args[2].clone(), args[3].clone()],
            });
        }
        "modify" | "m" | "mod" => {
            if args.len() < 4 {
                print_error_and_exit(Error::NotEnoughArguments);
            }
        }
        "delete" | "del" | "d" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }
        }
        "show" | "sho" | "sh" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            return Result::Ok({
                CommandDetails {
                    action: Action::Show,
                    args: vec![args[2].clone()],
                }
            });
        }
        "search" | "se" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            return Result::Ok({
                CommandDetails {
                    action: Action::Search,
                    args: vec![args[2].clone()],
                }
            });
        }
        "help" | "h" => {
            print_help();
        }
        _ => {
            print_error_and_exit(Error::UnspecifiedCommand);
        }
    }

    print_error_and_exit(Error::UnspecifiedCommand);
    Err(())
}

fn parse_toml() -> DocumentMut {
    let path = Path::new("./descriptions/unix.toml");

    let mut file: File = match File::open(path) {
        Err(why) => {
            print_error_and_exit(Error::CouldNotOpenFile);
            panic!("{:?}", why);
        }
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(why) => {
            panic!("{:?}", why);
        }
        Ok(_) => {}
    }

    let res = match content.parse::<DocumentMut>() {
        Err(why) => {
            panic!("{:?}", why);
        }
        Ok(res) => res,
    };

    res
}

fn show_command_tags(details: CommandDetails, toml: toml_edit::DocumentMut) {
    if toml.contains_key(&details.args[0]) {
        println!("Found descriptions for command `{}`:", details.args[0]);

        let mut i = 0;
        while i < toml[&details.args[0]]["tags"].as_array().unwrap().len() {
            println!("\t{}", toml[&details.args[0]]["tags"][i].as_str().unwrap());
            i += 1;
        }
    }
}

fn search_descriptions(details: CommandDetails, toml: toml_edit::DocumentMut) {
    let mut commands: Vec<String> = vec![];
    for (key, _val) in toml.as_table().into_iter() {
        if toml[key]["tags"].is_array() {
            if !toml[key]["tags"]
                .as_array()
                .unwrap_or(&toml_edit::Array::new())
                .is_empty()
            {
                for desc in toml[key]["tags"]
                    .as_array()
                    .unwrap_or(&toml_edit::Array::new())
                {
                    if desc.is_str() && desc.as_str().unwrap().contains(&details.args[0]) {
                        commands.push(String::from(key));
                        break;
                    }
                }
            }
        }
    }

    if commands.len() > 0 {
        println!("Commands with matching functionality:");
        let mut i = 0;
        while i < commands.len() {
            println!("{} -- {}", commands[i], toml[&commands[i]]["tags"]);
            i += 1;
        }
    } else {
        println!("No commands match the searched functionality.");
    }
}

fn add_description(details: CommandDetails, toml: &mut toml_edit::DocumentMut) {
    let path = Path::new("./descriptions/unix.toml");

    let mut file: File = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
    {
        Err(why) => {
            print_error_and_exit(Error::CouldNotOpenFile);
            panic!("{:?}", why);
        }
        Ok(file) => file,
    };

    let arr: &mut Array = match toml[&details.args[0]]["tags"].as_array_mut() {
        None => {
            print_error_and_exit(Error::Internal);
            &mut toml_edit::Array::new()
        }
        Some(arr) => arr,
    };

    if !arr.is_empty() {
        arr.push(details.args[1].clone());
    } else {
    }

    println!("{}", toml.to_string());

    match file.write_all(toml.to_string().as_bytes()) {
        Err(why) => {
            println!("{}", why);
        }
        _ => (),
    }

    match file.flush() {
        Err(why) => {
            println!("{}", why);
        }
        _ => (),
    }
}

fn main() -> ExitCode {
    let details = match parse_arguments() {
        Ok(details) => details,
        Err(_) => {
            print_error_and_exit(Error::NotEnoughArguments);
            panic!();
        }
    };

    match details.action {
        Action::Show => {
            show_command_tags(details, parse_toml());
        }
        Action::Search => {
            search_descriptions(details, parse_toml());
        }
        Action::Add => {
            add_description(details, &mut parse_toml());
        }
        _ => {}
    }

    ExitCode::SUCCESS
}
