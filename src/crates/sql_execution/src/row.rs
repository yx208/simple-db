use std::collections::HashMap;
use std::rc::Rc;
use crate::error::QueryExceptionError;
use crate::table::ColumnInfo;

/// 查询响应中的一行
#[derive(Debug, Clone)]
pub struct Row<'a> {
    id: usize,
    columns: Rc<ColumnInfo>,
    data: &'a HashMap<String, String>
}

impl<'a> Row<'a> {
    pub fn new(columns: Rc<ColumnInfo>, id: usize, data: &'a HashMap<String, String>) -> Self {
        Self { id, columns, data }
    }

    pub fn columns(&self) -> &ColumnInfo {
        self.columns.as_ref()
    }

    pub fn get(&self, column: &String) -> String {
        self.try_get(column).unwrap()
    }

    /// Get a single value from the row
    pub fn try_get(&self, column: &String) -> Result<String, QueryExceptionError> {
        self.data.get(column).map_or_else(
            || Err(QueryExceptionError::ColumnDoesNotExists(column.to_owned())),
            |val| Ok(val.to_string())
        )
    }
}
