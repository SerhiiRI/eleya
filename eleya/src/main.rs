
use std::{env, str, fs, process::exit, str::Lines};
use crate::parser::animation_struct::{Animation, AnimationConfiguration, Frame};
mod parser;

static mut AnimationStack:Animation = Animation::new();

fn create_simple_data_struct(){
    println!("saulda");
}

fn create_animation(path: &String){
    println!("Animation File:{} ", path);

    let file:String = fs::read_to_string(path).expect("");
    let lines:Vec<&str> = file.lines().collect();

    if  lines.len() == 0 {return}

    for line in lines {

        let char_array: Vec<char> = line.chars().collect();
        if char_array.len() > 0 && char_array[0] == '#' {
            println!("Komment " );
        }
        else {
            println!("Line => {}", line);
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
    match animation_file_path {
        Option::Some(path) => create_animation(path),
        Option::None => exit_program(),
    }


    println!("Input arguments {:?}", cli_arguments);
}
