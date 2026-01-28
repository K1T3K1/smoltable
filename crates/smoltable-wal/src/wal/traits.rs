use smoltable_core::{datapoint::StructuredRow, errors::SmoltableError};

pub trait WAL {
    fn write(&mut self, rows: Vec<StructuredRow>) -> Result<(), SmoltableError>;
}
