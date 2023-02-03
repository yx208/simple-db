use std::collections::{BTreeMap,HashMap};
use serde::{Deserialize,Serialize};
use sql_parser::Column;
use std::process::Stdio;


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
            .map_or(0, |max_id, _| max_id + 1);

        let row: StoreRow = values
            .into_iter()
            .zip(self.columns.iter())
            .map(|(value, col)| (col.name.to_owned(), value))
            .collect();

        self.rows.insert(id, row);
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<usize, Row> {
        self.rows.iter()
    }
}
