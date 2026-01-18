use bytes::Bytes;

use crate::datapoint::{
    Datapoint, FullKey, RetrievalKey, StructuredRow, StructuredRowCell, StructuredRowColumn,
    StructuredRowFamily, Value,
};
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

impl BMapMemtable {
    fn get_single_row(&self, row_key: Bytes) -> StructuredRow {
        let start = FullKey {
            row: row_key.clone(),
            family: Bytes::new(),
            qualifier: Bytes::new(),
            timestamp: i64::MAX,
        };

        let mut families: Vec<StructuredRowFamily> = Vec::new();

        for (key, value) in self.data.range(start..) {
            if key.row != row_key {
                break;
            }

            let v = match value {
                Value::Some(v) => v.clone(),
                Value::Tombstone => continue,
            };

            let is_new_family = families.last().map_or(true, |f| f.name != key.family);

            if is_new_family {
                families.push(StructuredRowFamily {
                    name: key.family.clone(),
                    columns: Vec::new(),
                });
            }

            let current_family = families.last_mut().unwrap();

            let is_new_col = current_family
                .columns
                .last()
                .map_or(true, |c| c.qualifier != key.qualifier);

            if is_new_col {
                current_family.columns.push(StructuredRowColumn {
                    qualifier: key.qualifier.clone(),
                    cells: Vec::new(),
                });
                let current_col = current_family.columns.last_mut().unwrap();

                current_col.cells.push(StructuredRowCell {
                    value: v,
                    timestamp_micros: key.timestamp,
                });
            }
        }

        StructuredRow { row_key, families }
    }
}

impl Memtable for BMapMemtable {
    fn insert(&mut self, key: FullKey, value: Value) {
        println!("Inserted {:#?}", value.clone());
        self.data.insert(key, value);
    }

    fn get(&self, key: RetrievalKey) -> Vec<Datapoint> {
        Vec::new()
    }

    fn get_row(&self, row_key: Bytes) -> StructuredRow {
        self.get_single_row(row_key)
    }
}
