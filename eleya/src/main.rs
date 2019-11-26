mod parser;
mod tools;
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration, FrameSetting};
use crate::parser::logger::log_attribute as LogAttribute;
use crate::parser::logger::log_stack as Logger;


use std::{env, str, fs, process::exit};
use std::iter::FromIterator;
use std::ops::Add;
//use rand::{thread_rng, Rng};
//use rand::distributions::Alphanumeric;
use rand::prelude as prelude;


use crate::parser::logger::log_stack::MessageList;
use crate::parser::logger::log_attribute::{LogStatus, LogStyle};


type UColumn = usize;
type ULine   = usize;

const COMMENT: &str  = "#";
const PARAM:   &str  = "#:";
const FRAME:   &str  = "---";
static mut MESSAGE_PIPE:MessageList = MessageList::new();
//static mut MESSAGE_PIPE:MessageList = MessageList::new();

fn is_empty(line: &str) -> bool{
    line.to_string().trim().len() == 0
}
fn is_comment(line: &str) -> bool{
    match line.len() {
        0 => {true},
        1 => {&line[0..1] == COMMENT},
	    _ => {&line[0..1] == COMMENT && &line[0..1] != PARAM},
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

fn initialize_animation_body(lines: &Vec<&str>, root_setting: &mut FrameSetting ) -> Result<Animation, LogAttribute::LogStatus> {
    if let Option::Some(position) = lines.iter().position(|&line| is_frame(line.trim())) {
        let mut animation:Animation = Animation::new();
        let mut line_number:ULine = position;
        let line_number_max:ULine = (lines.len() - 1) as ULine;
        loop{
            if line_number == line_number_max {break;}
            if is_frame(lines[line_number]){
                line_number += 1;
                let mut frame : Frame = Frame::new();
                let mut frame_strings:Vec<_> = Vec::new();
                loop {
                    if line_number == line_number_max || (lines[line_number].len() >= 3 && lines[line_number][0..3] == FRAME ) {break;}

                    if is_param(lines[line_number]) {
                        let mut start_value:UColumn = 2;
                        if let Option::Some(column_number) = lines[line_number].chars().collect::<Vec<char>>().iter().position(|&x| x == '=') {
                            start_value = column_number + 1;
                        }
                        match parameter_syntax_analizator(lines[line_number]) {
                            Ok((parameter, value)) => {
                                match setting_up_parameter((parameter.as_str(), value.as_str()), log_status) {

                                    Result::Err((error_header, error_message)) => {
//                                        error_count += 1;
                                        error(lines[line_number], error_header.as_str(), error_message.as_str(), line_number, start_value);
//                                        return Result::Err("Please verify parameter name or correct param value form".to_string())
                                        return Result::Err(LogAttribute::LogStatus::Error);
                                    }
                                    _ => {}
                                }
                            },
                            Err(log_status) => {
                                error(lines[line_number], "Parsing error", err_msg.as_str(), 0, err_col);
                                // return Result::Err("Param syntax is not allowed".to_string())
                                return Result::Err(log_status + LogAttribute::LogStatus::Error);
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
//        return Result::Err("No frame found. To create frame draw your frame between '---' delimiters ".to_string());
        return Result::Err(LogAttribute::LogStatus::Error);
    }
}

fn initialize_settings(lines: &Vec<&str>) -> Result<FrameSetting, LogAttribute::LogStatus>{
    let mut setting: FrameSetting = FrameSetting::new();
    let mut line_number:ULine = 0;
    let line_number_max:ULine = (lines.len() - 1) as ULine;
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
            match parameter_syntax_analizator(line, line_number) {
                Ok((parameter, value)) => {
                    match setting_up_parameter((parameter.as_str(), value.as_str()), &mut setting) {
                        Result::Err((error_header, error_message)) => {
                            error(String::from_iter(&char_array).as_str(), error_header.as_str(), error_message.as_str(), line_number, start_value);
                        }
                        _ => {}
                    }
                    // debug("param parsing", format!("{}={}", parameter, value).as_str())
                },
                Err(log_status) => {
                    error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), line_number, err_col);
                }
            }
            line_number +=1; continue;
        }
        if is_comment(line) {line_number +=1; continue;}
        error(&line, "Undefined syntax", "the keyword is not known", line_number, 0);
//        if line_number == line_number_max || (lines[line_number].len() >= 3 && &lines[line_number][0..3] == "---" ) {break;}
        line_number += 1;
    }
    match error_count {
        0 => Ok (setting),
        _ => {
            info(format!("Application finished with {} error(-s)", error_count).as_str());
            Err(LogAttribute::LogStatus::Error)
        },
    }
}


fn setting_up_parameter<'a, 'b>(param_value: (&'a str, &'a str) ,  setting: &'b mut FrameSetting) -> Result<String, LogAttribute::LogStatus> {
    let (param, value) = param_value;
    match param {
        "delay"  => {
            match FrameConfiguration::parse_delay(&value) {
                Result::Ok(delay) => {
                    setting.delay = delay;
                }
                Result::Err(message) => {
                    let e:Logger::Log::GeneralLog = Logger::Log::general_log(message, "Value parsing error", LogStatus, LogAttribute::LogStyle::default());
                    MESSAGE_PIPE.push(e);
                    return Result::Err(LogAttribute::LogStatus::Error);
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
//                    return Result::Err(("Value parsing error".to_string(), message));
                    let e:Logger::Log::GeneralLog = Logger::Log::general_log(message, "Value parsing error", LogStatus, LogAttribute::LogStyle::default());
                    MESSAGE_PIPE.push(e);
                    return Result::Err(LogAttribute::LogStatus::Error);
                }
            }
        },
        "offset-x" => {
            match FrameConfiguration::parse_xoffset(&value) {
                Result::Ok(offset) => {
                    setting.xoffset = offset;
                }
                Result::Err(message) => {
//                    return Result::Err(("Value parsing error".to_string(), message));
                    let e:Logger::Log::GeneralLog = Logger::Log::general_log(message, "Value parsing error", LogStatus, LogAttribute::LogStyle::default());
                    MESSAGE_PIPE.push(e);
                    return Result::Err(LogAttribute::LogStatus::Error);
                }
            }
        },
        _ => {
//            return Result::Err(("Parsing error".to_string(), format!("parameter name '{}' is not defined", param)));
            return Result::Err(LogAttribute::LogStatus::Error);
        }
    };
    Result::Ok("Success".to_string())
}

#[allow(dead_code)]
fn column_in_line(line: &str, searched_text: &str) -> (UColumn, UColumn) {
    match line.find(searched_text) {
        Option::Some(value) => (value, value + searched_text.len()),
        Option::None => {
            let line_trim = line.len();
            match line.find(line_trim){
                Option::Some(value) => (value, value+line_trim),
                Option::None => (0,0),
            }
        }
    }
}

fn parameter_syntax_analizator(line_str: &str) -> Result<(String, String), LogStatus> {
    let line:Vec<char> = line_str.trim().chars().collect::<Vec<char>>();
    let param = &line[2..];
    if String::from_iter(param).trim().len() == 0 {
//        return Result::Err((2, String::from("The parameter is empty, nonetheless param annotation(#:) is")));
        return Result::Err(LogAttribute::LogStatus::Error);
    }
    if let Some(pos) = param.iter().position(|&x| x == '=') {
        let param_name: String = String::from_iter(&param[0..pos]).trim().to_string();
        let value: String = String::from_iter(&param[(pos+1)..]).trim().to_string();
        if let Some(size) = param_name.find(' ') {
//            return Result::Err((2+size as UColumn, String::from("Not approved space symbol in parameter name")))
            return Result::Err(LogAttribute::LogStatus::Error)
        }
        if param_name.len() == 0 {
//            return Result::Err((2, String::from("Parameter is empty")));}
            return Result::Err(LogAttribute::LogStatus::Error)
        }
        return Result::Ok((param_name, value));
    } else {
        let end_line = String::from_iter(param).trim().len();
        let llog = Logger::Log::general_log(
            "Expected '='(equal) symbol".to_string(),
            "parsing error".to_string(),
            LogAttribute::LogStatus::Error,
            LogStyle::default());
        return Result::Err(LogAttribute::LogStatus::Error);
    }
}

fn create_animation(path: &String) {

    let file: String = fs::read_to_string(path).expect("");
    let lines: Vec<&str> = file.lines().collect();
    if lines.len() == 0 { return }
    let mut root_settings = FrameSetting::new();


    match initialize_settings(&lines) {
        Result::Ok(header_setting) => { root_settings = header_setting },
        Result::Err(log_status) => {
            match log_status:LogAttribute::LogStatus {
                LogAttribute::LogStatus::Error => exit_program(),
                _ => exit_program()
            }
        },
    }
    match initialize_animation_body(&lines, &mut root_settings) {
        Ok(animation) => {
            for frm in animation.frames.iter(){
                println!("{}", frm.to_string());
            }
        },
        Err(log_status) => {
            match log_status: LogAttribute::LogStatus {
                LogAttribute::LogStatus::Error => exit_program(),
                _ => exit_program()
            }
        },
    }

    // TEST: print finished settings
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_empty(){
        assert_eq!(true, is_empty("     "));
        assert_eq!(false, is_empty(" _ "));
    }

    #[test]
    fn test_is_param(){
        assert_eq!(true, is_param("  #:dupa"));
        assert_eq!(true, is_param("#:"));
        assert_eq!(true, is_param("#:bliat=suka"));
    }

    #[test]
    fn test_is_comment(){
        assert_eq!(false, is_param("#:bliat=suka"));
        assert_eq!(true, is_param("#"));
        assert_eq!(true, is_param(""));
        assert_eq!(true, is_param("#d"));
        assert_eq!(true, is_param("# Comment"));
    }

    #[test]
    fn test_is_frame(){
        assert_eq!(false, is_param("--"));
        assert_eq!(true, is_param("---"));
        assert_eq!(true, is_param("  ---  "));
        assert_eq!(true, is_param("------  "));
    }

    #[test]
    #[should_panic]
    fn test_param_parser(){
        if let Result::Err((_, _)) = parameter_syntax_analizator("#:sukin=syn"){
            panic!("unparsed param");
        }
        if let Result::Err((_, _)) = parameter_syntax_analizator("   #:sukin=syn"){
            panic!("unparsed param with space");
        }
        if let Result::Err((_, _)) = parameter_syntax_analizator("#: sukin = syn"){
            panic!("unparsed param with space between word");
        }
        if let Result::Err((_, _)) = parameter_syntax_analizator("#:sukin="){
            panic!("unparsed param with no value");
        }
    }

}