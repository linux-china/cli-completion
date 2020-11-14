//! clap App for command cli
use clap::{App, Arg, ValueHint};

const COMMAND: &str = "cli-completion";
const VERSION: &str = "0.1.0";

pub fn build_app() -> App<'static> {
    // init Clap
    App::new(COMMAND)
        .version(VERSION)
        .author("linux_china https://twitter.com/linux_china")
        .about("CLI completion for bash, zsh, fish and powershell.")
        .arg(
            Arg::new("zsh")
                .long("zsh")
                .takes_value(false)
                .about("Zsh completion")
                .required(false),
        )
        .arg(
            Arg::new("powershell")
                .long("powershell")
                .takes_value(false)
                .about("PowerShell completion")
                .required(false),
        )
        .arg(
            Arg::new("bash")
                .long("bash")
                .takes_value(false)
                .about("Bash completion")
                .required(false),
        )
        .arg(
            Arg::new("fish")
                .long("fish")
                .takes_value(false)
                .about("Fish completion")
                .required(false),
        )
        .arg(
            Arg::new("yaml")
                .takes_value(true)
                .required(true)
                .value_name("FILE")
                .value_hint(ValueHint::FilePath)
                .about("CLI clap-rs yaml file"),
        )
}
