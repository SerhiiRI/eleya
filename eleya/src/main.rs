
use std::{env, str, fs, process::exit, str::Lines};
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration};
use std::iter::FromIterator;
use std::borrow::Borrow;

mod parser;

//static mut ANIMATION_STACK:Animation = Animation::new();

fn clean_up_string(string: String) -> String{
    String::from(string.trim())
}


type uColumn = u16;
fn parse_parameter_opt(line: Vec<char>) -> Result<(String, String), (uColumn, String)> {
    // #:dupa=123
    let param = &line[2..];
    println!("1: {}", String::from_iter(param));
    if String::from_iter(param).trim().len() == 0 {
        return Result::Err((2, String::from("The parameter is empty, nonetheless param annotation(#:) is"))); }
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
    println!("Animation File:{} ", path);

    let file:String = fs::read_to_string(path).expect("");
    let lines:Vec<&str> = file.lines().collect();


//    if  lines.len() == 0 {return}
//
//    for line in lines {
//        let char_array: Vec<char> = line.chars().collect();
//        if char_array.len() == 0 {
//            continue;
//        }
//
////        match String::from_iter(&char_array[0..2]).as_str() {
////            "#:" => parse_parameter(line.chars().collect()),
////            _ =>
////        }
//
////        if let "#:" = String::from_iter(&char_array[0..2]).as_str() {
//        if let "#:" = String::from_iter("#:suka=bliat".chars()).as_str() {
//            let (param, value) = parse_parameter(line.chars().collect());
//            let str_line = format!("Param -'{}' value - '{}'", param, value);
//            println!("{}", str_line);
//        } else {
//            println!("line | {}", line);
//        }
//
//    }
    println!("PARAM : ");
    let line:Vec<char> = "#:suk".chars().collect();
    //if let "#:" = String::from_iter(&line).as_str()[0..2] {
    if let "#:" = &(String::from_iter(&line).as_str()[0..2]) {
//        let (param, value) = parse_parameter(line);
        match parse_parameter_opt(line.clone()) {
            Result::Ok((parameter, value)) => {
                let str_line = format!("Param -'{}' value - '{}'", parameter, value);
                println!("{}", str_line);
            },
            Result::Err((err_col, err_msg)) => {
                println!("[Error] In line:\n{}", String::from_iter(&line));
                let pointer_to_error = format!("{}^", (0..err_col).map(|_| " ").collect::<String>());
                println!("{}-{}", pointer_to_error, err_msg)
            }
        }
//        let str_line = format!("Param -'{}' value - '{}'", param, value);

    } else {
        println!("line | {}", String::from_iter(line));
    }
    //println!("Content:\n{}", anim_file);
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
