use nom::{
    sequence::{ tuple },
    bytes::complete::tag_no_case,
    character::complete::multispace1,
    error::context,
};
use nom_supreme::ParserExt;
use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>
}

impl<'a> Parse<'a> for SelectStatement {
    /// `SELECT col1, col2 FROM foo`
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, fields, _, _, _, table)) = context(
            "Select Statement",
            tuple((
                tag_no_case("select"),
                multispace1,
                // 使用逗号分割标识符
                comma_sep(identifier).context("Select Columns"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("From Table")
            ))
        )(input)?;

        Ok((remaining_input, SelectStatement { fields, table }))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_select() {

        let select_statement = SelectStatement {
            table: "t1".into(),
            fields: vec!["foo".into(), "bar".into()]
        };

        assert_eq!(
            select_statement,
            SelectStatement::parse_from_raw("SELECT foo, bar FROM t1").unwrap().1
        );

    }

}
