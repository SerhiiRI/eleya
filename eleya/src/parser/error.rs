pub enum ParserErrorType{
    SyntaxError{line:usize, column:usize, message: String, header: String},
}
