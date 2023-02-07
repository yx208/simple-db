mod table;
mod row;
mod error;

use std::collections::HashMap;
use derive_more::Display;
use sql_parser::ast::{parse_sql_query, SqlQuery};
use crate::error::{QueryExceptionError, SQLError};
use crate::row::Row;
use crate::table::Table;

#[derive(Debug, Display)]
pub enum ExecResponse<'a> {
    #[display(fmt = "{_0:?}")] // only show the values not "Select(...)"
    Select(Vec<Row<'a>>),
    Insert,
    Create
}

#[derive(Debug, Default)]
pub struct Execution {
    tables: HashMap<String, Table>
}

impl Execution {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new()
        }
    }

    pub fn run(&mut self, query: SqlQuery) -> Result<ExecResponse, QueryExceptionError> {
        // 判别语义
        match query {
            SqlQuery::Select(select) => {
                let table = select.table;
                let table = self
                    .tables
                    .get(&table)
                    .ok_or(QueryExceptionError::TableNotFound(table))?;

                let rows = table.iter().collect();
                Ok(ExecResponse::Select(rows))
            }
            SqlQuery::Insert(insert) => {
                let Some(table) = self.tables.get_mut(&insert.table) else {
                    return Err(QueryExceptionError::TableNotFound(insert.table))
                };

                table.insert(insert.values);
                Ok(ExecResponse::Insert)
            }
            SqlQuery::Create(create) => {
                let table = Table::new(create.columns);

                self.tables.insert(create.table, table);
                Ok(ExecResponse::Create)
            }
        }
    }

    pub fn parse_and_run<'a>(&mut self, query: &'a str) -> Result<ExecResponse, SQLError<'a>> {
        let query = parse_sql_query(query)?;
        let res = self.run(query)?;
        Ok(res)
    }
}





