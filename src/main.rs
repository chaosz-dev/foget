use std::{
    env,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    process::{self, ExitCode},
};

use toml_edit::*;

enum Action {
    Search,
    Show,
    Add,
    Modify,
    Delete,
}

enum Error {
    NotEnoughArguments,
    UnspecifiedCommand,
    CouldNotOpenFile,
    Internal,
    CommandNotFound,
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
        Error::CommandNotFound => {
            eprintln!("Command not found in database.")
        }
        Error::UnknownReason | _ => {
            eprintln!("Unknown error. Exiting.");
        }
    }

    process::exit(1);
}

fn print_help() {
    println!("Usage:\n  foget [action] [action parameters]\n");
    println!("Actions:");

    println!("[search or se] [string to search]");
    println!("   show commands which description cointain the entered string");

    println!("[show or sho or s] [name of the command]");
    println!("   show command and associated tags");

    println!("[add or a] [name of the command] [tag to be added]");
    println!("   add a new command and tags to the datatabase");

    println!("[modify or mod or m] [name of the command] [new tags]");
    println!("   modify a command by adding new tags to the datatabase");

    println!("[del] [name of the command]");
    println!("   delete the command and it's associated tags");

    process::exit(0);
}

fn parse_arguments() -> Result<CommandDetails, ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
    }

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

            return Result::Ok(CommandDetails {
                action: Action::Modify,
                args: vec![args[2].clone(), args[3].clone()],
            });
        }
        "delete" | "del" | "d" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            if args.len() < 4 {
                return Result::Ok(CommandDetails {
                    action: Action::Delete,
                    args: vec![args[2].clone()],
                });
            }

            return Result::Ok(CommandDetails {
                action: Action::Delete,
                args: vec![args[2].clone(), args[3].clone()],
            });
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
        "help" | "h" | _ => {
            print_help();
        }
    }

    print_error_and_exit(Error::UnknownReason);
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

        let arr = match toml[&details.args[0]]["tags"].as_array() {
            Some(arr) => arr,
            _ => {
                print_error_and_exit(Error::Internal);
                panic!();
            }
        };

        let mut i = 0;
        while i < arr.len() {
            println!(
                "\t{}",
                toml[&details.args[0]]["tags"][i]
                    .as_str()
                    .unwrap_or_else(|| {
                        print_error_and_exit(Error::Internal);
                        ""
                    })
            );
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
                    if desc.is_str()
                        && desc
                            .as_str()
                            .unwrap_or_else(|| {
                                print_error_and_exit(Error::Internal);
                                ""
                            })
                            .contains(&details.args[0])
                    {
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

// TODO: filter duplicates
fn add_description(details: CommandDetails, toml: &mut toml_edit::DocumentMut) {
    if toml.contains_key(&details.args[0]) {
        let arr: &mut Array = match toml[&details.args[0]]["tags"].as_array_mut() {
            None => {
                print_error_and_exit(Error::Internal);
                &mut toml_edit::Array::new()
            }
            Some(arr) => arr,
        };

        arr.push(details.args[1].clone());
    } else {
        toml[&details.args[0]] = Item::Table(toml_edit::Table::new());
        toml[&details.args[0]]["tags"] =
            Item::Value(toml_edit::Value::Array(toml_edit::Array::new()));
        match toml[&details.args[0]]["tags"].as_array_mut() {
            Some(arr) => arr.push(details.args[1].clone()),
            _ => {}
        }
    }

    write_doc_to_file(toml);

    if details.args.len() < 2 {
        println!("Successfully added {} to database", details.args[0]);
    } else {
        println!(
            "Successfully added tag: '{}' to '{}' to command database",
            details.args[0], details.args[1]
        );
    }
}

fn delete(details: CommandDetails, toml: &mut toml_edit::DocumentMut) {
    if details.args.len() < 2 {
        toml.remove_entry(&details.args[0]);
    } else {
        match toml[&details.args[0]]["tags"].as_array_mut() {
            None => {
                print_error_and_exit(Error::Internal);
            }
            Some(arr) => {
                arr.retain(|x| x.as_str().unwrap_or("") != details.args[1]);
            }
        };
    }

    write_doc_to_file(toml);
    println!("Successfully deleted command {}", details.args[0]);
}

fn modify(details: CommandDetails, toml: &mut toml_edit::DocumentMut) {
    if !toml.contains_key(details.args[0].as_str()) {
        print_error_and_exit(Error::CommandNotFound);
    }

    match toml[&details.args[0]]["tags"].as_array_mut() {
        None => {
            print_error_and_exit(Error::Internal);
        }
        Some(arr) => {
            arr.push(details.args[1].clone());
        }
    };

    write_doc_to_file(toml);

    println!(
        "Successfully added {} to {}",
        details.args[1], details.args[0]
    );
}

fn write_doc_to_file(toml: &toml_edit::DocumentMut) {
    let datab = if cfg!(unix) {
        std::env::home_dir()
            .unwrap_or(PathBuf::new())
            .into_os_string()
            .into_string()
            .unwrap_or(String::new())
            + "/foget/descriptions/unix.toml"
    } else {
        todo!();
    };

    let mut file: File = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(datab)
    {
        Err(why) => {
            print_error_and_exit(Error::CouldNotOpenFile);
            panic!("{:?}", why);
        }
        Ok(file) => file,
    };

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
        Action::Delete => {
            delete(details, &mut parse_toml());
        }
        Action::Modify => {
            modify(details, &mut parse_toml());
        }
        _ => {
            print_error_and_exit(Error::UnknownReason);
        }
    }

    ExitCode::SUCCESS
}
