pub enum FrameAvailableConfiguration{
    Delay{delay: u32},
    XOffset(u8),
    YOffset(u8),
}

pub struct FrameConfiguration(FrameAvailableConfiguration);
impl FrameConfiguration{
    pub fn x_offset(offset: u8){ FrameConfiguration(FrameAvailableConfiguration::XOffset(0)); }
    pub fn y_offset(offset: u8){ FrameConfiguration(FrameAvailableConfiguration::YOffset(0)); }
    pub fn delay(time_scale: &str, delay: u32) -> FrameConfiguration{
        if delay < 0 {
            return FrameConfiguration(FrameAvailableConfiguration::Delay {delay: 0});
        }
        match time_scale {
            "s"  => FrameConfiguration(FrameAvailableConfiguration::Delay {delay: delay * 1000}),
            "ms" => FrameConfiguration(FrameAvailableConfiguration::Delay {delay: delay}),
            _    => FrameConfiguration(FrameAvailableConfiguration::Delay {delay: delay}),
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
}


pub struct Animation{ pub frames: Vec<Frame> }
impl Animation {
    pub fn new() -> Animation{
        Animation{ frames: Vec::new() }
    }

}
