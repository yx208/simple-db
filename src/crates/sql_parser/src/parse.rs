use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::IResult;
use nom_locate::LocatedSpan;

// 使用 LocatedSpan 作为字符串输入的包装器
pub type RawSpan<'a> = LocatedSpan<&'a str>;
// 这将使用默认错误类型，但我们将更改后者
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T>;

// 解析列和表的标识符
pub(crate) fn identifier(i: RawSpan) -> ParseResult<Stirng> {
    map(
        take_while1(|c: char| c.is_alphanumeric()),
        |s: RawSpan| s.fragment().to_string()
    )(i)
}

pub trait Parse<'a>: Sized {

    // parse the given span into self
    fn parse(input: RawSpan<'a>) -> ParseResult<'a,  Self>;

    // 测试辅助方法, 将 str 转换为原始 span 并解析
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let i = LocatedSpan::new(input);
        Self::parse(i)
    }

}

fn main() {
    println!("Hello, world!");
}


