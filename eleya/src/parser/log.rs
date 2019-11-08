use crate::tools::cli::vt100::escapes as ESC;
use std::ops::Add;
use crate::parser::log::Log::{SyntaxLog, GeneralLog, SimpleLog, CustomLog};

type TColor = &'static str;
type TTitle = String;

pub struct LogLocation{
    line_from:usize,
    line_to: usize,
    column_from:usize,
    column_to  :usize,
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
    pub fn get(&self, log_stack_id: Option<usize>) -> String{
        match log_stack_id {
            Option::Some(id) => LogStatus::styling(self.status_color(), self.add_id(id).as_str()),
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
    status: LogStatus,
    color: TColor,
    underscore: bool,
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


pub enum Log{
    SyntaxLog {status: LogStatus, message: String, header: String, text: String, style: LogStyle, location: LogLocation},
    GeneralLog{status: LogStatus, message: String, header: String, style: LogStyle},
    SimpleLog {status: LogStatus, message: String, style: LogStyle},
    CustomLog {status: LogStatus, message: String},
}
#[allow(dead_code)]
impl Log{
    pub fn syntax_log(message: &str, header: &str,  text: &str, status: LogStatus, style: LogStyle, location: LogLocation) -> Log{
        SyntaxLog {
            message: message.to_string(),
            header: header.to_string(),
            text: text.to_string(),
            status,
            style,
            location,
        }
    }
    pub fn general_log(message: &str, header: &str, status: LogStatus, style: LogStyle) -> Log{
        GeneralLog {
            message: message.to_string(),
            header: header.to_string(),
            status,
            style,
        }
    }
    pub fn simple_log(message: &str, status: LogStatus, style: LogStyle) -> Log{
        SimpleLog {
            message: message.to_string(),
            status,
            style,
        }
    }
    pub fn custom_log(message: &str, status: LogStatus) -> Log{
        CustomLog {
            message: message.to_string(),
            status,
        }
    }

    fn multiline_view(lines: &Vec<&str>){}
    fn oneline_view(){}

    fn syntax_log_view(){
        println!("{} {} in line {} :\n"
                 , generate_status(ESC::RED, "error", ESC::RESET)
                 , error_header
                 , line_and_column_output(line_number, 1+column_number));
        let offset = (0..column_number).map(|_| " ").collect::<String>();
        println!("{}", line);
        println!("{}^---- {}", offset, error_msg);
        println!();
    }
    fn global_view(){
        println!("{} {} in line {} :\n"
                 , generate_status(ESC::RED, "error", ESC::RESET)
                 , error_header
                 , line_and_column_output(line_number, 1+column_number));
        let offset = (0..column_number).map(|_| " ").collect::<String>();
        println!("{}", line);
        println!("{}^---- {}", offset, error_msg);
        println!();
    }
    #[allow(dead_code)]
    fn generate_status<'b, 'a>(a: &'b str, b: &'a str, c: &'b str) -> String{
        format!("({}{}{})", a, b, c)
    }
}
impl ToString for LogStyle{
    fn to_string(&self) -> String {
        unimplemented!()
    }
}



struct LogTrace{
    logs: Vec<Log>,
    id: String,
}
impl LogTrace {
    pub fn generate_id()->String{
        let rand_key = thread_rng().sample_iter(&Alphanumeric).take(30).collect();
        rand_key
    }
}

struct MessageList{
    list: LogTrace,
}
impl MessageList{
    pub fn get_all(&self){
        for log_trace in self.list.iter(){
            for log in log_trace.logs.iter() {
                pritnln("{}", log.to_string());
            }
        }
    }
}



