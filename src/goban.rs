use std::{
    collections::HashMap,
    env,
    io::{self, Write},
    process::{Command, Output},
};

use std::fs;
use std::hash::Hash;

use anyhow::{Context, Result};
use serde_json::Value;

use crate::{params::Params, translator::Translator};

pub struct Goban<T: Translator> {
    shell: String,
    data: String,
    command: String,
    translator: T,
}

impl<T: Translator> Goban<T> {
    pub fn new(command: String, filename: String, translator: T) -> Result<Self> {
        let shell = get_current_shell().unwrap_or_else(|_| "sh".to_string());
        let data = read_file(filename)?;

        Ok(Self {
            shell,
            command,
            data,
            translator,
        })
    }

    pub fn run(&self) -> Result<()> {
        let kvm: HashMap<String, Vec<Value>> =
            serde_json::from_str(&self.data).context("The format of the input data is invalid.")?;
        let (keys, values) = split_keys_values(kvm);
        let values = values_to_strings(values);
        let params = Params::new(keys, values)?;

        for (i, param) in params.iter()?.enumerate() {
            let cmd = self
                .translator
                .render(&param, &self.command)
                .context("Failed to replace keys in the command with values in the input data.")?;

            println!("\n[{} / {}]", i + 1, params.get_combination());
            println!("Parameters: {:?}", &param); // FIXME: show keys in the same order everytime.
            println!("$ {}", &cmd);

            let output = self.run_command(cmd);

            io::stdout()
                .write_all(&output.stdout)
                .context("Failed to write the result to stdout.")?;
            io::stderr()
                .write_all(&output.stderr)
                .context("Failed to write the result to stderr.")?;
            println!("[{}]", output.status);
        }

        Ok(())
    }

    fn run_command(&self, cmd: String) -> Output {
        Command::new(&self.shell)
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("failed to execute process")
    }
}

fn read_file(filename: String) -> Result<String> {
    let data = fs::read_to_string(filename)
        .context(format!("Failed to read {}", filename))?;
    Ok(data)
}

fn get_current_shell() -> Result<String> {
    env::var("SHELL").context("Failed to get the current shell.")
}

fn split_keys_values<K: Hash + Eq + Clone, V: Clone>(kvm: HashMap<K, V>) -> (Vec<K>, Vec<V>) {
    let keys = kvm.keys().cloned().collect::<Vec<K>>();
    let values = keys
        .iter()
        .map(|k| kvm.get(k).expect("it must be Some").clone())
        .collect::<Vec<V>>();

    (keys, values)
}

fn values_to_strings(values: Vec<Vec<Value>>) -> Vec<Vec<String>> {
    values
        .iter()
        .map(|list| {
            list.iter()
                .map(|v| match v {
                    Value::String(s) => s.clone(),
                    _ => v.to_string(),
                })
                .collect()
        })
        .collect()
}
