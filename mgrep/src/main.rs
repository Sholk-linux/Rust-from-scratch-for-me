use std::{env, process};
use mgrep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Проблема передачи аргументов: {err}");
        process::exit(1);
    });


    if let Err(e) = mgrep::run(config) {
        eprintln!("Ошибка программы: {e}");
        process::exit(1);
    }
}




