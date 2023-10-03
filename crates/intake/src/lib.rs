use chrono::{DateTime, Utc};

pub enum ColumnTypes {
    I64,
    F64,
    String,
    Bool,
    DateTime,
    UUID,
    Bytes,
    BinaryVector
}

pub enum DataValue {
    I64(i64),
    F64(f64),
    String(String),
    Bool(bool),
    DateTime(DateTime<Utc>),
    UUID(uuid::Uuid),
    Bytes(Vec<u8>)
}

pub struct Column {
    pub name: String,
    pub data_type: ColumnTypes,
}

pub struct PrimaryKey {
    pub name: String,
    pub data_type: ColumnTypes,
}

pub struct Index {
    pub name: String,
    pub data_type: ColumnTypes,
    pub is_partition_key: bool,
}

pub struct CreateTable {
    pub name: String,
    pub primary_key: Column,
    pub indexes: Vec<Index>,
}
pub struct DropTable {
    pub name: String,
}

type PartitionId = u64;

pub struct Query {
    pub table: String,
    pub partitions: Vec<PartitionId>,
    pub query: query_parser::OpTree,
    pub parameters: Vec<DataValue>,
    pub columns: Vec<Column>,
}
