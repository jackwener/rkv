use clap::{App, Arg};
use std::process::exit;
use rkv::kv::KV;

fn main() {
    // [env variables doc](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            App::new("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::new("KEY").about("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .about("The string value of the key")
                        .required(true),
                ),
        )
        .subcommand(
            App::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::new("KEY").about("A string key").required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("Remove a given key")
                .arg(Arg::new("KEY").about("A string key").required(true)),
        )
        .get_matches();


    match matches.subcommand() {
        Some(("set", _sub_m)) => {
            let key = matches.value_of("KEY").expect("KEY argument missing");
            let value = matches.value_of("VALUE").expect("VALUE argument missing");


        }
        Some(("get", _sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("remove", _sub_m)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => { } ,
    }
}
