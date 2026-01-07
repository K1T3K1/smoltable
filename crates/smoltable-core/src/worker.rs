use crate::datapoint::{Datapoint, RetrievalKey, RetrievalStructure, Value};
use crate::memtable::Memtable;

#[derive(Default)]
pub struct SmoltableWorker<T> {
    memtable: T,
}

impl<T> SmoltableWorker<T>
where
    T: Memtable,
{
    pub fn new(memtable: T) -> Self {
        SmoltableWorker { memtable: memtable }
    }

    pub fn insert(&mut self, datapoint: Datapoint) {
        self.memtable.insert(datapoint.full_key, datapoint.value);
    }

    fn prepare_retrieval_keys(&self, key: RetrievalStructure) -> Vec<RetrievalKey> {
        Vec::new()
    }

    pub fn get(&self, key: RetrievalStructure) -> Vec<Vec<Datapoint>> {
        let prepared_keys = self.prepare_retrieval_keys(key);
        prepared_keys.into_iter().map(|key| { self.memtable.get(key) }).collect()
    }
}
