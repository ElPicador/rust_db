use std::io;
use std::io::Write;

mod statement;
mod row;

fn main() {
    loop {
        print_prompt();
        let input = &read_input();

        if input.starts_with('.') {
            match do_meta_command(input) {
                Some(_) => continue,
                None => {
                    println!("Unrecognized command '{}'.", input);
                    continue
                }
            }
        }

        let statement = statement::prepare_statement(input);
        match statement {
            Err(_) => {
                println!("Unrecognized keyword at start of '{}'.", input);
                continue;
            }
            Ok(s) => {
                statement::execute_statement(s);
                println!("Executed.");
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    match io::stdout().flush() {
        Err(_) => panic!("can not flush stdin"),
        _ => (),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(_) => panic!("can not read from stdin"),
        _ => input.trim().to_string(),
    }
}

fn do_meta_command(input: &String) -> Option<()> {
    if input == ".exit" {
        std::process::exit(0);
    } else {
        Some(())
    }
}
