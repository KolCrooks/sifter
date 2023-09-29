pub enum DataType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Bool,
    DateTime,
    UUID,
    Bytes,
    BinaryVector,
}

pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

pub struct PrimaryKey {
    pub name: String,
    pub data_type: DataType,
}

pub struct Index {
    pub name: String,
    pub data_type: DataType,
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

pub struct Query {}
