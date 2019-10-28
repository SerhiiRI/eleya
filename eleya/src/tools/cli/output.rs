use std::iter::FromIterator;
use crate::tools::cli::vt100::escapes as ESC;

// = status(ESC::LIGHT_RED, "error", ESC::RESET);
////static ErrorType: &'static str = format!("({}{}{})",   ESC::LIGHT_RED, "error", ESC::RESET).as_str();
//static InfoType: String = format!("({}{}{})",    ESC::LIGHT_BLUE, "info", ESC::RESET);
//static WarningType: String = format!("({}{}{})", ESC::LIGHT_YELLOW, "warn", ESC::RESET);


fn GenerateStatus<'b, 'a>(a: &'b str, b: &'a str, c: &'b str) -> String{
    format!("({}{}{})", a, b, c)
}

fn line_and_column_output(l: u32, c: u16) -> String {
    format!("({}{}{},{}{}{})",
            ESC::BOLD, l, ESC::RESET,
            ESC::BOLD, c, ESC::RESET
    )
}

pub fn error(line: &str, error_header: &str,  error_msg: &str, line_number: u32, column_number: u16) {
    println!("{} {} in line {} :\n"
             , GenerateStatus(ESC::RED, "error", ESC::RESET)
             , error_header
             , line_and_column_output(line_number, 1+column_number));
    let offset = (0..column_number).map(|_| " ").collect::<String>();
//    println!("{} {} in line {} :", ErrorType, error_msg, line_and_column_output(line_number, column_number));
    println!("{}", line);
    println!("{}^---- {}", offset, error_msg);
    println!();
}

pub fn debug<'a>(msg: &'a str, line: &'a str){
    println!("{} {} : {}\n"
             , GenerateStatus(ESC::GREEN, "debug", ESC::RESET)
             , msg
             , line);
}

pub fn comment(line: &str){
    println!("{}{}{}", ESC::DARK_GRAY, line, ESC::RESET);
}




