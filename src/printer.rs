use crate::cli::Cli;

pub fn me_print_stdout(msg: &str, args: &Cli) {
    match args.no_markdown_input {
        true => {
            println!("ME: {}", &msg);
        }
        _ => {
            println!("ME: {}", termimad::inline(&msg));
        }
    }
}

pub fn ai_print_stdout(msg: &str, args: &Cli) {
    match args.no_markdown_output {
        true => {
            println!("AI: {}", &msg);
        }
        _ => {
            println!("AI: {}", termimad::inline(&msg));
        }
    }
}

pub fn print_error(msg: &str) {
    println!("ERROR: {}", &msg);
}
