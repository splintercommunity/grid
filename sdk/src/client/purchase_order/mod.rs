// Copyright 2021 Cargill Incorporated
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

use std::convert::TryFrom;

use crate::error::ClientError;
use crate::protocol::purchase_order::state::{
    PurchaseOrderAlternateId, PurchaseOrderAlternateIdBuilder,
};
use crate::purchase_order::store::{ListPOFilters, ListVersionFilters};

use super::Client;

#[cfg(feature = "client-reqwest")]
pub mod reqwest;

pub struct PurchaseOrder {
    pub purchase_order_uid: String,
    pub workflow_status: String,
    pub buyer_org_id: String,
    pub seller_org_id: String,
    pub is_closed: bool,
    pub accepted_version_id: Option<String>,
    pub versions: Vec<PurchaseOrderVersion>,
    pub created_at: i64,
    pub workflow_type: String,
}

pub struct PurchaseOrderVersion {
    pub version_id: String,
    pub workflow_status: String,
    pub is_draft: bool,
    pub current_revision_id: u64,
    pub revisions: Vec<PurchaseOrderRevision>,
}

pub struct PurchaseOrderRevision {
    pub revision_id: u64,
    pub order_xml_v3_4: String,
    pub submitter: String,
    pub created_at: i64,
}

pub struct AlternateId {
    pub purchase_order_uid: String,
    pub alternate_id_type: String,
    pub alternate_id: String,
}

impl AlternateId {
    pub fn new(purchase_order_uid: &str, alternate_id_type: &str, alternate_id: &str) -> Self {
        Self {
            purchase_order_uid: purchase_order_uid.to_string(),
            alternate_id_type: alternate_id_type.to_string(),
            alternate_id: alternate_id.to_string(),
        }
    }
}

impl TryFrom<AlternateId> for PurchaseOrderAlternateId {
    type Error = ClientError;

    fn try_from(id: AlternateId) -> Result<PurchaseOrderAlternateId, Self::Error> {
        let po = PurchaseOrderAlternateIdBuilder::new()
            .with_id_type(id.alternate_id_type.to_string())
            .with_id(id.alternate_id.to_string())
            .with_purchase_order_uid(id.purchase_order_uid)
            .build()
            .map_err(|err| {
                Self::Error::DaemonError(format!("Could not convert Alternate ID: {}", err))
            })?;

        Ok(po)
    }
}

pub trait PurchaseOrderClient: Client {
    /// Retrieves the purchase order with the specified `id`.
    ///
    /// # Arguments
    ///
    /// * `id` - The UID of the `PurchaseOrder` to be retrieved
    /// * `service_id` - Filter by service ID on the list of `PurchaseOrder`s
    fn get_purchase_order(
        &self,
        id: String,
        service_id: Option<&str>,
    ) -> Result<Option<PurchaseOrder>, ClientError>;

    /// Retrieves the purchase order version with the given `version_id` of the purchase
    /// order with the given `id`
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the `PurchaseOrder` containing the `PurchaseOrderVersion` to be retrieved
    /// * `version_id` - The version id of the `PurchaseOrderVersion` to be retrieved
    /// * `service_id` - The service ID to fetch the versions from
    fn get_purchase_order_version(
        &self,
        id: String,
        version_id: String,
        service_id: Option<&str>,
    ) -> Result<Option<PurchaseOrderVersion>, ClientError>;

    /// Retrieves the purchase order revision with the given `revision_id` of
    /// the purchase order version with the given `version_id` of the purchase order with the given `id`
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the `PurchaseOrder` containing the `PurchaseOrderRevision` to be retrieved
    /// * `version_id` - The version id of the `PurchaseOrderVersion` containing the
    ///   `PurchaseOrderRevision` to be retrieved
    /// * `revision_id` - The revision number of the `PurchaseOrderRevision` to be retrieved
    /// * `service_id` - The service ID to fetch the revision from
    fn get_purchase_order_revision(
        &self,
        id: String,
        version_id: String,
        revision_id: u64,
        service_id: Option<&str>,
    ) -> Result<Option<PurchaseOrderRevision>, ClientError>;

    /// lists purchase orders.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter to apply to the list of `PurchaseOrder`s
    /// * `service_id` - optional service id if running splinter
    fn list_purchase_orders(
        &self,
        filters: Option<ListPOFilters>,
        service_id: Option<&str>,
    ) -> Result<Vec<PurchaseOrder>, ClientError>;

    /// lists the purchase order versions of a specific purchase order.
    ///
    /// # Arguments
    ///
    /// * `id` - The uuid of the `PurchaseOrder` containing the `PurchaseOrderVersion`s to be listed
    /// * `filters` - Optional filters for for the versions
    /// * `service_id` - The service ID to fetch the versions from
    fn list_purchase_order_versions(
        &self,
        id: String,
        filters: Option<ListVersionFilters>,
        service_id: Option<&str>,
    ) -> Result<Vec<PurchaseOrderVersion>, ClientError>;

    /// lists the purchase order revisions of a specific purchase order version.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the `PurchaseOrder` containing the `PurchaseOrderRevision`s to be listed
    /// * `version_id` - The version id of the `PurchaseOrderVersion` containing
    ///   the `PurchaseOrderRevision`s to be listed
    /// * `service_id` - The service ID to fetch the revisions from
    fn list_purchase_order_revisions(
        &self,
        id: String,
        version_id: String,
        service_id: Option<&str>,
    ) -> Result<Vec<PurchaseOrderRevision>, ClientError>;

    /// Lists the purchase order's alternate IDs.
    ///
    /// # Arguments
    ///
    /// * `id` - The uid of the `PurchaseOrder` for the `AlternateId`s to be listed
    fn list_alternate_ids(
        &self,
        id: String,
        service_id: Option<&str>,
    ) -> Result<Vec<AlternateId>, ClientError>;
}
