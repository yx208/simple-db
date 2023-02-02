use nom::character::streaming::multispace0;
use nom::error::context;
use nom::sequence::tuple;
use crate::parse::{identifier, Parse, ParseResult, RawSpan};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct InsertStatement;

impl<'a> Parse<'a> for InsertStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        let (res, _) = context(
            "Insert statement",
            tuple((multispace0, identifier))
        )(input)?;
        Ok((res, InsertStatement {}))
    }
}
