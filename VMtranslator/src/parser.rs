use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq)]
pub enum CommandType {
    Arithmetic,
    Push,
    Pop,
    Label,
    Goto,
    If,
    Function,
    Return,
    Call,
}

pub struct Parser {
    commands: Vec<Vec<String>>,
    current_cmd: Vec<String>,
}

impl Parser {
    pub fn new(f: File) -> Self {
        let reader = BufReader::new(f);
        let commands: Vec<Vec<String>> = reader.lines().map(|line| {
            let line = line.unwrap();
            let l = line.as_str();
            match l.find("//") {
                Some(n) => { // cut off the comment part
                    l[..n].trim().to_string()
                },
                None => {
                    l.trim().to_string()
                }
            }
        })
        .filter(|l| l.as_str() != "")
        .map(|l| l.split_whitespace().map(|s| String::from(s)).collect::<Vec<String>>())
        .collect();

        let commands_rev: Vec<Vec<String>> = commands.into_iter().rev().collect();

        Parser {
            commands: commands_rev,
            current_cmd: vec![],
        }
    }

    pub fn has_more_commands(&mut self) -> bool {
        self.commands.len() > 0
    }

    pub fn advance(&mut self) {
        if !self.has_more_commands() {
            panic!("cannot advance because no more commands");
        }
        if self.has_more_commands() {
            self.current_cmd = self.commands.pop().unwrap();
        }
    }

    pub fn command_type(&self) -> CommandType {
        match self.current_cmd[0].as_str() {
            "add" | "sub" | "neg" |
            "eq"  | "gt"  | "lt"  |
            "and" | "or"  | "not" => {
                CommandType::Arithmetic
            },
            "push" => {
                CommandType::Push
            },
            "pop" => {
                CommandType::Pop
            },
            "label" => {
                CommandType::Label
            },
            "goto" => {
                CommandType::Goto
            },
            "if-goto" => {
                CommandType::If
            },
            "function" => {
                CommandType::Function
            },
            "return" => {
                CommandType::Return
            },
            "call" => {
                CommandType::Call
            },
            _ => {
                panic!("no such command");
            },
        }
    }

    pub fn arg1(&self) -> String {
        if self.command_type() == CommandType::Return {
            panic!("this command has no arg1");
        }
        if self.command_type() == CommandType::Arithmetic {
            self.current_cmd[0].clone()
        } else {
            self.current_cmd[1].clone()
        }
    }

    pub fn arg2(&self) -> i16 {
        if self.command_type() != CommandType::Push 
        && self.command_type() != CommandType::Pop
        && self.command_type() != CommandType::Function
        && self.command_type() != CommandType::Call
        {
            panic!("this command has no arg2");
        }
        self.current_cmd[2].parse::<i16>().unwrap()
    }
}