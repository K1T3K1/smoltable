use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use smoltable_core::{datapoint::StructuredRow, errors::SmoltableError};

use crate::wal::traits::WAL;

struct DefaultWAL {
    size: i64,
    capacity: i64,
    transaction_id: i64,
    path: Option<Box<PathBuf>>,
}

impl DefaultWAL {
    fn new(&self) -> Self {
        DefaultWAL {
            size: 0,
            capacity: 67_108_864, // 64 MB per file
            transaction_id: 0,
            path: None,
        }
    }

    fn get_fd_name(&self) -> Box<PathBuf> {
        if let Some(p) = self.path.clone() {
            return p;
        }
        Box::new(
            Path::new(&format!("/tmp/smoltable/wal_{}.log", self.transaction_id)).to_path_buf(),
        )
    }
}

impl WAL for DefaultWAL {
    fn write(&mut self, rows: Vec<StructuredRow>) -> Result<(), SmoltableError> {
        let path = self.get_fd_name();
        let mut fd = File::create(*path);
        let mut fd = match fd {
            Ok(fd) => fd,
            Err(_) => panic!(),
        };
        let rows: Vec<Vec<u8>> = rows
            .iter()
            .map(|sr| sr.to_row(self.transaction_id))
            .collect();
        let len: usize = rows.iter().map(|vec| vec.len()).sum();
        for row in rows {
            fd.write(&row);
        }
        self.size += len as i64;
        self.transaction_id += 1;
        if self.size > self.capacity {
            self.path = None;
            self.size = 0;
        }
        Ok(())
    }
}
