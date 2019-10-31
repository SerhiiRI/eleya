mod parser;
mod tools;

use std::{env, str, fs, process::exit, str::Lines};
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration, FrameSetting};
use crate::tools::cli::vt100::escapes as ESC;
use crate::tools::cli::output::{error, debug, comment};
use std::iter::FromIterator;
use std::borrow::Borrow;

//static mut ANIMATION_STACK:Animation = Animation::new();
type uColumn = u16;
type uLine   = u32;

const COMMENT: &str  = "#";
const PARAM: &str    = "#:";
const FRAME: &str    = "---";


fn clean_up_string(string: String) -> String{
    String::from(string.trim())
}


is_empty(line: &str) -> bool{
    clean_up_string(line.to_string()).len() == 0
}
is_comment(line: &str) -> bool{
    match line.len() {
	0 => true,
	1 => line[0] == &'#',
	_ => line[0..2] != &"#:",
    }
}
is_param(line: &str) -> bool{
    match line.len() {
	0..1 => false,
	_ => line[0..2] == &"#:",
    }
}


fn initialize_settings(lines: &Vec<&str>) -> Result<FrameSetting, FrameSetting>{
    let mut setting: FrameSetting = FrameSetting::new();
    let mut line_number = 0;
    let mut error_count = 0; 
    loop{
	let line = lines[line_number];
        let char_array: Vec<char> = line.chars().collect();
	
        if is_empty(line) { line_number += 1; continue; }
	if is_comment(line) { comment(line); line_number +=1; continue; } 
        if is_param(&line) {
            match parse_parameter_opt(char_array.clone()) {
                Ok((parameter, value)) => {
                    match parameter.as_str() {
                        "delay"    => {
                            if let Some(delay) = FrameConfiguration::parse_delay(&value){
                                setting.delay = delay;
                            }
                        },
                        "offset-y" => {
			    if let Some(offset) = FrameConfiguration::parse_yoffset(&value){
                                setting.yoffset = offset;
                            }
			},
                        "offset-x" => {
			    if let Some(offset) = FrameConfiguration::parse_xoffset(&value){
                                setting.xoffset = offset;
                            }},
                        _          => {}
                    };
                    debug("param parsing", format!("{}={}", parameter, value).as_str())
                },
                Err((err_col, err_msg)) => {
		    error_count += 1;
                    error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), 0, err_col);
                }
            }
        }
    }
    match error_count {
	0 => Ok (setting),
	_ => Err(setting),
    }
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


// fn parse_multiline(



fn create_animation(path: &String){
    let mut animation     = Animation   ::new();
    let mut root_setting = FrameSetting::new();

    
//    let mut frame_global_config: FrameConfiguration = Vec::new();
    
    println!("Animation File:{} ", path);
    let file:String = fs::read_to_string(path).expect("");
    let lines:Vec<&str> = file.lines().collect();

    if  lines.len() == 0 {return}
    let mut line_iterator = lines.iter();
    let mut line_number = 0;
    let line_number_max = lines.len() - 1;
    loop {
        if line_number != line_number_max {
            let line = lines[line_number];
//        if Some(&line) = line_iterator.next() {
            let char_array: Vec<char> = line.chars().collect();
            line.len();
            if char_array.len() == 0 { line_number += 1; continue; }
            if char_array.len() == 1 {
                if let '#' = &char_array[0] {
                    comment(line);
                    line_number += 1;
                    continue
                }
            }
            if char_array.len() >= 2 {
                if let "# " = &(String::from_iter(&char_array)[0..2]) {
                    comment(line);
                    line_number += 1;
                    continue
                }
            }
            if let "#:" = &(String::from_iter(&char_array)[0..2]) {
                match parse_parameter_opt(char_array.clone()) {
                    Ok((parameter, value)) => {
                        match parameter.as_str() {
                            "delay"    => {
                                if let Some(delay) = FrameConfiguration::parse_delay(&value){
                                    default_animation_config.push(delay);
                                }
                            },
                            "offset-y" => {},
                            "offset-x" => {},
                            _          => {}
                        };
                        debug("param parsing", format!("{}={}", parameter, value).as_str())
                    },
                    Err((err_col, err_msg)) => {
                        error(String::from_iter(&char_array).as_str(), "Parsing error", err_msg.as_str(), 0, err_col);
                    }
                }
            }
            if let "---" = &(String::from_iter(&char_array)[0..3]) {
                let mut frame:Frame = Frame::new();
                line_number += 1;
                loop {
                    if line_number == line_number_max
                        || ( lines[line_number].len() >= 3 && &lines[line_number][0..3] == "---" ) {break;}
                    println!("register line of frame ");
                    frame.frame.push(lines[line_number].to_string());
                    line_number += 1;
                }
                frame.config = default_animation_config.to_vec();
                animation.frames.push(frame);
                if line_number == line_number_max {
                    continue;
                }
                if lines[line_number].len() >= 3 && &lines[line_number][0..3] == "---" {
                    line_number -= 1;
                    continue;
                }

            }

            else {
                println!("| {}", line)
            }
            line_number += 1;
        } else {
            break
        }
    }
    println!("FUCK ! {}", &animation.frames.len());
    for i in animation.frames.iter(){
        i.print();
    }
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

