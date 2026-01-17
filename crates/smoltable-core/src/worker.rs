use crate::datapoint::{Datapoint, RetrievalKey, RetrievalStructure};
use crate::errors::SmoltableError;
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

    fn validate_retrieval_keys(&self, keys: &RetrievalStructure) -> Result<(), SmoltableError> {
        for key in &keys.retrieval_keys {
            match (key.timestamp, key.n_last) {
                (Some(_), None) => {}
                (None, Some(_)) => {}
                _ => return Err(SmoltableError::InvalidRetrievalKey),
            }
        }
        Ok(())
    }

    pub fn get(&self, key: RetrievalStructure) -> Result<Vec<Vec<Datapoint>>, SmoltableError> {
        self.validate_retrieval_keys(&key)?;
        let retrieved_data = key
            .retrieval_keys
            .into_iter()
            .map(|key| self.memtable.get(key))
            .collect();

        Ok(retrieved_data)
    }
}
