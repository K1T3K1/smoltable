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

pub struct RetrievalKey { 
    pub row: Bytes,
    pub family: Bytes,
    pub qualifier: Bytes,
    pub timestamp: Option<i64>
}

pub struct RetrievalStructure {
    pub retrieval_key: Vec<RetrievalKey>,
    pub n_last: i8,
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
