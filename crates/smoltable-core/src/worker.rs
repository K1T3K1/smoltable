use crate::datapoint::Datapoint;
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
}
