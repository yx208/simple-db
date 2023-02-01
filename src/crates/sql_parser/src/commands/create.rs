//# 解析的 sql 语句
//# ```sql
//# CREATE TABLE FOO (
//#     col1 string,
//#     col2 int
//# )
//# INSERT INTO FOO VALUES 1,2;
//# SELECT col1, col2 FROM foo;
//# ```

use nom::{
    error::context,
    combinator::map,
    branch::alt,
    // sequence: `组合器`按顺序应用`解析器` 模块
    sequence::{separated_pair, tuple, preceded},
    character::complete::{char, multispace1}
};
// tag_no_case 的错误处理更好
// ParserExt 主要用于在调用标识符时添加 `.context` 以说明我们想要哪种标识符
use nom_supreme::{tag::complete::tag_no_case, ParserExt};
use serde::{Deserialize, Serialize};

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};

// A colum's type
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum SqlTypeInfo {
    String,
    Int,
}

/// parses String | Int
impl <'a> Parse<'a> for SqlTypeInfo {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        // `context` 参数将有助于稍后提供更好的错误消息
        context(
            "Column Type",
            // alt 将尝试每个通过的解析器并返回成功的
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int)
            ))
        )(input)
    }
}

/// 列名 + 类型
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub type_info: SqlTypeInfo
}

/// parses "<columnName> <columnType>"
/// 解析出语句：`NAME string`
impl<'a> Parse<'a> for Column {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        context(
            "Create Column",
            map(
                // 分割一对值
                // `first` 是要应用的第一个解析器。
                // `seq` 是分隔符解析器
                // `second` 是要应用的第二个解析器
                separated_pair(
                    identifier.context("Column Name"),
                    multispace1,
                    SqlTypeInfo::parse
                ),
                // 把解析出来的东西，构造成一个 struct
                |(name, type_info)| Self { name, type_info }
            )
        )(input)
    }
}

/// 要创建的表及其列
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>
}

/// 解析包含在括号中的以逗号分隔的列定义列表
/// example:
/// (
///      col1 string,
///      col2 int
/// )
fn column_definitions(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "Column Definitions",
        map(
            tuple((
                char('('),
                comma_sep(Column::parse),
                char(')')
            )),
            |(_, cols, _)| cols
        )
    )(input)
}

/// parses "CREATE TABLE <table name> <column defs>
impl<'a> Parse<'a> for CreateStatement {
    fn parse(input: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            separated_pair(
                // 匹配：`create table NAME`
                preceded(
                    // 匹配：`create table`
                    tuple((
                        tag_no_case("create"),
                        multispace1,
                        tag_no_case("table"),
                        multispace1
                    )),
                    identifier.context("Table Name")
                ),
                multispace1,
                // 解析表主体
                column_definitions
            ).context("Create Table"),
            |(table, columns)| Self { table, columns }
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let expected = CreateStatement {
            table: "foo".into(),
            columns: vec![
                Column {
                    name: "col1".into(),
                    type_info: SqlTypeInfo::Int
                },
                Column {
                    name: "col2".into(),
                    type_info: SqlTypeInfo::String
                },
                Column {
                    name: "col3".into(),
                    type_info: SqlTypeInfo::String
                }
            ]
        };

        assert_eq!(
            CreateStatement::parse_from_raw("CREATE TABLE foo (col1 int, col2 string, col3 string)")
                .unwrap()
                .1,
            expected
        );
    }
}
