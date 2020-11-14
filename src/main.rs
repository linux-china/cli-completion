mod app;

use crate::app::build_app;
use clap_generate::generate;
use clap_generate::generators::{Bash, Zsh, Fish, PowerShell};
use clap::{App};
use clap::ArgMatches;
use yaml_rust::{YamlLoader, Yaml};
use std::fs;

fn main() {
    let app = build_app();
    let matches = app.get_matches();
    if matches.is_present("yaml") {
        let yaml_file = matches.value_of("yaml").unwrap();
        let yaml_text = fs::read_to_string(yaml_file).unwrap();
        let yaml_vec = YamlLoader::load_from_str(&yaml_text).unwrap();
        if yaml_vec.len() == 1 {
            let yaml = yaml_vec.get(0).unwrap();
            generate_shell_completion(&matches, &yaml);
        } else {
            println!("😂 Incorrect yaml file!");
        }
    } else {
        println!("😂 Please use `cli-completion --zsh multipass.yaml` to generate shell completion!");
    }
}

pub fn generate_shell_completion(args: &ArgMatches, yaml: &Yaml) {
    let app = App::from(yaml);
    let command = app.get_name();
    if args.is_present("zsh") {
        generate::<Zsh, _>(&mut app.clone(), command, &mut std::io::stdout());
    } else if args.is_present("bash") {
        generate::<Bash, _>(&mut app.clone(), command, &mut std::io::stdout());
    } else if args.is_present("fish") {
        generate::<Fish, _>(&mut app.clone(), command, &mut std::io::stdout());
    } else if args.is_present("powershell") {
        generate::<PowerShell, _>(&mut app.clone(), command, &mut std::io::stdout());
    } else {
        println!("😂 the shell not support!");
    }
}
