use bytes::Bytes;
use smoltable_core::{
    datapoint::{Datapoint, FullKey, Value},
    memtable::BMapMemtable,
    worker::SmoltableWorker,
};
use smoltable_proto::proto::{
    TableInsertRequest, TableInsertResponse, smoltable_table_server::SmoltableTable,
    table_insert_request::ValueType,
};
use std::sync::{Arc, RwLock};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct SmoltableTableService {
    pub worker: Arc<RwLock<SmoltableWorker<BMapMemtable>>>,
}

#[tonic::async_trait]
impl SmoltableTable for SmoltableTableService {
    async fn table_insert(
        &self,
        request: Request<TableInsertRequest>,
    ) -> Result<Response<TableInsertResponse>, Status> {
        let req = request.into_inner();

        let row_key = req
            .row_key
            .ok_or_else(|| tonic::Status::invalid_argument("row_key is required"))?;

        let value = match req.value_type {
            Some(ValueType::Value(v)) => Ok(Value::Some(Bytes::from(v))),
            Some(ValueType::Tombstone(_)) => Ok(Value::Tombstone),
            None => Err(tonic::Status::invalid_argument("value is required")),
        }?;

        let datapoint = Datapoint {
            full_key: FullKey {
                row: Bytes::from(row_key.row),
                family: Bytes::from(row_key.family),
                qualifier: Bytes::from(row_key.qualifier),
                timestamp: row_key.timestamp,
            },
            value: value,
        };

        {
            let mut worker_guard = self
                .worker
                .write()
                .map_err(|_| Status::internal("Internal RAM storage error (lock poisoned)"))?;
            worker_guard.insert(datapoint);
        }

        Ok(Response::new(TableInsertResponse { status: 0 }))
    }
}
