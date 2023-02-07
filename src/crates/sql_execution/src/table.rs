use std::collections::{BTreeMap,HashMap};
use std::rc::Rc;
use serde::{Deserialize,Serialize};
use sql_parser::Column;
use crate::row::Row;

// A row stored in a table
type StoreRow = HashMap<String, String>;

// List of column info
pub type ColumnInfo = Vec<Column>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Table {
    // row id to row
    rows: BTreeMap<usize, StoreRow>,
    // 表中所有列的列信息
    columns: ColumnInfo
}

impl Table {

    /// Create a table with the given column definitions
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            rows: BTreeMap::new(),
            columns
        }
    }

    /// Insert values (a row) into the table
    ///
    /// 假设值与传递给创建的列的顺序相同
    pub fn insert(&mut self, values: Vec<String>) {
        let id = self.rows
            .last_key_value()
            .map_or(0, |(max_id, _)| max_id + 1);

        let row: StoreRow = values
            .into_iter()
            .zip(self.columns.iter())
            .map(|(value, col)| (col.name.to_owned(), value))
            .collect();

        self.rows.insert(id, row);
    }

    pub fn iter(&self) -> impl Iterator<Item = Row> {
        self.into_iter()
    }
}

/// 使得 table 可以转换为迭代器
impl<'a> IntoIterator for &'a Table {
    type Item = Row<'a>;
    type IntoIter = TableIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let col_info = Rc::new(self.columns.clone());
        TableIter::new(self.rows.iter(), col_info)
    }
}

/// 现在我们可以为表创建自己的迭代器。我们可以从 BTreeMap 包装现有的 Iterator 并将每个响应转换为我们的 Row 结构。
/// 表中 [`Row`] 的迭代器
pub(crate) struct TableIter<'a> {
    /// btree_map 的底层迭代器
    map_iter: std::collections::btree_map::Iter<'a, usize, StoreRow>,
    /// table 的每一列
    columns: Rc<ColumnInfo>
}

impl<'a> TableIter<'a> {
    pub fn new(
        map_iter: std::collections::btree_map::Iter<'a, usize, StoreRow>,
        columns: Rc<ColumnInfo>
    ) -> Self
    {
        Self { map_iter, columns }
    }
}

impl<'a> Iterator for TableIter<'a> {
    type Item = Row<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.map_iter
            .next()
            .map(|(id, data)| {
                Row::new(self.columns.clone(), id.clone(), data)
            })
    }
}
