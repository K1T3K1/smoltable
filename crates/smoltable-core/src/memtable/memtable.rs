use crate::datapoint::{Datapoint, FullKey, RetrievalKey, Value};
use crate::memtable::traits::Memtable;
use std::collections::BTreeMap;
use std::ops::Range;

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

impl BMapMemtable {
    fn get_key_range(&self, key: RetrievalKey) -> Range<FullKey> {
        let start_key = FullKey {
            row: key.row.to_vec().into(),
            family: vec![].into(),
            qualifier: vec![].into(),
            timestamp: i64::MAX,
        };

        let mut end_row = key.row.to_vec();
        if let Some(last) = end_row.last_mut() {
            if *last < 255 {
                *last += 1;
            } else {
                end_row.push(0);
            }
        } else {
            end_row.push(0);
        }

        let end_key = FullKey {
            row: end_row.into(),
            family: vec![].into(),
            qualifier: vec![].into(),
            timestamp: i64::MAX,
        };

        start_key..end_key
    }
}

impl Memtable for BMapMemtable {
    fn insert(&mut self, key: FullKey, value: Value) {
        println!("Inserted {:#?}", value.clone());
        self.data.insert(key, value);
    }

    fn get(&self, key: RetrievalKey) -> Vec<Datapoint> {
        let key_range = self.get_key_range(key);

        #[allow(unused_variables)]
        let datapoints: Vec<Datapoint> = self
            .data
            .range(key_range)
            .filter(|(full_key, _value)| {
                true // To do add filtering
            })
            .map(|(full_key, value)| Datapoint {
                full_key: full_key.clone(),
                value: value.clone(),
            })
            .collect();

        datapoints
    }
}
