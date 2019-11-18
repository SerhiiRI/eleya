use std::ops::Add;
use crate::column_in_line;

type TColor = &'static str;
type TTitle = String;

pub struct LogLocation{
    pub lines: usize,
    pub columns: (usize, usize),
}
impl LogLocation {
    fn column_in_line(line: &str, searched_text: &str) -> Option<(usize, usize)> {
        match line.find(searched_text) {
            Option::Some(value) => Option::Some((value, value + searched_text.len())),
            Option::None => {
                let line_trim = line.len();
                match line.find(line_trim){
                    Option::Some(value) => Option::Some((value, value+line_trim)),
                    Option::None => Option::None,
                }
            }
        }
    }
    fn test_lines(line: &str) -> Option<(usize, usize)> {
        let trim_line = line.trim();
        if trim_line == "" {
            return Option::None;
        }
        LogLocation::column_in_line(&trim_line, line)
    }
    pub fn lines_of_problem(text: &Vec<&str>) -> Vec<LogLocation>{
        let mut locations = Vec::<LogLocation>::new();
        for (indx, line) in text.iter().enumerate() {
            if let Option::Some(columns) = LogLocation::test_lines(line){
                locations.push(LogLocation { lines: indx, columns })
            }
        }
        locations
    }
    pub fn line_of_problem(error_region: &str, text: &str) -> Vec<LogLocation>{
        let mut log_location  = LogLocation::new();
        if let Option::Some(columns) = LogLocation::column_in_line(error_region, text){
            locations.push(LogLocation { lines: indx, columns })
        }
        log_location
    }
    pub fn new() -> Vec<LogLocation> {
        Vec::<LogLocation>::new()
    }

}

pub enum LogStatus{
    Error,
    Info,
    Warning,
    Debug,
    Other(TTitle, &'static str),
    None
}
impl LogStatus{
    fn add_id(&self, log_stack_id: usize) -> String{
        format!("{}: {}", self.status_name(), log_stack_id)
    }
    fn styling(color: TColor, text_in: &str) -> String{
        format!("({}{}{})", color, text_in, ESC::RESET)
    }
    pub fn status_name(&self) -> String{
        match self {
            LogStatus::None => "none".to_string(),
            LogStatus::Info => "info".to_string(),
            LogStatus::Warning => "warning".to_string(),
            LogStatus::Debug => "debug".to_string(),
            LogStatus::Error => "error".to_string(),
            LogStatus::Other(_, _) => "custom".to_string(),
        }
    }
    pub fn status_color(&self) -> TColor{
        match self {
            LogStatus::None => ESC::WHITE,
            LogStatus::Info => ESC::LIGHT_BLUE,
            LogStatus::Warning => ESC::YELLOW,
            LogStatus::Debug => ESC::GREEN,
            LogStatus::Error => ESC::LIGHT_RED,
            LogStatus::Other(_, color) => color,
        }
    }
    pub fn get(&self, log_stack_id: &Option<usize>) -> String{
        match log_stack_id {
            Option::Some(id) => LogStatus::styling(self.status_color(), self.add_id(id.clone()).as_str()),
            Option::None => LogStatus::styling(self.status_color(), self.status_name().as_str()),
        }
    }
}
impl ToString for LogStatus{
    fn to_string(&self) -> String {
        match self {
            LogStatus::None => "".to_string(),
            LogStatus::Info => LogStatus::styling(ESC::LIGHT_BLUE, "info"),
            LogStatus::Warning => LogStatus::styling(ESC::YELLOW, "warning"),
            LogStatus::Debug => LogStatus::styling(ESC::GREEN, "debug"),
            LogStatus::Error => LogStatus::styling(ESC::LIGHT_RED, "error"),
            LogStatus::Other(title, color) => LogStatus::styling(color, title.trim()),
        }
    }
}
impl Add<LogStatus> for LogStatus {
    type Output = LogStatus;
    fn add(self, _rhs: LogStatus) -> LogStatus {
        match self {
            LogStatus::Error => LogStatus::Error,
            _ => _rhs,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LogStyle{
    pub status: LogStatus,
    pub color: TColor,
    pub underscore: bool,
}
impl LogStyle{
    pub fn default() -> LogStyle{
        LogStyle{
            status: LogStatus::None,
            color: ESC::WHITE,
            underscore: false
        }
    }
    pub fn error() -> LogStyle{
        LogStyle {status: LogStatus::Error, ..LogStyle::default()}
    }
    pub fn info() -> LogStyle{
        LogStyle {status: LogStatus::Info, ..LogStyle::default()}
    }
    pub fn debug() -> LogStyle{
        LogStyle {status: LogStatus::Debug, ..LogStyle::default()}
    }
    pub fn warning() -> LogStyle{
        LogStyle {status: LogStatus::Warning, ..LogStyle::default()}
    }
    pub fn custom(status: LogStatus, color: &'static str, underscore: bool) -> LogStyle {
        LogStyle{status,color,underscore}
    }
}
impl ToString for LogStyle{
    fn to_string(&self) -> String {
        format!("<LogStyle:type={},underscore={},color={}>", self.status_name() , self.underscore.to_string(), self.color)
    }
}