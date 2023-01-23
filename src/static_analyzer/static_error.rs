#[derive(Debug)]
pub enum StaticError {
    NonExistingVariable,
    NonExistingFunction,
    ReDeclaringVariable,
    DuplicatesFunctionArgs,
    InvalidArgumentsCount,
}
