// Copyright 2018-2021 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub mod models;
mod operations;
pub(in crate) mod schema;

use diesel::r2d2::{ConnectionManager, Pool};

use super::diesel::models::{BatchModel, TransactionModel};
use super::{Batch, BatchList, BatchStore, BatchStoreError, Transaction};
use crate::error::ResourceTemporarilyUnavailableError;

use operations::add_batch::AddBatchOperation as _;
use operations::fetch_batch::FetchBatchOperation as _;
use operations::list_batches::ListBatchesOperation as _;
use operations::BatchStoreOperations;

#[derive(Clone)]
pub struct DieselBatchStore<C: diesel::Connection + 'static> {
    connection_pool: Pool<ConnectionManager<C>>,
}

impl<C: diesel::Connection> DieselBatchStore<C> {
    #[allow(dead_code)]
    pub fn new(connection_pool: Pool<ConnectionManager<C>>) -> Self {
        DieselBatchStore { connection_pool }
    }
}

#[cfg(feature = "postgres")]
impl BatchStore for DieselBatchStore<diesel::pg::PgConnection> {
    fn add_batch(&self, batch: Batch) -> Result<(), BatchStoreError> {
        BatchStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            BatchStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_batch(batch)
    }

    fn fetch_batch(&self, id: &str) -> Result<Option<Batch>, BatchStoreError> {
        BatchStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            BatchStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .fetch_batch(id)
    }

    fn list_batches(&self, offset: i64, limit: i64) -> Result<BatchList, BatchStoreError> {
        BatchStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            BatchStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_batches(offset, limit)
    }
}

#[cfg(feature = "sqlite")]
impl BatchStore for DieselBatchStore<diesel::sqlite::SqliteConnection> {
    fn add_batch(&self, batch: Batch) -> Result<(), BatchStoreError> {
        BatchStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            BatchStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .add_batch(batch)
    }

    fn fetch_batch(&self, id: &str) -> Result<Option<Batch>, BatchStoreError> {
        BatchStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            BatchStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .fetch_batch(id)
    }

    fn list_batches(&self, offset: i64, limit: i64) -> Result<BatchList, BatchStoreError> {
        BatchStoreOperations::new(&*self.connection_pool.get().map_err(|err| {
            BatchStoreError::ResourceTemporarilyUnavailableError(
                ResourceTemporarilyUnavailableError::from_source(Box::new(err)),
            )
        })?)
        .list_batches(offset, limit)
    }
}

impl From<(BatchModel, Vec<TransactionModel>)> for Batch {
    fn from((batch_model, transaction_models): (BatchModel, Vec<TransactionModel>)) -> Self {
        Self {
            header_signature: batch_model.header_signature,
            data_change_id: batch_model.data_change_id,
            signer_public_key: batch_model.signer_public_key,
            trace: batch_model.trace,
            serialized_batch: batch_model.serialized_batch,
            submitted: batch_model.submitted,
            submission_error: batch_model.submission_error,
            submission_error_message: batch_model.submission_error_message,
            dlt_status: batch_model.dlt_status,
            claim_expires: batch_model.claim_expires,
            created: batch_model.created,
            transactions: transaction_models
                .into_iter()
                .map(Transaction::from)
                .collect(),
        }
    }
}

impl From<Batch> for (BatchModel, Vec<TransactionModel>) {
    fn from(batch: Batch) -> Self {
        let batch_model = BatchModel {
            header_signature: batch.header_signature,
            data_change_id: batch.data_change_id,
            signer_public_key: batch.signer_public_key,
            trace: batch.trace,
            serialized_batch: batch.serialized_batch,
            submitted: batch.submitted,
            submission_error: batch.submission_error,
            submission_error_message: batch.submission_error_message,
            dlt_status: batch.dlt_status,
            claim_expires: batch.claim_expires,
            created: batch.created,
        };

        let transaction_models = batch
            .transactions
            .into_iter()
            .map(TransactionModel::from)
            .collect();

        (batch_model, transaction_models)
    }
}

impl From<TransactionModel> for Transaction {
    fn from(model: TransactionModel) -> Self {
        Self {
            header_signature: model.header_signature,
            batch_id: model.batch_id,
            family_name: model.family_name,
            family_version: model.family_version,
            signer_public_key: model.signer_public_key,
        }
    }
}

impl From<Transaction> for TransactionModel {
    fn from(transaction: Transaction) -> Self {
        Self {
            header_signature: transaction.header_signature,
            batch_id: transaction.batch_id,
            family_name: transaction.family_name,
            family_version: transaction.family_version,
            signer_public_key: transaction.signer_public_key,
        }
    }
}
