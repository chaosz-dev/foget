use std::{env, process, process::ExitCode};

enum Action {
    Add,
    Modify,
    Delete,
    Show,
}

enum Error {
    NotEnoughArguments,
    UnspecifiedCommand,
}

struct CommandDetails {
    action: Action,
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

fn parse_arguments() -> CommandDetails {
    let args: Vec<String> = env::args().collect();
    let mut action: Action = Action::Show;

    match args[1].as_str() {
        "add" | "a" => {
            if args.len() < 4 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            action = Action::Add;
        }
        "modify" | "m" | "mod" => {
            if args.len() < 4 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            action = Action::Modify;
        }
        "delete" | "del" | "d" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            action = Action::Delete;
        }
        "show" | "s" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            action = Action::Show;
        }
        "help" | "h" => {
            print_help();
        }
        _ => {
            print_error_and_exit(Error::UnspecifiedCommand);
        }
    }

    CommandDetails { action: action }
}

fn parse_toml() {
    // TODO
}

fn main() -> ExitCode {
    parse_arguments();

    ExitCode::SUCCESS
}
