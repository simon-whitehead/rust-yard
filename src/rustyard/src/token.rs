#[derive(Clone)]
pub enum Token {
    Operator(char, u32),
    WholeNumber(i64),
    DecimalNumber(f64),
    FunctionCall(String, Vec<String>)
}
