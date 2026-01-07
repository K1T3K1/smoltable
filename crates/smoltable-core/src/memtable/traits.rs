use crate::datapoint::{FullKey, Value};

pub trait Memtable {
    fn insert(&mut self, key: FullKey, value: Value);
}
