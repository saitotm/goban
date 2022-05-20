use std::{
    collections::HashMap,
    io::{self, Write},
    process::{Command, Output},
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

        for (i, param) in params.iter().enumerate() {
            let cmd = self.translator.render(&param, &self.command);

            println!("\n[{} / {}]", i + 1, params.get_combination());
            println!("Parameters: {:?}", &param); // FIXME: show keys in the same order everytime.
            println!("{}", &cmd);

            let output = self.run_command(cmd);

            // FIXME: remove unwrap
            io::stdout().write_all(&output.stdout).unwrap(); 
            io::stderr().write_all(&output.stderr).unwrap();
            println!("[{}]", output.status);
        }
    }

    fn run_command(&self, cmd: String) -> Output {
        match cmd.split_once(' ') {
            Some((program, arg)) => Command::new(program)
                .arg(arg)
                .output()
                .expect("failed to execute process"),
            None => Command::new(cmd)
                .output()
                .expect("failed to execute process"),
        }
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
