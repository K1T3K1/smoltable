use bytes::Bytes;
use smoltable_core::{
    datapoint::{Datapoint, FullKey, Value},
    errors::SmoltableError,
    memtable::BMapMemtable,
    utils::time::now,
    worker::SmoltableWorker,
};
use smoltable_proto::proto::{Cell, Column, Family, mutation::Mutation};
use smoltable_proto::proto::{
    MutateRowRequest, MutateRowResponse, ReadRowRequest, ReadRowResponse,
    smoltable_table_server::SmoltableTable,
};
use std::sync::{Arc, RwLock};
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

        let datapoints = req.mutations.into_iter().map(|m| match m.mutation {
            Some(Mutation::SetCell(sc)) => {
                let ts = if sc.timestamp_micros > 0 {
                    sc.timestamp_micros
                } else {
                    now()
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
                    now()
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
        let req = request.into_inner();

        let structured_row = {
            let wg = self
                .worker
                .read()
                .map_err(|_| Status::internal("Memtable error (lock poisoned)"))?;

            wg.get_row(req.row_key).map_err(|e| match e {
                SmoltableError::InvalidRetrievalKey => Status::invalid_argument(e.to_string()),
            })?
        };

        let response = ReadRowResponse {
            row_key: structured_row.row_key,
            families: structured_row
                .families
                .into_iter()
                .map(|f| Family {
                    name: f.name,
                    columns: f
                        .columns
                        .into_iter()
                        .map(|c| Column {
                            qualifier: c.qualifier,
                            cells: c
                                .cells
                                .into_iter()
                                .map(|cell| Cell {
                                    value: cell.value,
                                    timestamp_micros: cell.timestamp_micros,
                                })
                                .collect(),
                        })
                        .collect(),
                })
                .collect(),
        };

        Ok(Response::new(response))
    }
}
