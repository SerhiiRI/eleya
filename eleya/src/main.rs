
use std::{env, str, fs, process::exit, str::Lines};
use crate::parser::animation_struct::{Animation, Frame, FrameConfiguration};
use std::iter::FromIterator;
use std::borrow::Borrow;
use syntax::util::map_in_place::MapInPlace;

mod parser;

//static mut ANIMATION_STACK:Animation = Animation::new();

fn clean_up_string(string: &String) -> &str{
    string.trim()
}

fn parse_parameter(line: Vec<char>) -> (String, String) {
    let param = line[2..];
    if String::from_iter(param).trim().len() == 0 { return Option::None }
    if param.contains(&'=') {
        let position_of_eq = Iterator::position(param, |&x| x == '=');
        if let Some(pos) = position_of_eq{
            let param_name:&str = clean_up_string(&String::from_iter(param[1..pos]));
            let value:&str = clean_up_string(( &String::from_iter(param[pos..])));
            // TODO: return param with value

        } else {
            return ( String::from(" - "), String::from(" - "));
        }

    } else {
        return (String::from("dupa"), String::from("chuj"));
    }

    return (String::from("dupa"), String::from("chuj"));
}

fn create_animation(path: &String){
    println!("Animation File:{} ", path);

    let file:String = fs::read_to_string(path).expect("");
    let lines:Vec<&str> = file.lines().collect();


    if  lines.len() == 0 {return}

    for line in lines {
        let char_array: Vec<char> = line.chars().collect();
        if char_array.len() == 0 {
            continue;
        }


        match String::from_iter(&char_array[0..2]).as_str() {
            "#:" =>
            _ =>
        }

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
//    match animation_file_path {
//        Option::Some(path) => create_animation(path),
//        Option::None => exit_program(),
//    }


    println!("Input arguments {:?}", cli_arguments);

}
