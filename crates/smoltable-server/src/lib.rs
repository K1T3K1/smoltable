mod server;

use crate::server::table::SmoltableTableService;
use smoltable_core::{memtable::BMapMemtable, worker::SmoltableWorker};
use smoltable_proto::proto::smoltable_table_server::SmoltableTableServer;
use std::sync::{Arc, RwLock};
use tonic::transport::Server;

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;

    let worker = SmoltableWorker::new(BMapMemtable::new());
    let service = SmoltableTableService {
        worker: Arc::new(RwLock::new(worker)),
    };

    Server::builder()
        .add_service(SmoltableTableServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
