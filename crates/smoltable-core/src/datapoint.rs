use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreKey {
    pub row: Bytes,
    pub family: Bytes,
    pub qualifier: Bytes,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FullKey {
    pub row: Bytes,
    pub family: Bytes,
    pub qualifier: Bytes,
    pub timestamp: i64,
}

impl Ord for FullKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row
            .cmp(&other.row)
            .then(self.family.cmp(&other.family))
            .then(self.qualifier.cmp(&other.qualifier))
            .then(other.timestamp.cmp(&self.timestamp))
    }
}

impl PartialOrd for FullKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
pub struct RetrievalKey {
    pub row: Bytes,
    pub family: Bytes,
    pub qualifier: Bytes,
    pub timestamp: Option<i64>,
    pub n_last: Option<i8>,
}

pub struct RetrievalStructure {
    pub retrieval_keys: Vec<RetrievalKey>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Some(Bytes),
    Tombstone,
}

pub struct Datapoint {
    pub full_key: FullKey,
    pub value: Value,
}

pub struct StructuredRowCell {
    pub value: Bytes,
    pub timestamp_micros: i64,
}

pub struct StructuredRowColumn {
    pub qualifier: Bytes,
    pub cells: Vec<StructuredRowCell>,
}

pub struct StructuredRowFamily {
    pub name: Bytes,
    pub columns: Vec<StructuredRowColumn>,
}
pub struct StructuredRow {
    pub row_key: Bytes,
    pub families: Vec<StructuredRowFamily>,
}

impl StructuredRow {
    pub fn to_row(&self, transaction_id: i64) -> Vec<u8> {
        Vec::new()
    }
}
