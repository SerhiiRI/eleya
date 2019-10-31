//pub enum FrameAvailableConfiguration{
//    Delay{delay: u32},
//    XOffset(u8),
//    YOffset(u8),
//}
//pub struct FrameConfiguration(FrameAvailableConfiguration);
//impl FrameConfiguration{
//    pub fn x_offset(offset: u8){ FrameConfiguration(FrameAvailableConfiguration::XOffset(0)); }
//    pub fn y_offset(offset: u8){ FrameConfiguration(FrameAvailableConfiguration::YOffset(0)); }
//    pub fn delay(time_scale: &str, delay: u32) -> FrameConfiguration{
//        if delay < 0 {
//            return FrameConfiguration(FrameAvailableConfiguration::Delay {delay: 0});
//        }
//        match time_scale {
//            "s"  => FrameConfiguration(FrameAvailableConfiguration::Delay {delay: delay * 1000}),
//            "ms" => FrameConfiguration(FrameAvailableConfiguration::Delay {delay: delay}),
//            _    => FrameConfiguration(FrameAvailableConfiguration::Delay {delay: delay}),
//        }
//    }
//}


use std::iter::FromIterator;

#[derive(Copy, Clone)]
pub enum FrameConfiguration{
    Delay{delay: u32},
    XOffset(u8),
    YOffset(u8),
}
impl FrameConfiguration{
    pub fn x_offset(offset: u8) { FrameConfiguration::XOffset(0); }
    pub fn y_offset(offset: u8) { FrameConfiguration::YOffset(0); }

    pub fn delay(time_scale: &str, delay: u32) -> FrameConfiguration {
        if delay < 0 {
            return FrameConfiguration::Delay { delay: 0 }
        }
        match time_scale {
            "s" => FrameConfiguration::Delay { delay: delay * 1000 },
            "ms" => FrameConfiguration::Delay { delay },
            _ => FrameConfiguration::Delay { delay }
        }
    }
    pub fn parse_delay(value:&str) -> Option<FrameConfiguration> {
        let unparsed_value:Vec<char> = value.chars().collect();
        let numeral_part:&Vec<char> = &unparsed_value.clone().into_iter().filter(|&c| c.is_digit(10)).collect();
        let string_part:&Vec<char> = &unparsed_value.clone().into_iter().filter(|&c| !c.is_digit(10)).collect();
        if let Result::Ok(value) = String::from_iter(numeral_part).parse::<u32>(){
            match String::from_iter(string_part).as_str() {
                "s"   => return Option::Some(FrameConfiguration::Delay { delay: value * 1000 }),
                "ms"  => return Option::Some(FrameConfiguration::Delay { delay: value }),
                ""    => return Option::Some(FrameConfiguration::Delay { delay: value }),
                _     => return Option::None,
            }
            return Option::None;
        }
        Option::None
    }
    pub fn print(&self){
        match self {
            FrameConfiguration::Delay { delay: delay_time } => { println!("Delay time {}", delay_time) },
            FrameConfiguration::XOffset(offset) => { println!("X-Offset {}", offset)},
            FrameConfiguration::YOffset(offset) => { println!("Y-Offset {}", offset); },
        }
    }
}



pub struct Frame {
    pub frame:  Vec<String>,
    pub config: Vec<FrameConfiguration>,
}
impl Frame {
    pub fn new() -> Frame{
        Frame{ frame: Vec::new(), config: Vec::new() }
    }
    pub fn print(&self){
        println!("______ Frame ______");
        println!("Frame: ");
        for config in self.config.iter(){
            config.print();
        }
        for string in self.frame.iter(){
            println!("{}", string);
        }
        println!("_______ END _______");
    }
}


pub struct Animation{ pub frames: Vec<Frame> }
impl Animation {
    pub fn new() -> Animation{
        Animation{ frames: Vec::new() }
    }
}


pub fn initialize_global_configuration() -> Vec<FrameConfiguration>{
    let mut
    configuration:Vec<FrameConfiguration> = Vec::new();
    configuration.push(FrameConfiguration::YOffset(0));
    configuration.push(FrameConfiguration::XOffset(0));
    configuration.push(FrameConfiguration::Delay{delay:100});
    configuration
}

