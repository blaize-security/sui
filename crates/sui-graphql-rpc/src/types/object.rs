// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{connection::Connection, *};

use super::name_service::NameService;
use super::{
    balance::Balance, coin::Coin, owner::Owner, stake::Stake, sui_address::SuiAddress,
    transaction_block::TransactionBlock,
};
use crate::{server::context_ext::DataProviderContextExt, types::base64::Base64};

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct Object {
    pub address: SuiAddress,
    pub version: u64,
    pub digest: String,
    pub storage_rebate: Option<u64>,
    pub owner: Option<SuiAddress>,
    pub bcs: Option<Base64>,
    pub previous_transaction: Option<String>,
    pub kind: Option<ObjectKind>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum ObjectKind {
    Owned,
    Child,
    Shared,
    Immutable,
}

#[derive(InputObject)]
pub(crate) struct ObjectFilter {
    package: Option<SuiAddress>,
    module: Option<String>,
    ty: Option<String>,

    owner: Option<SuiAddress>,
    object_ids: Option<Vec<SuiAddress>>,
    object_keys: Option<Vec<ObjectKey>>,
}

#[derive(InputObject)]
pub(crate) struct ObjectKey {
    object_id: SuiAddress,
    version: u64,
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
#[Object]
impl Object {
    async fn version(&self) -> u64 {
        self.version
    }

    async fn digest(&self) -> String {
        self.digest.clone()
    }

    async fn storage_rebate(&self) -> Option<u64> {
        self.storage_rebate
    }

    async fn bcs(&self) -> Option<Base64> {
        self.bcs.clone()
    }

    async fn previous_transaction_block(
        &self,
        ctx: &Context<'_>,
    ) -> Result<Option<TransactionBlock>> {
        if let Some(tx) = &self.previous_transaction {
            ctx.data_provider().fetch_tx(tx).await
        } else {
            Ok(None)
        }
    }

    async fn kind(&self) -> Option<ObjectKind> {
        self.kind
    }

    async fn owner(&self) -> Option<Owner> {
        self.owner.as_ref().map(|q| Owner { address: *q })
    }

    // =========== Owner interface methods =============

    pub async fn location(&self) -> SuiAddress {
        self.address
    }

    pub async fn object_connection(
        &self,
        ctx: &Context<'_>,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        filter: Option<ObjectFilter>,
    ) -> Result<Connection<String, Object>> {
        ctx.data_provider()
            .fetch_owned_objs(&self.address, first, after, last, before, filter)
            .await
    }

    pub async fn balance(&self, ctx: &Context<'_>, type_: Option<String>) -> Result<Balance> {
        ctx.data_provider()
            .fetch_balance(&self.address, type_)
            .await
    }

    pub async fn balance_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Option<Connection<String, Balance>> {
        unimplemented!()
    }

    pub async fn coin_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
        type_: Option<String>,
    ) -> Option<Connection<String, Coin>> {
        unimplemented!()
    }

    pub async fn stake_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Option<Connection<String, Stake>> {
        unimplemented!()
    }

    pub async fn default_name_service_name(&self) -> Option<String> {
        unimplemented!()
    }

    pub async fn name_service_connection(
        &self,
        first: Option<u64>,
        after: Option<String>,
        last: Option<u64>,
        before: Option<String>,
    ) -> Option<Connection<String, NameService>> {
        unimplemented!()
    }
}
