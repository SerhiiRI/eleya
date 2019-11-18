use crate::parser::logger::log_attribute::*;

/// # One of possible log
/// the log represent one of message, which
/// would send to user, for resolving problem
/// For those aim Log implement view() function
/// that build string error message
///
/// Log contain `LogStatus` from log_attribute
/// module. Status set which type of log will
/// print to user tty( Error, Info... e.t.s. )
///
/// Log contain `LogStyle` also from log_attribute
/// module. Styling mean color?, highlight? boolean
/// parameter. For coloring `message` or `header`
/// parameter
///
/// Finally `LogLocation` mean two points (row,col),
/// where syntax error start and where it finished.
/// Constructed in that method, because syntax error
/// also maybe one line, char or whole region.
///
pub enum Log{
    SyntaxLog {status: LogStatus, message: String, header: String, text: &Vec<&str>, style: LogStyle, location: Vec<LogLocation>},
    GeneralLog{status: LogStatus, message: String, header: String, style: LogStyle},
    SimpleLog {status: LogStatus, message: String, style: LogStyle},
    CustomLog {status: LogStatus, message: String}
}
#[allow(dead_code)]
impl Log {
    pub fn syntax_log(message: &str, header: &str,  text: &Vec<&str>, status: LogStatus, style: LogStyle, location: Vec<LogLocation>) -> Log{
        Log::SyntaxLog {
            message: message.to_string(),
            header: header.to_string(),
            text,
            status,
            style,
            location,
        }
    }
    pub fn general_log(message: &str, header: &str, status: LogStatus, style: LogStyle) -> Log{
        Log::GeneralLog {
            message: message.to_string(),
            header: header.to_string(),
            status,
            style,
        }
    }
    pub fn simple_log(message: &str, status: LogStatus, style: LogStyle) -> Log{
        Log::SimpleLog {
            message: message.to_string(),
            status,
            style,
        }
    }
    pub fn custom_log(message: &str, status: LogStatus) -> Log{
        Log::CustomLog {
            message: message.to_string(),
            status,
        }
    }
    fn styling(text: &String, style: &LogStyle) -> String{
        format!("{}{}{}", style.color, text, ESC::RESET)
    }
    fn get_columns(line: &str, searched_text: &str) -> (usize, usize) {
        match line.find(searched_text) {
            Option::Some(value) => (value, value + searched_text.len()),
            Option::None => {
                let line_trim = line.len();
                match line.find(line_trim){
                    Option::Some(value) => (value, value+line_trim),
                    Option::None => (0,0),
                }
            }
        }
    }
    fn line_and_column_output(&self) -> String {
        match self {
            Log::SyntaxLog { status, message, header, text, style, location } => {
                match location.len() {
                    0 => "".to_string(),
                    1 => {
                        let default_location = LogLocation{ lines: 0, columns: (0, 0) };
                        let LogLocation { lines:l, columns: (c1,c2) } = &location.get(0).unwrap_or(&default_location);
                        return format!("(L{},C{}:{})", l, c1, c2);
                    }
                    _ => {
                        if let (Option::Some(loc_1), Option::Some(loc_2)) =  (location.first(), location.last()) {
                            return format!("(L{}-{}", loc_1.lines, loc_2.lines);
                        }
                    }
                }
            },
            _ => {}
        }
        "".to_string()
    }
    pub fn view(&self) -> String {
        match self {
            Log::CustomLog { status, message } => {
                format!("{} {}", status.get(Option::None), message)
            }
            Log::SimpleLog { status, message, style } => {
                format!("{} {}", status.get(Option::None), Log::styling(message, style))
            }
            Log::GeneralLog { status, message, header, style } => {
                format!("{} {}:\n{}", status.get(Option::None), Log::styling(header, style), message)
            }
            Log::SyntaxLog { status, message, header, text, style, location } => {
                let lines = text.lines().collect::<Vec<&str>>();
                let mut log_message = format!("{} {} in line {} :\n"
                                      , status.get(Option::None), header,
                                      , self.line_and_column_output());
                for LogLocation{lines: index, columns: (start, end)} in location {
                    let offset =     (0..start)  .map(|_| " ").collect::<String>();
                    let underscore = (start..end).map(|_| "^").collect::<String>();
                    log_message.push_str(format!("{}\n{}{}\n",lines[indx], offset, underscore).as_str());
                }
                log_message.push_str(format!("{}", message).as_str());
                log_message
            }
        }
    }
}
impl ToString for LogStyle{
    fn to_string(&self) -> String {
        unimplemented!()
    }
}



pub struct LogTrace{
    logs: Vec<Log>,
    id: Option<String>,
}
impl LogTrace {
    pub fn generate_id() -> String {
        thread_rng().sample_iter(&Alphanumeric).take(10).collect()
    }
    pub fn generate_id_not_duplicate(message_list: MessageList) -> String {
        let _tmp01 = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
        message_list.list.into_iter().find(
            | item_trace: &LogTrace |
                match item_trace.id {
                    Some(id_string) => &id_string == &_tmp01,
                    _ => false
                }
        );
        thread_rng().sample_iter(&Alphanumeric).take(10).collect()
    }
    pub fn new() -> LogTrace() {
        LogTrace{ logs: Vec::<Log>::new(), id: Option::None }
    }

    pub fn set_id(&mut self, id: String) {
        self.id = Option::Some(id);
    }

    pub fn push(&mut self, log: Log){
        self.logs.push(log);
    }
}

pub struct MessageList{
    list: Vec<LogTrace>,
}
impl MessageList {
    pub fn get_all(&self) {
        for log_trace in self.list.iter() {
            for log in log_trace.logs.iter() {
                pritnln("{}", log.to_string());
            }
        }
    }
    pub fn new() -> MessageList{
        MessageList { list: Vec::<LogTrace>::new() }
    }
    pub fn push(&mut self, trace: LogTrace){
        if !trace.logs.is_empty() {
            self.list.push(trace);
        }
    }
}