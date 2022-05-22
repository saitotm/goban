use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    process::{Command, Output},
};

use std::fs;
use std::hash::Hash;

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
        let values = Self::values_to_string(values);
        let params = Params::new(keys, values);

        for (i, param) in params.iter().enumerate() {
            let cmd = self.translator.render(&param, &self.command);
            let shell = self.get_current_shell().unwrap_or_else(|| "sh".to_string());

            println!("\n[{} / {}]", i + 1, params.get_combination());
            println!("Parameters: {:?}", &param); // FIXME: show keys in the same order everytime.
            println!("$ {}", &cmd);

            let output = self.run_command(shell, cmd);

            // FIXME: remove unwrap
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();
            println!("[{}]", output.status);
        }
    }

    fn run_command(&self, shell: String, cmd: String) -> Output {
        Command::new(shell)
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    }

    // FIXME: Return Result instead of Option
    fn get_current_shell(&self) -> Option<String> {
        env::var("SHELL").ok()
    }

    fn read_file(&self) -> String {
        // FIXME: remove unwrap()
        fs::read_to_string(&self.file_name).unwrap()
    }

    fn values_to_string(values: Vec<Vec<Value>>) -> Vec<Vec<String>> {
        values
            .iter()
            .map(|list| {
                list.iter().map(|v| match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                }).collect()
            })
            .collect()
    }

    fn split_keys_values<K: Hash + Eq + Clone, V: Clone>(kvm: HashMap<K, V>) -> (Vec<K>, Vec<V>) {
        let keys = kvm.keys().cloned().collect::<Vec<K>>();
        let values = keys
            .iter()
            .map(|k| kvm.get(k).expect("it must be Some").clone())
            .collect::<Vec<V>>();

        (keys, values)
    }
}
