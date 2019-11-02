mod parser;
mod tools;

use std::{env, str, fs, process::exit, str::Lines};
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration, FrameSetting};
use crate::tools::cli::vt100::escapes as ESC;
use crate::tools::cli::output::{error, debug, comment, info};
use std::iter::FromIterator;


//static mut ANIMATION_STACK:Animation = Animation::new();
type UColumn = usize;
type ULine   = usize;

const COMMENT: &str  = "#";
const PARAM:   &str  = "#:";
const FRAME:   &str  = "---";

fn is_empty(line: &str) -> bool{
    line.to_string().trim().len() == 0
}
fn is_comment(line: &str) -> bool{
    match line.len() {
        0 => {true},
        1 => {&line[0..1] == COMMENT},
        _ => {&line[0..2] != PARAM},
    }
}
fn is_param(line: &str) -> bool{
    match line.len() {
	0...1 => {false},
	_     => {&line[0..2] == PARAM},
    }
}

fn initialize_settings(lines: &Vec<&str>) -> Result<FrameSetting, FrameSetting>{
    let mut setting: FrameSetting = FrameSetting::new();
    let mut line_number:ULine = 0;
    let line_number_max:ULine = (lines.len() - 1) as ULine;
    let mut error_count = 0;
    loop{
        let line = lines[line_number].trim();
        let char_array: Vec<char> = line.chars().collect();

        if line_number == line_number_max  || ( lines[line_number].len() >= 3 && &lines[line_number][0..3] == "---" ) {break;}
        if is_empty(line) { line_number += 1; continue; }
        if is_comment(line) {
            // comment(line);
            line_number +=1; continue;
        }
        if is_param(&line) {
            let mut start_value:UColumn = 2;
            if let Option::Some(column_number) = (char_array.iter().position(|&x| x == '=')) {
                start_value = column_number + 1;
            }
            match parameter_syntax_analizator(char_array.clone()) {
                Ok((parameter, value)) => {
                    match setting_up_parameter((parameter.as_str(), value.as_str()), &mut setting) {
                        Result::Err((error_header, error_message)) => {
                            error(String::from_iter(&char_array).as_str(), error_header.as_str(), error_message.as_str(), line_number, start_value);
                        }
                        _ => {
                            // success of parsing.
                        }
                    }
                    // debug("param parsing", format!("{}={}", parameter, value).as_str())
                },
                Err((err_col, err_msg)) => {
                    error_count += 1;
                    error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), 0, err_col);
                }
            }
        }
//        if line_number == line_number_max || (lines[line_number].len() >= 3 && &lines[line_number][0..3] == "---" ) {break;}
        line_number += 1;
    }
    match error_count {
        0 => Ok (setting),
        _ => {
            info(format!("Application finished with error {}-s.", error_count).as_str());
            Err(setting)
        },
    }
}


fn setting_up_parameter<'a, 'b>(param_value: (&'a str, &'a str) ,  setting: &'b mut FrameSetting) -> Result<String, (String, String)> {
    let (param, value) = param_value;
    info(format!("{}={}", param, value).as_str());
    match param {
        "delay"  => {
            match FrameConfiguration::parse_delay(&value) {
                Result::Ok(delay) => {
                    println!("{}", delay.to_string());
                    setting.delay = delay;
                }
                Result::Err(message) => {
                    return Result::Err(("Value parsing error".to_string(), message));
                }
            }
        },
        "offset-y" => {
            match FrameConfiguration::parse_yoffset(&value){
                Result::Ok(offset) => {
                    println!("{}", offset.to_string());
                    setting.yoffset = offset;
                    }
                Result::Err(message) => {
                    return Result::Err(("Value parsing error".to_string(), message));
                }
            }
        },
        "offset-x" => {
            match FrameConfiguration::parse_xoffset(&value) {
                Result::Ok(offset) => {
                    println!("{}", offset.to_string());
                    setting.xoffset = offset;
                }
                Result::Err(message) => {
                    return Result::Err(("Value parsing error".to_string(), message));
                }
            }
        },
        _ => {
            return Result::Err(("Parsing error".to_string(), format!("parameter name '{}' is not defined", param)));
        }
    };
    Result::Ok("Success".to_string())
}


fn parameter_syntax_analizator(line: Vec<char>) -> Result<(String, String), (UColumn, String)> {
    let param = &line[2..];
    if String::from_iter(param).trim().len() == 0 {
        return Result::Err((2, String::from("The parameter is empty, nonetheless param annotation(#:) is")));
    }
    if let Some(pos) = param.iter().position(|&x| x == '=') {
        let param_name: String = String::from_iter(&param[0..pos]).trim().to_string();
        let value: String = String::from_iter(&param[(pos+1)..]).trim().to_string();
        if let Some(size) = param_name.find(' ') {
            return Result::Err((2+size as UColumn, String::from("Not approved space symbol in parameter name")))
        }
        if param_name.len() == 0 {
            return Result::Err((2, String::from("Parameter is empty")));}
        return Result::Ok((param_name, value));
    } else {
        //#:dupa
        let end_line = String::from_iter(param).trim().len();
        return Result::Err((2+end_line as UColumn, String::from("Expected '='(equal) symbol")));
    }
    return Result::Err((0, String::from("Unknown parsing error")));
}


fn create_animation(path: &String) {
    let file: String = fs::read_to_string(path).expect("");
    let lines: Vec<&str> = file.lines().collect();
    if lines.len() == 0 { return }

    let mut line_number = 0;
    let line_number_max = lines.len() - 1;

    let mut animation = Animation::new();
    let mut root_settings = FrameSetting::new();
    match initialize_settings(&lines) {
        Result::Ok(header_setting) => { root_settings = header_setting },
        Result::Err(header_setting) => {
            exit_program();
            root_settings = header_setting
        },
    }
    println!("{}", root_settings.to_string());
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

