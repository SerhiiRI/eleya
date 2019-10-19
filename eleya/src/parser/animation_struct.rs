pub enum AnimationConfiguration{
    Delay{delay: u32},
    XOffset(u8),
    YOffset(u8),
}
impl AnimationConfiguration{
    pub fn delay(time_scale: &str, delay: u32) -> AnimationConfiguration{
        if delay < 0 {
            return AnimationConfiguration::Delay {delay: 0};
        }
        match time_scale {
            "s"  => AnimationConfiguration::Delay {delay: delay * 1000},
            "ms" => AnimationConfiguration::Delay {delay: delay},
            _    => AnimationConfiguration::Delay {delay: delay},
        }
    }

    pub fn x_offset(offset: u8){
        AnimationConfiguration::XOffset{0};
    }

    pub fn y_offset(offset: u8){
        AnimationConfiguration::YOffset{0};
    }
}



pub struct Frame {
    pub frame:  Vec<String>,
    pub config: Vec<AnimationConfiguration>,
}



pub struct Animation{ pub frames: Vec<Frame> }
impl Animation {
    pub const fn new() -> Animation{
        Animation{ frames: Vec::new() }
    }
}
