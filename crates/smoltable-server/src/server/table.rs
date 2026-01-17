use bytes::Bytes;
use smoltable_core::{
    datapoint::{Datapoint, FullKey, RetrievalStructure, Value},
    memtable::BMapMemtable,
    worker::SmoltableWorker,
};
use smoltable_proto::proto::mutation::Mutation;
use smoltable_proto::proto::{
    MutateRowRequest, MutateRowResponse, ReadRowRequest, ReadRowResponse,
    smoltable_table_server::SmoltableTable,
};
use std::{
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};
use tonic::{Request, Response, Status};

#[derive(Default)]
pub struct SmoltableTableService {
    pub worker: Arc<RwLock<SmoltableWorker<BMapMemtable>>>,
}

#[tonic::async_trait]
impl SmoltableTable for SmoltableTableService {
    async fn mutate_row(
        &self,
        request: Request<MutateRowRequest>,
    ) -> Result<Response<MutateRowResponse>, Status> {
        let req = request.into_inner();

        let row_key = Bytes::from(req.row_key);
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_micros() as i64)
            .unwrap_or(0);

        let datapoints = req.mutations.into_iter().map(|m| match m.mutation {
            Some(Mutation::SetCell(sc)) => {
                let ts = if sc.timestamp_micros > 0 {
                    sc.timestamp_micros
                } else {
                    now
                };
                Datapoint {
                    full_key: FullKey {
                        row: row_key.clone(),
                        family: Bytes::from(sc.family_name),
                        qualifier: Bytes::from(sc.column_qualifier),
                        timestamp: ts,
                    },
                    value: Value::Some(Bytes::from(sc.value)),
                }
            }
            Some(Mutation::DeleteFromColumn(dfc)) => {
                let ts = if dfc.timestamp_micros > 0 {
                    dfc.timestamp_micros
                } else {
                    now
                };
                Datapoint {
                    full_key: FullKey {
                        row: row_key.clone(),
                        family: Bytes::from(dfc.family_name),
                        qualifier: Bytes::from(dfc.column_qualifier),
                        timestamp: ts,
                    },
                    value: Value::Tombstone,
                }
            }
            None => todo!(),
        });
        {
            let mut wg = self
                .worker
                .write()
                .map_err(|_| Status::internal("Memtable error (lock poisoned)"))?;
            for dp in datapoints {
                wg.insert(dp);
            }
        }
        Ok(Response::new(MutateRowResponse {}))
    }

    async fn read_row(
        &self,
        request: Request<ReadRowRequest>,
    ) -> Result<Response<ReadRowResponse>, Status> {
        let rs = RetrievalStructure { retrieval_keys: [] };
        {
            let wg = self
                .worker
                .read()
                .map_err(|_| Status::internal("Memtable error (lock poisoned)"))?;

            return Ok(Response::new(ReadRowResponse { wg.get(rs) } ));
        }
        Err(Status::internal("OK"))
    }
}
