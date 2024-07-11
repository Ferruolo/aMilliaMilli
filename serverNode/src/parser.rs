use std::fs::File;
use std::io::{BufRead, BufReader};
use serverNode::internal_lang::{FakeDatum, KeyType, OperationsLang};
use serverNode::internal_lang::OperationsLang::{*};

// Parsing function to parse input lines into Command enum
fn parse_command(line: &str) -> Option<OperationsLang<FakeDatum>> {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }


    match parts[0] {
        "GET" => {
            if let (Ok(key)) = (parts[2].parse::<KeyType>()) {
                Some(Get(key))
            } else {
                None
            }
        },
        "SET" => {
            if (parts.len() < 5) { // TODO: Maybe be more rigorous than this. On the flip side, speed matters
                None
            } else {
                if let (Ok(key), Ok(val)) = (parts[2].parse::<KeyType>(), parts[4].parse::<FakeDatum>()) {
                    Some(Set(key, val))
                } else {
                    None
                }
            }
        },
        _ => {
            None
        },
    }
}

// Function to parse commands from a file and return a vector of commands
pub fn parse_commands_from_file(file_path: &str) -> Vec<OperationsLang<FakeDatum>> {
    let file = File::open(file_path).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut commands = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(command) = parse_command(&line) {
                commands.push(command);
            } else {
                eprintln!("Failed to parse command from line: {}", line);
            }
        }
    }

    commands
}