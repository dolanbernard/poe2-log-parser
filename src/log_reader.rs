use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    path::Path,
};

use regex::Regex;

pub fn find(log_file: &String,
    character_name: &Option<String>,
    regex: &String,
    writer: &mut Box<dyn Write>,
) {
    let chat_regex = Regex::new(&CHAT_MESSAGE_REGEX).unwrap();
    let search_regex = Regex::new(regex).unwrap();
    file_to_lines(log_file).for_each(|line| {
        let cap = chat_regex.captures(&line);
        if let Some(cap) = cap {
            if let Some(character_name) = character_name {
                if *character_name == cap["username"] {
                    if let Some(_) = search_regex.captures(&cap["message"]) {
                        write_chat_message(writer, &cap["username"], &cap["message"]);
                    }
                }
            } else {
                if let Some(_) = search_regex.captures(&cap["message"]) {
                    write_chat_message(writer, &cap["username"], &cap["message"]);
                }
            }
        }
    });
}

pub fn clean(log_file: &String, writer: &mut Box<dyn Write>) {
    let chat_regex = Regex::new(&CHAT_MESSAGE_REGEX).unwrap();
    file_to_lines(log_file).for_each(|line| {
        let cap = chat_regex.captures(&line);
        if let Some(cap) = cap {
            write_chat_message(writer, &cap["username"], &cap["message"]);
        }
    });
}

fn write_chat_message(writer: &mut Box<dyn Write>, character_name: &str, message: &str) {
    write(writer, b"#");
    write(writer, character_name.as_bytes());
    write(writer, b": ");
    write(writer, message.as_bytes());
    write(writer, b"\n");
}

fn write(writer: &mut Box<dyn Write>, bytes: &[u8]) {
    writer.write(bytes).unwrap();
}

pub fn file_to_lines(filename: impl AsRef<Path>) -> impl Iterator<Item = String> {
    let file = File::open(filename).expect("could not read file");
    let buf = BufReader::new(file);
    buf.lines().filter_map(|l| l.ok())
}


const CHAT_MESSAGE_REGEX: &'static str = "\\d{4}\\/\\d{2}\\/\\d{2} \\d{2}:\\d{2}:\\d{2} \\d{9} [abcdef\\d]{8} \\[INFO Client \\d{3}\\] #(?<username>.*): (?<message>.*)";
