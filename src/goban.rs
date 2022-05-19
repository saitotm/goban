use std::{
    collections::HashMap,
    io::{self, Write},
    process::Command,
};

use std::hash::Hash;
use std::fs;

use serde_json::Value;

use crate::{params::Params, translator::Translator};

pub struct Goban<T: Translator> {
    command: String,
    file_name: String,
    translator: T,
}

impl<T: Translator> Goban<T> {
    pub fn new(command: String, file_name: String, translator: T) -> Self {
        Self {
            command,
            file_name,
            translator,
        }
    }

    pub fn run(&self) {
        let data = self.read_file();

        let kvm: HashMap<String, Vec<Value>> = serde_json::from_str(&data).unwrap();
        let (keys, values) = Self::split_keys_values(kvm);

        let params = Params::new(keys, values);
        for param in params.iter() {
            self.run_command(param);
        }
    }

    fn run_command(&self, param: HashMap<String, String>) {
        let cmd = self.translator.render(&param, &self.command);

        let output = match cmd.split_once(' ') {
            Some((program, arg)) => Command::new(program)
                .arg(arg)
                .output()
                .expect("failed to execute process"),
            None => Command::new(cmd)
                .output()
                .expect("failed to execute process"),
        };

        println!();
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        println!("{}", output.status);
    }

    fn read_file(&self) -> String {
        // FIXME: remove unwrap()
        fs::read_to_string(&self.file_name).unwrap()
    }

    fn split_keys_values<K: Hash + Eq + Clone, V: Clone>(kvm: HashMap<K, V>) -> (Vec<K>, Vec<V>) {
        let keys = kvm.keys().cloned().collect::<Vec<K>>();
        let values = keys.iter().map(|k| kvm.get(k).expect("it must be Some").clone()).collect::<Vec<V>>();

        (keys, values)
    }
}
