use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SqlQuery {
    Select(SelectStatement),
    Create(CreateStatement),
    Insert(InsertStatement)
}

pub struct SelectStatement;
pub struct InsertStatement;
pub struct CreateStatement;
