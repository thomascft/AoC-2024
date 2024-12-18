use miette::Diagnostic;
use thiserror::Error;
use winnow::error::{ContextError, ParseError};

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse input\n{}", message)]
    #[diagnostic(code(aoc::parse_error))]
    ParseError {
        message: String,
        #[source_code]
        input: String,
        #[label("Here")]
        err_span: std::ops::Range<usize>,
    },
}

impl From<ParseError<&str, ContextError>> for AocError {
    fn from(err: ParseError<&str, ContextError>) -> Self {
        let input = err.input();
        let message = err.inner().to_string();
        let start = err.offset();
        let end = (start + 1..)
            .find(|e| input.is_char_boundary(*e))
            .unwrap_or(start);

        Self::ParseError {
            input: input.to_string(),
            message,
            err_span: start..end,
        }
    }
}
