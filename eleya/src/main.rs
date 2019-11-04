mod parser;
mod tools;

use std::{env, str, fs, process::exit};
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration, FrameSetting};

use crate::tools::cli::output::{error, info, error_line};
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
        _ => {&line[0..1] == COMMENT},
//        _ => {&line[0..2] != PARAM},
    }
}
fn is_param(line: &str) -> bool{
    match line.len() {
        0...1 => {false},
        _     => {&line[0..2] == PARAM},
    }
}
fn is_frame(line: &str) -> bool{
    match line.len() {
        0...2 => {false},
        _     => {&line[0..3] == FRAME},
    }
}
fn initialize_animation_body(lines: &Vec<&str>, root_setting: &mut FrameSetting ) -> Result<Animation, String> {
//    let mut error_count = 0;
    if let Option::Some(position) = lines.iter().position(|&line| is_frame(line.trim())) {
        let mut animation: Animation    = Animation    ::new();
        let mut line_number:ULine = position;
        let line_number_max:ULine = (lines.len() - 1) as ULine;

        loop{
            if line_number == line_number_max {break;}
            if is_frame(lines[line_number]){
                line_number += 1;
                let mut frame : Frame = Frame::new();
                let mut frame_strings:Vec<_> = Vec::new();
                loop {
                    if line_number == line_number_max || (lines[line_number].len() >= 3 && &lines[line_number][0..3] == FRAME ) {break;}

                    if is_param(lines[line_number]) {
                        let mut start_value:UColumn = 2;
                        if let Option::Some(column_number) = lines[line_number].chars().collect::<Vec<char>>().iter().position(|&x| x == '=') {
                            start_value = column_number + 1;
                        }
                        match parameter_syntax_analizator(lines[line_number].chars().collect::<Vec<char>>().clone()) {
                            Ok((parameter, value)) => {
                                match setting_up_parameter((parameter.as_str(), value.as_str()), root_setting) {
                                    Result::Err((error_header, error_message)) => {
//                                        error_count += 1;
                                        error(lines[line_number], error_header.as_str(), error_message.as_str(), line_number, start_value);
                                        return Result::Err("Please verify parameter name or correct param value form".to_string())
                                    }
                                    _ => {}
                                }
                            },
                            Err((err_col, err_msg)) => {
//                                error_count += 1;
                                error(lines[line_number], "Parsing error", err_msg.as_str(), 0, err_col);
                                return Result::Err("Param syntax is not allowed".to_string())
                            }
                        }
                        line_number+=1;
                        continue;
                    }
                    if is_comment(lines[line_number]){line_number +=1;continue;}
                    frame_strings.push(lines[line_number].to_string());
                    line_number+=1;
                }
                if frame_strings.len() > 0 {
                    frame.config = root_setting.clone();
                    frame.frame  = frame_strings;
                    animation.frames.push(frame);
                }
            }
        }
        return Result::Ok(animation);
    }
    else {

        return Result::Err("No frame found. To create frame draw your frame between '---' delimiters ".to_string());
    }
//    Result::Err("In time frame parsing: undefinied error".to_string())
}

fn initialize_settings(lines: &Vec<&str>) -> Result<FrameSetting, FrameSetting>{
    let mut setting: FrameSetting = FrameSetting::new();
    let mut line_number:ULine = 0;
    let line_number_max:ULine = (lines.len() - 1) as ULine;
    let mut error_count = 0;
    loop{
        let line = lines[line_number].trim();
        let char_array: Vec<char> = line.chars().collect();

        if line_number == line_number_max  || ( lines[line_number].len() >= 3 && &lines[line_number][0..3] == FRAME ) {break;}
        if is_empty(line) {line_number += 1; continue;}

        if is_param(&line) {
            let mut start_value:UColumn = 2;
            if let Option::Some(column_number) = char_array.iter().position(|&x| x == '=') {
                start_value = column_number + 1;
            }
            match parameter_syntax_analizator(char_array.clone()) {
                Ok((parameter, value)) => {
                    match setting_up_parameter((parameter.as_str(), value.as_str()), &mut setting) {
                        Result::Err((error_header, error_message)) => {
                            error(String::from_iter(&char_array).as_str(), error_header.as_str(), error_message.as_str(), line_number, start_value);
                        }
                        _ => {}
                    }
                    // debug("param parsing", format!("{}={}", parameter, value).as_str())
                },
                Err((err_col, err_msg)) => {
                    error_count += 1;
                    error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), line_number, err_col);
                }
            }
            line_number +=1; continue;
        }
        if is_comment(line) {line_number +=1; continue;}
        error_count += 1;
        error(&line, "Undefined syntax", "the keyword is not known", line_number, 0);
//        if line_number == line_number_max || (lines[line_number].len() >= 3 && &lines[line_number][0..3] == "---" ) {break;}
        line_number += 1;
    }
    match error_count {
        0 => Ok (setting),
        _ => {
            info(format!("Application finished with {} error(-s)", error_count).as_str());
            Err(setting)
        },
    }
}


fn setting_up_parameter<'a, 'b>(param_value: (&'a str, &'a str) ,  setting: &'b mut FrameSetting) -> Result<String, (String, String)> {
    let (param, value) = param_value;
//    info(format!("{}={}", param, value).as_str());
    match param {
        "delay"  => {
            match FrameConfiguration::parse_delay(&value) {
                Result::Ok(delay) => {
//                    println!("{}", delay.to_string());
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
//                    println!("{}", offset.to_string());
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
//                    println!("{}", offset.to_string());
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
        let end_line = String::from_iter(param).trim().len();
        return Result::Err((2+end_line as UColumn, String::from("Expected '='(equal) symbol")));
    }
//    return Result::Err((0, String::from("Unknown parsing error")));
}

fn create_animation(path: &String) {
    let file: String = fs::read_to_string(path).expect("");
    let lines: Vec<&str> = file.lines().collect();
    if lines.len() == 0 { return }
    let mut root_settings = FrameSetting::new();
    match initialize_settings(&lines) {
        Result::Ok(header_setting) => { root_settings = header_setting },
        Result::Err(e) => {
            exit_program();
        },
    }
    match initialize_animation_body(&lines, &mut root_settings) {
        Ok(animation) => {
            for frm in animation.frames.iter(){
                println!("{}", frm.to_string());
            }
        },
        Err(message) => {
            error_line(message.as_str());
            exit_program()
        },
    }
    info(format!("{}", root_settings.to_string()).as_str());
}

fn exit_program(){
    println!("Have a nice day)");
    exit(0);
}

fn main() {
    let cli_arguments: Vec<String> = env::args().collect();
    let animation_file_path:Option<&String> = cli_arguments.get(1);
    match animation_file_path {
        Option::Some(path) => create_animation(path),
        Option::None => exit_program(),
    }
    info(format!("Input arguments {:?}", cli_arguments).as_str());

}

