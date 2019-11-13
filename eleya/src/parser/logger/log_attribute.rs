use std::ops::Add;

type TColor = &'static str;
type TTitle = String;

pub struct LogLocation{
    pub line_from:usize,
    pub line_to: usize,
    pub column_from:usize,
    pub column_to  :usize,
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