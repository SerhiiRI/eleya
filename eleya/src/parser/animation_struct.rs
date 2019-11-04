use std::iter::FromIterator;

#[derive(Copy, Clone)]
pub enum FrameConfiguration{
    Delay(u32),
    XOffset(u8),
    YOffset(u8),
}
#[allow(dead_code)]
impl FrameConfiguration{
    pub fn x_offset(offset: u8) { FrameConfiguration::XOffset(offset); }
    pub fn y_offset(offset: u8) { FrameConfiguration::YOffset(offset); }

    pub fn delay(time_scale: &str, delay: u32) -> FrameConfiguration {
        match time_scale {
            "s" => FrameConfiguration::Delay(delay * 1000),
            "ms" => FrameConfiguration::Delay(delay),
            _ => FrameConfiguration::Delay(delay)
        }
    }
    pub fn parse_yoffset(value:&str) -> Result<FrameConfiguration, String> {
        if let Result::Ok(number_value) = String::from(value).parse::<u8>(){
            return Result::Ok(FrameConfiguration::YOffset(number_value));
        }
        Result::Err("Y offset wrong parameter value".to_string())
    }
    pub fn parse_xoffset(value:&str) -> Result<FrameConfiguration, String> {
        if let Result::Ok(number_value) = String::from(value).parse::<u8>(){
            return Result::Ok(FrameConfiguration::XOffset(number_value));
        }
        Result::Err("X offset wrong parameter value".to_string())
    }
    pub fn parse_delay(value:&str) -> Result<FrameConfiguration, String> {
        let unparsed_value:Vec<char> = value.chars().collect();
        let numeral_part :&Vec<char> = &unparsed_value.clone().into_iter().filter(|&c| c.is_digit(10)).collect();
        let string_part  :&Vec<char> = &unparsed_value.clone().into_iter().filter(|&c| !c.is_digit(10)).collect();
        if let Result::Ok(value) = String::from_iter(numeral_part).parse::<u32>(){
            match String::from_iter(string_part).as_str() {
                "s"   => return Result::Ok(FrameConfiguration::Delay(value * 1000)),
                "ms"  => return Result::Ok(FrameConfiguration::Delay(value)),
                ""    => return Result::Ok(FrameConfiguration::Delay(value)),
                _     => return Result::Err("Error delay time unit".to_string()),
            }
        }
        if numeral_part.len().eq(&0){
            return Result::Err("Empty number value".to_string());
        }
        Result::Err("Error delay parameter value".to_string())
    }

}
impl ToString for FrameConfiguration{
    fn to_string(&self)-> String{
        match self {
            FrameConfiguration::Delay(delay_time) => format!("Delay:{}", delay_time),
            FrameConfiguration::XOffset(offset)    => format!("XOffset:{}", offset),
            FrameConfiguration::YOffset(offset)    => format!("YOffset:{}", offset),
        }
    }
}

/// # Frame Setting
/// is a summary type for key
///
#[derive(Copy, Clone)]
pub struct FrameSetting{
    pub xoffset: FrameConfiguration,
    pub yoffset: FrameConfiguration,
    pub delay  : FrameConfiguration
}
impl FrameSetting {
    pub fn new() -> FrameSetting{
        FrameSetting{
            xoffset: FrameConfiguration::XOffset(0),
            yoffset: FrameConfiguration::YOffset(0),
            delay: FrameConfiguration::Delay(100)
        }
    }
}
impl ToString for FrameSetting{
    fn to_string(&self) -> String {
        format!("{};{};{}", self.delay.to_string(), self.xoffset.to_string(), self.yoffset.to_string())
    }
}


pub struct Frame {
    pub frame:  Vec<String>,
    pub config: FrameSetting,
}
impl Frame {
    pub fn new() -> Frame{
        Frame{ frame: Vec::new(), config: FrameSetting::new() }
    }
}
impl ToString for Frame{
    fn to_string(&self) -> String {
        format!("{}\n{}", self.config.to_string(), self.frame.join("\n"))
    }
}


pub struct Animation{ pub frames: Vec<Frame> }
impl Animation {
    pub fn new() -> Animation{
        Animation{ frames: Vec::new() }
    }
}

