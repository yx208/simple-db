use std::collections::HashMap;
use std::rc::Rc;
use crate::table::ColumnInfo;
s
/// 查询响应中的一行
#[derive(Debug, Clone)]
pub struct Row<'a> {
    id: usize,
    columns: Rc<ColumnInfo>,
    data: &'a HashMap<String, String>
}
