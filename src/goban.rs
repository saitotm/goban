use std::{
    io::{self, Write},
    process::Command,
};

pub struct Goban {}

impl Goban {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&self) {
        let output = Command::new("echo")
            .arg("hello goban")
            .output()
            .expect("failed to execute process");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("{}", output.status);
    }
}
