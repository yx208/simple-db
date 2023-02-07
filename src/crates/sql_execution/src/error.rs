use miette::Diagnostic;
use thiserror::Error;
use sql_parser::error::FormattedError;

/// 查询执行期间的错误
#[derive(Error, Debug, Diagnostic)]
#[error("Query Exception; Error")]
pub enum QueryExceptionError {
    #[error("Table ${0} was not found")]
    TableNotFound(String),

    #[error("Table {0} already exists")]
    TableAlreadyExists(String),

    #[error("Column ${0} does not exist")]
    ColumnDoesNotExists(String)
}

/// Errors at any point in the SQL "pipeline"
#[derive(Error, Debug, Diagnostic)]
#[error(transparent)]
pub enum SQLError<'a> {
    #[diagnostic(transparent)]
    QueryExceptionError(#[from] QueryExceptionError),

    #[diagnostic(transparent)]
    ParsingError(FormattedError<'a>)
}

/// 需要一个手动实现，因为错误 `#[from]` 似乎对生命周期很悲伤
impl<'a> From<FormattedError<'a>> for SQLError<'a> {
    fn from(value: FormattedError<'a>) -> Self {
        SQLError::ParsingError(value)
    }
}
