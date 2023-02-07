use nom::{
    bytes::complete::tag_no_case,
    character::complete::multispace1,
    error::context,
    sequence::{preceded, tuple}
};
use nom_supreme::ParserExt;
use serde::{Serialize, Deserialize};
use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<String>
}

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (remaining_input, (_, _, table, _, values)) = context(
            "Insert statement",
            tuple((
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, identifier.context("Table Name")),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, comma_sep(identifier).context("Values"))
            ))
        )(input)?;

        Ok((remaining_input, InsertStatement { table, values }))
    }
}
