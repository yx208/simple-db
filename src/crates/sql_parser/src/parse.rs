use nom::{
    IResult,
    combinator::map,
    character::complete::{char, multispace0},
    bytes::complete::take_while1,
    sequence::tuple,
    multi::separated_list1
};
use nom_locate::LocatedSpan;

/// 使用 LocatedSpan 作为字符串输入的包装器
pub type RawSpan<'a> = LocatedSpan<&'a str>;
/// 这将使用默认错误类型，但我们将更改后者
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T>;

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

