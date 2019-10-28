mod parser;
mod tools;

use std::{env, str, fs, process::exit, str::Lines};
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration, initialize_global_configuration};
use crate::tools::cli::vt100::escapes as ESC;
use crate::tools::cli::output::{error, debug, comment};
use std::iter::FromIterator;
use std::borrow::Borrow;

//static mut ANIMATION_STACK:Animation = Animation::new();
type uColumn = u16;
type uLine   = u32;

fn clean_up_string(string: String) -> String{
    String::from(string.trim())
}



fn parse_parameter_opt(line: Vec<char>) -> Result<(String, String), (uColumn, String)> {
    // #:dupa=123
    let param = &line[2..];
    if String::from_iter(param).trim().len() == 0 {
        return Result::Err((2, String::from("The parameter is empty, nonetheless param annotation(#:) is")));
    }
    if let Some(pos) = param.iter().position(|&x| x == '=') {
        let param_name: String = clean_up_string(String::from_iter(&param[0..pos]));
        let value: String = clean_up_string((String::from_iter(&param[(pos+1)..])));
        if let Some(size) = param_name.find(' ') {
            return Result::Err((2+size as uColumn, String::from("Not approved space symbol in parameter name")))
        }
        if param_name.len() == 0 {
            return Result::Err((2, String::from("Parameter is empty")));}
        return Result::Ok((param_name, value));
    } else {
        //#:dupa
        let end_line = String::from_iter(param).trim().len();
        return Result::Err((2+end_line as uColumn, String::from("Expected '='(equal) symbol")));
    }
    return Result::Err((0, String::from("Unknown parsing error")));
}




fn create_animation(path: &String){
//    let mut frame_global_config: FrameConfiguration = Vec::new();

    println!("Animation File:{} ", path);
    let file:String = fs::read_to_string(path).expect("");
    let lines:Vec<&str> = file.lines().collect();
    let mut default_animation_config = initialize_global_configuration();

    if  lines.len() == 0 {return}
    let mut line_iterator = lines.iter();
    loop {
        if let Some(&line) = line_iterator.next() {
            let char_array: Vec<char> = line.chars().collect();
            line.len();
            if char_array.len() == 0 { continue; }
            if char_array.len() == 1 {
                if let '#' = &char_array[0] {
                    comment(line);
                    continue
                }
            }
            if char_array.len() >= 2 {
                if let "# " = &(String::from_iter(&char_array)[0..2]) {
                    comment(line);
                    continue
                }
            }
            if let "#:" = &(String::from_iter(&char_array)[0..2]) {
                match parse_parameter_opt(char_array.clone()) {
                    Ok((parameter, value)) => {
                        match parameter.as_str() {
                            "delay" => {
                                if let Some(delay) = FrameConfiguration::parse_delay(&value){
                                    default_animation_config.push(delay);
                                }
                            },
                            "offset-y" => {},
                            "offset-x" => {},
                            _ => {}
                        };
                        debug("param parsing", format!("{}={}", parameter, value).as_str())
                    },
                    Err((err_col, err_msg)) => {
                        error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), 0, err_col);
                    }
                }
            }
            else {
                println!("| {}", line)
            }
            if let "---" = &(String::from_iter(&char_array)[0..3]) {

            }
            if let "--" = &(String::from_iter(&char_array)[0..3]) {

            }
            if let "-" = &(String::from_iter(&char_array)[0..3]) {

            }




        } else {
            break
        }
    }
    println!("Parameters stack length {}", &default_animation_config.len());
    for i in &default_animation_config{
        i.print();
    }

//    for line in lines {
//        let char_array: Vec<char> = line.chars().collect();
//        if char_array.len() == 0 {
//            continue;
//        }
//        if char_array.len() == 1 {
//            if let '#' = &char_array[0] {
//                comment(line);
//                continue
//            }
//        }
//        if char_array.len() >= 2 {
//            if let "# " = &(String::from_iter(&char_array)[0..2]) {
//                comment(line);
//                continue
//            }
//        }
//
//        if let "#:" = &(String::from_iter(&char_array)[0..2]) {
//            match parse_parameter_opt(char_array.clone()) {
//                Result::Ok((parameter, value)) => {
//                    debug("param parsing", format!("{}={}", parameter, value).as_str())
//                },
//                Result::Err((err_col, err_msg)) => {
//                    error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), 0, err_col);
//                }
//            }
//        } else {
//            println!("| {}", line)
//        }
//
//    }

//    println!("PARAM : ");
//    let line:Vec<char> = "#:suka job=".chars().collect();
//    if let "#:" = &(String::from_iter(&line).as_str()[0..2]) {
////        let (param, value) = parse_parameter(line);
//        match parse_parameter_opt(line.clone()) {
//            Result::Ok((parameter, value)) => {
//                let str_line = format!("Param -'{}' value - '{}'", parameter, value);
//                println!("{}", str_line);
//            },
//            Result::Err((err_col, err_msg)) => {
//                error(String::from_iter(&line).as_str(), "Parsing error", err_msg.as_str(), 0, err_col);
//            }
//        }
//    } else {
//        println!("line | {}", String::from_iter(line));
//    }
}

fn exit_program(){
    println!("Animation file not found");
    exit(0);
}

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();
    let animation_file_path:Option<&String> = cli_arguments.get(1);
    match animation_file_path {
        Option::Some(path) => create_animation(path),
        Option::None => exit_program(),
    }


    println!("Input arguments {:?}", cli_arguments);

}

