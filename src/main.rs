use std::process::exit;

use clap::{Arg, Command};
use smartkv::SmartKV;

fn main() {
    let cli_app = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .disable_help_subcommand(true)
        .subcommand_required(true)
        .arg_required_else_help(true)

        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a given key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .get_matches();

    let mut store = SmartKV::new();
    match cli_app.subcommand() {
        Some(("set", sub_m)) => {
            store.set(
                sub_m.value_of("KEY").expect("required").to_string(),
                sub_m.value_of("VALUE").expect("required").to_string(),
            );
            eprintln!("unimplemented");
            exit(1)
        }
        Some(("get", _sub_m)) => {
            // let get_value = store.get(sub_m.value_of("KEY").expect("required").to_string());
            // match get_value {
            //     Some(t) => eprintln!("Value: {}", t),
            //     None => eprintln!("Value NOT FOUND"),
            // }
            eprintln!("unimplemented");
            exit(1)
        }
        Some(("rm", sub_m)) => {
            store.remove(sub_m.value_of("KEY").expect("required").to_string());
            eprintln!("unimplemented");
            exit(1)
        }
        _ => eprintln!("Wrong input"),
    }
}
