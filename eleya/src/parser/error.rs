use crate::tools::cli::vt100::escapes as ESC;

type TColor = &'static str;
type TTitle = String;


pub struct LogLocation{
    line_from:usize,
    line_to: usize,
    column_from:usize,
    column_to  :usize,
}

enum LogStatus{
    Error,
    Info,
    Warning,
    Debug,
    Other(TTitle, &'static str),
    None
}
impl LogStatus{
    fn styling(a: &str, b: &str) -> String{
        format!("({}{}{})", a, b, ESC::RESET)
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

#[derive(Copy, Clone)]
pub struct LogStyle{
    status: LogStatus,
    color: &'static str,
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
    SyntaxLog{message: String, header: String, style: LogStyle, location: InTextLocation},
    GeneralLog{message: String, header: String, style: LogStyle}
}
