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
    SyntaxLog {status: LogStatus, message: String, header: String, text: String, style: LogStyle, location: LogLocation},
    GeneralLog{status: LogStatus, message: String, header: String, style: LogStyle},
    SimpleLog {status: LogStatus, message: String, style: LogStyle},
    CustomLog {status: LogStatus, message: String}
}
#[allow(dead_code)]
impl Log {
    pub fn syntax_log(message: &str, header: &str,  text: &str, status: LogStatus, style: LogStyle, location: LogLocation) -> Log{
        Log::SyntaxLog {
            message: message.to_string(),
            header: header.to_string(),
            text: text.to_string(),
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
    fn view(&self){
        match self {
            Log::CustomLog { status, message } => {
                format!("{} {}", status.get(Option::None), message);
            }
            Log::SimpleLog { status, message, style } => {
                format!("{} {}{}", status.get(Option::None), Log::styling(message, style));
            }
            Log::GeneralLog { status, message, header, style } => {
                format!("{} {}{}:\n{}", status.get(Option::None), Log::styling(header, style), message)
            }
            Log::SyntaxLog { status, message, header, text, style, location } => {
                let lines = text.lines().collect::<Vec<&str>>();
                let mut _vstr: Vec<String> = Vec::new();
                for l in lines.iter() {
                    if l.trim() != 0 {
                        _vstr.push(l.to_string());
                    }

                    fn multiline_view(lines: &Vec<&str>) {}
                    fn oneline_view() {}

                    fn syntax_log_view() {
                        println!("{} {} in line {} :\n"
                                 , generate_status(ESC::RED, "error", ESC::RESET)
                                 , error_header
                                 , line_and_column_output(line_number, 1 + column_number));
                        let offset = (0..column_number).map(|_| " ").collect::<String>();
                        println!("{}", line);
                        println!("{}^---- {}", offset, error_msg);
                        println!();
                    }

                    fn global_view() {
                        println!("{} {} in line {} :\n"
                                 , generate_status(ESC::RED, "error", ESC::RESET)
                                 , error_header
                                 , line_and_column_output(line_number, 1 + column_number));
                        let offset = (0..column_number).map(|_| " ").collect::<String>();
                        println!("{}", line);
                        println!("{}^---- {}", offset, error_msg);
                        println!();
                    }

                    #[allow(dead_code)]
                    fn generate_status<'b, 'a>(a: &'b str, b: &'a str, c: &'b str) -> String {
                        format!("({}{}{})", a, b, c)
                    }
                }
            }
        }
    }
}
impl ToString for LogStyle{
    fn to_string(&self) -> String {
        unimplemented!()
    }
}



struct LogTrace{
    logs: Vec<Log>,
    id: Option<String>,
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
impl MessageList {
    pub fn get_all(&self) {
        for log_trace in self.list.iter() {
            for log in log_trace.logs.iter() {
                pritnln("{}", log.to_string());
            }
        }
    }
}