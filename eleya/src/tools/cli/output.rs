use std::iter::FromIterator;
use crate::tools::cli::vt100::escapes as ESC;

fn generate_status<'b, 'a>(a: &'b str, b: &'a str, c: &'b str) -> String{
    format!("({}{}{})", a, b, c)
}

fn line_and_column_output(l: usize, c: usize) -> String {
    format!("({}{}{},{}{}{})",
            ESC::BOLD, l, ESC::RESET,
            ESC::BOLD, c, ESC::RESET
    )
}

pub fn error(line: &str, error_header: &str,  error_msg: &str, line_number: usize, column_number: usize) {
    println!("{} {} in line {} :\n"
             , generate_status(ESC::RED, "error", ESC::RESET)
             , error_header
             , line_and_column_output(line_number, 1+column_number));
    let offset = (0..column_number).map(|_| " ").collect::<String>();
    println!("{}", line);
    println!("{}^---- {}", offset, error_msg);
    println!();
}

pub fn info(error_msg: &str) {
    println!("{} {}", generate_status(ESC::BOLD, "info", ESC::RESET), error_msg);
}

pub fn debug<'a>(msg: &'a str, line: &'a str){
    println!("{} {} : {}\n"
             , generate_status(ESC::GREEN, "debug", ESC::RESET)
             , msg
             , line);
}

pub fn comment(line: &str){
    println!("{}{}{}", ESC::DARK_GRAY, line, ESC::RESET);
}




