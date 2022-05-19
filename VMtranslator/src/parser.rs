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
                    l[..n].trim()
                },
                None => {
                    l.trim()
                }
            }
        })
        .filter(|l| l != "")
        .map(|l| l.split_whitespace().collect::<Vec<String>>())
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

    pub arg1(&self) -> String {
        if self.command_type() == CommandType::Return {
            panic!("this command has no arg1");
        }
        if self.command_type() == CommandType::Arithmetic {
            self.current_cmd[0]
        } else {
            self.current_cmd[1]
        }
    }

    pub arg2(&self) -> String {
        if self.command_type() != CommandType::Push 
        && self.command_type() != CommandType::Pop
        && self.command_type() != CommandType::Function
        && self.command_type() != CommandType::Call
        {
            panic!("this command has no arg2");
        }
        self.current_cmd[2]
    }
}