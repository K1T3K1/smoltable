use crate::datapoint::{FullKey, Value};
use crate::memtable::traits::Memtable;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct BMapMemtable {
    data: BTreeMap<FullKey, Value>,
}

impl BMapMemtable {
    pub fn new() -> Self {
        BMapMemtable {
            data: BTreeMap::new(),
        }
    }
}

impl Memtable for BMapMemtable {
    fn insert(&mut self, key: FullKey, value: Value) {
        println!("Inserted {:#?}", value.clone());
        self.data.insert(key, value);
    }
}
