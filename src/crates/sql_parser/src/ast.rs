use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::map,
    error::context,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};
use crate::commands::{
    CreateStatement,
    SelectStatement,
    InsertStatement
};
use crate::error::FormattedError;
use crate::parse::{Parse, ParseResult, peek_then_cut, RawSpan};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Create(CreateStatement),
    Insert(InsertStatement),
    Select(SelectStatement),
}

impl<'a> Parse<'a> for SqlQuery {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (rest, (query, _, _, _)) = context(
            "Query",
            preceded(
                multispace0,
                tuple((
                    alt((
                        peek_then_cut("select", map(SelectStatement::parse, |s| {
                            // 与下面写法等价
                            SqlQuery::Select(s)
                        })),
                        peek_then_cut("create", map(CreateStatement::parse, SqlQuery::Create)),
                        peek_then_cut("insert", map(InsertStatement::parse, SqlQuery::Insert))
                    )),
                    multispace0,
                    char(';'),
                    multispace0
                ))
            )
        )(input)?;

        Ok((rest, query))
    }
}

impl<'a> TryFrom<&'a str> for SqlQuery {
    type Error = FormattedError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match SqlQuery::parse_format_error(value) {
            Ok(query) => Ok(query),
            Err(err) => Err(err)
        }
    }
}

pub fn parse_sql_query(input: &str) -> Result<SqlQuery, FormattedError<'_>> {
    input.try_into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let query = SqlQuery::parse_from_raw("select fart;");
        assert!(query.is_err(), "expected parse to fail, got {query:?}");
    }

    #[test]
    fn test_select() {
        let expected = SelectStatement {
            table: "t1".to_string(),
            fields: vec!["foo".to_string(), "bar".to_string()]
        };
        assert_eq!(
            SqlQuery::parse_from_raw("select foo, bar from t1;").unwrap().1,
            SqlQuery::Select(expected)
        );
    }

}
