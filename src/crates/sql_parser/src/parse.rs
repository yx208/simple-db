use nom::{IResult, combinator::{map, peek}, character::complete::{char, multispace0}, bytes::complete::{take_while1, tag_no_case}, sequence::{tuple, pair}, multi::separated_list1, Finish};
use nom::combinator::all_consuming;
use nom_locate::LocatedSpan;
use nom_supreme::error::ErrorTree;
use crate::error::{format_parse_error, FormattedError};

pub type MyParseError<'a> = ErrorTree<RawSpan<'a>>;
/// 使用 LocatedSpan 作为字符串输入的包装器
pub type RawSpan<'a> = LocatedSpan<&'a str>;
/// 这将使用默认错误类型，但我们将更改后者
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, MyParseError<'a>>;

/// 解析列名和表名的标识符
pub(crate) fn identifier(i: RawSpan) -> ParseResult<String> {
    map(
        take_while1(|c: char| c.is_alphanumeric()),
        |s: RawSpan| s.fragment().to_string()
    )(i)
}

pub trait Parse<'a>: Sized {

    /// parse the given span into self
    fn parse(input: RawSpan<'a>) -> ParseResult<'a,  Self>;

    /// 测试辅助方法, 将 str 转换为原始 span 并解析
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }

    fn parse_format_error(i: &'a str) -> Result<Self, FormattedError<'a>> {
        let input = LocatedSpan::new(i);
        match all_consuming(Self::parse)(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(i, e))
        }
    }

}

/// 构造出解析逗号分割语句的函数
pub(crate) fn comma_sep<'a, O, E, F>(f: F) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, Vec<O>, E>
    where
        F: nom::Parser<RawSpan<'a>, O, E>,
        E: nom::error::ParseError<RawSpan<'a>>
{
    // multispace0: 识别零个或多个空格、制表符、回车符和换行符
    // tuple 语句表示为逐个执行匹配
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}

/// 检查输入是否有传入的标签
pub(crate) fn peek_then_cut<'a, T, O, E, F>(peek_tag: T, f: F)
    -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, O, E>
    where
        T: nom::InputLength + Clone,
        F: nom::Parser<RawSpan<'a>, O, E>,
        E: nom::error::ParseError<RawSpan<'a>> + nom_supreme::tag::TagError<RawSpan<'a>, T>,
        LocatedSpan<&'a str>: nom::Compare<T>
{
    map(
        pair(
            // 尝试在不消耗输入的情况下应用其解析器
            peek(tag_no_case(peek_tag)),
            f
        ),
        |(_, f_res)| f_res
    )
}
