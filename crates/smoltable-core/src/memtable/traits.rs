use bytes::Bytes;

use crate::datapoint::{Datapoint, FullKey, RetrievalKey, StructuredRow, Value};

pub trait Memtable {
    fn insert(&mut self, key: FullKey, value: Value);
    fn get(&self, key: RetrievalKey) -> Vec<Datapoint>;
    fn get_row(&self, row_key: Bytes) -> StructuredRow;
}
