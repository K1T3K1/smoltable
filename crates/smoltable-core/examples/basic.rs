use smoltable_core::{memtable::BMapMemtable, worker::SmoltableWorker};

fn main() {
    let _mt_worker = SmoltableWorker::new(BMapMemtable::new());
}
