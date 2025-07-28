use std::{
    env, fs,
    fs::File,
    io::prelude::*,
    path::PathBuf,
    process::{self, ExitCode},
};

use colored::{control::set_override, *};
use toml_edit::*;

enum Action {
    Search,
    Show,
    Add,
    Modify,
    Delete,
    Default,
}

enum Error {
    NotEnoughArguments,
    CouldNotOpenFile,
    Internal,
    CommandNotFound,
    TagNotFound,
    AlreadyInDatabase,
}

struct CommandDetails {
    action: Action,
    args: Vec<String>,
    descriptions: PathBuf,
}

fn search_for_toml() -> PathBuf {
    let path: PathBuf = PathBuf::new();

    match env::var("HOME") {
        Ok(res) => {
            let descriptions: PathBuf = [res, "unix.toml".to_string()].into_iter().collect();
            match fs::exists(&descriptions) {
                Ok(res) => {
                    if res {
                        return descriptions;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    match env::var("HOME") {
        Ok(res) => {
            let descriptions: PathBuf = [res.as_str(), ".config", "foget", "unix.toml"]
                .into_iter()
                .collect();
            match fs::exists(&descriptions) {
                Ok(res) => {
                    if res {
                        return descriptions;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    match env::var("FOGET_DESCRIPTIONS") {
        Ok(res) => {
            let descriptions: PathBuf = PathBuf::from(res);
            match fs::exists(&descriptions) {
                Ok(res) => {
                    if res {
                        return descriptions;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }

    print_error_and_exit(Error::CouldNotOpenFile);
    path
}

impl Default for CommandDetails {
    fn default() -> CommandDetails {
        CommandDetails {
            action: Action::Default,
            args: Vec::new(),
            descriptions: search_for_toml(),
        }
    }
}

fn print_error_and_exit(err: Error) {
    eprint!("{}", String::from("Error: ").red());
    match err {
        Error::NotEnoughArguments => {
            eprintln!("Incorrect amount of arguments! You should use [foget] [action] [action parameters]");
        }
        Error::CouldNotOpenFile => {
            eprintln!("Could not open database file.");
        }
        Error::Internal => {
            eprintln!("Unexpected internal error.");
        }
        Error::CommandNotFound => {
            eprintln!("Command not found in database.")
        }
        Error::TagNotFound => {
            eprintln!("Tag not found in database.");
        }
        Error::AlreadyInDatabase => {
            eprintln!("Tag already in database.")
        }
    }

    process::exit(1);
}

fn print_help() {
    println!(
        "{}",
        "Usage:\n  foget [action] [action parameters]\n".bold()
    );
    println!("{}", "Actions:".red());

    println!("{}", "[search or se] [string to search]".bright_cyan());
    println!("   show commands which description cointain the entered string");

    println!(
        "{}",
        "[show or sho or s] [name of the command]".bright_cyan()
    );
    println!("   show command and associated tags");

    println!(
        "{}",
        "[add or a] [name of the command] [tag to be added]".bright_cyan()
    );
    println!("   add a new command and tags to the datatabase");

    println!(
        "{}",
        "[modify or mod or m] [name of the command] [new tags]".bright_cyan()
    );
    println!("   modify a command by adding new tags to the datatabase");

    println!("{}", "[del] [name of the command]".bright_cyan());
    println!("   delete the command and it's associated tags");

    process::exit(0);
}

fn search_for_options(details: &mut CommandDetails, args: &Vec<String>) {
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--no-color" => {
                set_override(false);
            }
            "--descriptions" => {
                if args.len() < i + 1 {
                    print_error_and_exit(Error::NotEnoughArguments);
                }

                details.descriptions = PathBuf::from(args[i + 1].clone());
                i += 1;
            }
            _ => {}
        }

        i += 1;
    }
}

fn parse_arguments() -> CommandDetails {
    let args: Vec<String> = env::args().collect();
    let mut details: CommandDetails = CommandDetails {
        ..Default::default()
    };

    if args.len() < 2 {
        print_help();
    }

    match args[1].as_str() {
        "add" | "a" => {
            if args.len() < 4 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            search_for_options(&mut details, &args);

            details.action = Action::Add;
            details.args = vec![args[2].clone(), args[3].clone()];
        }
        "modify" | "m" | "mod" => {
            if args.len() < 4 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            search_for_options(&mut details, &args);

            details.action = Action::Modify;
            details.args = vec![args[2].clone(), args[3].clone()];
        }
        "delete" | "del" | "d" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            search_for_options(&mut details, &args);

            details.action = Action::Delete;

            if args.len() < 4 {
                details.args = vec![args[2].clone()];
            } else {
                details.args = vec![args[2].clone(), args[3].clone()];
            }
        }
        "show" | "sho" | "sh" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            search_for_options(&mut details, &args);

            details.action = Action::Show;
            details.args = vec![args[2].clone()];
        }
        "search" | "se" => {
            if args.len() < 3 {
                print_error_and_exit(Error::NotEnoughArguments);
            }

            search_for_options(&mut details, &args);

            details.action = Action::Search;
            details.args = vec![args[2].clone()];
        }
        "help" | "h" | _ => {
            search_for_options(&mut details, &args);
            print_help();
        }
    }

    details
}

fn parse_toml(location: &PathBuf) -> DocumentMut {
    let mut file: File = match File::open(location) {
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
        println!(
            "{} :`{}`",
            "Found descriptions for command".bold(),
            details.args[0].blue().bold()
        );

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
                " {}. {}",
                (i + 1).to_string().red(),
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
        println!("{}", "Commands with matching functionality:".bold());

        let mut i = 0;
        while i < commands.len() {
            println!("{}", commands[i].blue().bold(),);
            let mut j = 0;
            while j < toml[&commands[i]]["tags"]
                .as_array()
                .unwrap_or(&toml_edit::Array::new())
                .len()
            {
                println!(
                    "  {}. {}",
                    (j + 1).to_string().red(),
                    toml[&commands[i]]["tags"][j]
                        .as_str()
                        .unwrap_or_else(|| {
                            print_error_and_exit(Error::Internal);
                            ""
                        })
                        .trim()
                );
                j += 1;
            }
            i += 1;
        }
    } else {
        print_error_and_exit(Error::TagNotFound);
    }
}

fn add_description(details: CommandDetails, toml: &mut toml_edit::DocumentMut) {
    if toml.contains_key(&details.args[0]) {
        let arr: &mut Array = match toml[&details.args[0]]["tags"].as_array_mut() {
            None => {
                print_error_and_exit(Error::Internal);
                &mut toml_edit::Array::new()
            }
            Some(arr) => arr,
        };

        if arr
            .iter()
            .find(|&x| x.as_str().unwrap_or("") == details.args[1])
            .is_some()
        {
            print_error_and_exit(Error::AlreadyInDatabase);
        }

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

    write_doc_to_file(details.descriptions, toml);

    if details.args.len() < 2 {
        println!("Successfully added {} to database", details.args[0].red());
    } else {
        println!(
            "Successfully added tag: '{}' to '{}' to command database",
            details.args[0].blue(),
            details.args[1].red()
        );
    }
}

fn delete(details: CommandDetails, toml: &mut toml_edit::DocumentMut) {
    if details.args.len() < 2 {
        toml.remove_entry(&details.args[0]);
        println!("Successfully deleted command {}", details.args[0].red());
    } else {
        match toml[&details.args[0]]["tags"].as_array_mut() {
            None => {
                print_error_and_exit(Error::Internal);
            }
            Some(arr) => {
                arr.retain(|x| x.as_str().unwrap_or("") != details.args[1]);
            }
        };
        println!(
            "Successfully deleted command {}'s entry: {}",
            details.args[0].red(),
            details.args[1].blue()
        );
    }

    write_doc_to_file(details.descriptions, toml);
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

    write_doc_to_file(details.descriptions, toml);

    println!(
        "Successfully added {} to {}",
        details.args[1].blue(),
        details.args[0].red()
    );
}

fn write_doc_to_file(descriptions: PathBuf, toml: &toml_edit::DocumentMut) {
    let mut file: File = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(descriptions)
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
    let details = parse_arguments();
    let mut toml = parse_toml(&details.descriptions);

    match details.action {
        Action::Show => {
            show_command_tags(details, toml);
        }
        Action::Search => {
            search_descriptions(details, toml);
        }
        Action::Add => {
            add_description(details, &mut toml);
        }
        Action::Delete => {
            delete(details, &mut toml);
        }
        Action::Modify => {
            modify(details, &mut toml);
        }
        _ => {
            print_error_and_exit(Error::Internal);
        }
    }

    ExitCode::SUCCESS
}
