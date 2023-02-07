use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, GenericErrorTree, StackContext};
use thiserror::Error;
use crate::parse::MyParseError;

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error")]
pub struct FormattedError<'b> {

    // need 'b since Diagnostic derive uses 'a
    #[source_code]
    src: &'b str,

    #[label("{kind}")]
    span: miette::SourceSpan,

    // TLDR: the parsing error
    kind: BaseErrorKind<&'b str, Box<dyn std::error::Error + Send + Sync + 'static>>,

    #[related]
    others: Vec<FormattedErrorContext<'b>>

}

#[derive(Error, Debug, Diagnostic)]
#[error("Parse Error Context")]
pub struct FormattedErrorContext<'b> {

    #[source_code]
    src: &'b str,

    #[label("${context}")]
    span: miette::SourceSpan,

    context: StackContext<&'b str>

}

pub fn format_parse_error<'a>(input: &'a str, e: MyParseError<'a>) -> FormattedError<'a> {
    match e {
        // a "normal" error like unexpected charcter
        GenericErrorTree::Base { location, kind } => {
            // location 类型是 nom_locate 的 RawSpan 类型
            // 只使用我们自己的 span/mark 一个包装器来实现可能会很好
            // From<OurSpan> for miette::SourceSpan
            let offset = location.location_offset().into();
            FormattedError {
                src: input,
                span: miette::SourceSpan::new(offset, 0.into()),
                kind,
                others: Vec::new()
            }
        }
        // 附加了上下文的错误（来自 nom 的上下文函数）
        GenericErrorTree::Stack { base, contexts } => {
            let mut base = format_parse_error(input, *base);
            let mut contexts: Vec<FormattedErrorContext> = contexts
                .into_iter()
                .map(|(location, context)| {
                    let offset = location.location_offset().into();
                    FormattedErrorContext {
                        src: input,
                        span: miette::SourceSpan::new(offset, 0.into()),
                        context,
                    }
                })
                .collect();
            base.others.append(&mut contexts);
            base
        }
        // an error from an "alt"
        GenericErrorTree::Alt(alt_errors) => {
            // 获取上下文最多的错误
            // 因为那解析了最多的东西
            // TODO: 弄清楚如何处理 ties
            alt_errors
                .into_iter()
                .map(|e| format_parse_error(input, e))
                .max_by_key(|formatted| formatted.others.len())
                .unwrap()
        }
    }
}
