use crate::db::types::{KeyType, ValueType};
use std::path::Path;

// commands provided by the domain specific language.
pub enum Command {
    Put(KeyType, ValueType), // insert a kv pair into the db. Old value will be replaced.
    Get(KeyType),            // fetch the associated value of the given key if the key exists.
    Range(KeyType, KeyType), // fetch values in the key range [start_key, end_key).
    Delete(KeyType),         // remove the kv pair associated with the given key.
    Load(String),            // load kv pairs stored in the file.
    PrintStats, // print the current state of the db including the in-mem states and the on-disk states.
    Quit,       // terminate the session.
    Help,       // print help options.
}

impl Command {
    // construct a cmd from tokens parsed from command line input.
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<Command> {
        match tokens[0] {
            "p" | "put" => {
                if tokens.len() == 3 && is_valid_key(tokens[1]) && is_valid_value(tokens[2]) {
                    return Some(Command::Put(
                        tokens[1].as_ptr() as KeyType,
                        tokens[2].as_ptr() as ValueType,
                    ));
                }
                None
            }
            "g" | "get" => {
                if tokens.len() == 2 && is_valid_key(tokens[1]) {
                    return Some(Command::Get(tokens[1].as_ptr() as KeyType));
                }
                None
            }
            "r" | "range" => {
                if tokens.len() == 3 && is_valid_key(tokens[1]) && is_valid_key(tokens[2]) {
                    return Some(Command::Range(
                        tokens[1].as_ptr() as KeyType,
                        tokens[2].as_ptr() as KeyType,
                    ));
                }
                None
            }
            "d" | "delete" => {
                if tokens.len() == 2 && is_valid_key(tokens[1]) {
                    return Some(Command::Get(tokens[1].as_ptr() as KeyType));
                }
                None
            }
            "l" | "load" => {
                if tokens.len() == 2 && Path::new(tokens[1]).is_file() {
                    return Some(Command::Load(tokens[1].to_owned()));
                }
                None
            }
            "s" | "print" => {
                if tokens.len() == 1 {
                    return Some(Command::PrintStats);
                }
                None
            }
            "q" | "quit" => {
                if tokens.len() == 1 {
                    return Some(Command::Quit);
                }
                None
            }
            "h" | "help" => {
                if tokens.len() == 1 {
                    return Some(Command::Help);
                }
                None
            }
            _ => None,
        }
    }
}

/// return true if the int_str str can be casted to the key type without error.
fn is_valid_key(int_str: &str) -> bool {
    if let Err(_) = int_str.parse::<KeyType>() {
        return false;
    }
    return true;
}

/// return true if the int_str str can be casted to the value type without error.
fn is_valid_value(int_str: &str) -> bool {
    if let Err(_) = int_str.parse::<ValueType>() {
        return false;
    }
    return true;
}

/// print help options.
pub fn print_help() {
    static PUT: &str = "p | put <key> <value>";
    static GET: &str = "g | get <key>";
    static RANGE: &str = "r | range <start_key> <end_key>";
    static DELETE: &str = "d | delete <key>";
    static LOAD: &str = "l | load <command_batch_file>";
    static PRINT_STATS: &str = "s | print";
    static QUIT: &str = "q | quit";
    static HELP: &str = "h | help";

    print!(
        "  Usage:\n\t{:<35}{}\n\t{:<35}{}\n\t{:<35}{}\n\t{:<35}{}\n\t{:<35}{}\n\t{:<35}{}\n\t{:<35}{}\n\t{:<35}{}\n",
        PUT, "insert a key-value pair into the database",
        GET, "fetch the associated value of the given key",
        RANGE, "fetch values in the key range from start_key to end_key",
        DELETE, "delete the key-value pair associated with the given key",
        LOAD, "insert a sequence of key-value pairs stored in the file",
        PRINT_STATS, "print the current state of the database",
        QUIT, "terminate the session",
        HELP, "print this help message"
    );
}
