use crate::datapoint::{Datapoint, FullKey, RetrievalKey, Value};

pub trait Memtable {
    fn insert(&mut self, key: FullKey, value: Value);
    fn get(&self, key: RetrievalKey) -> Vec<Datapoint>;
}
