pub use super::Core;

use crate::types::ChildNft;
use ink_prelude::string::String as PreludeString;

use ink_env::{
    AccountId,
    CallFlags,
};

use openbrush::{
    contracts::psp34::{
        extensions::{
            enumerable::*,
            metadata::*,
        },
        PSP34Ref,
        PSP34,
    },
    traits::{
        Storage,
        String,
    },
};

/// Trait definitions for Minting internal functions.

/// Helper trait for Minting
impl<T> Core<Id, PSP34Error> for T
where
    T: Storage<psp34::Data<enumerable::Balances>>
        + Storage<metadata::Data>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal,
{
    fn _set_attribute(&mut self, name: String, value: PreludeString) {
        let id = self._get_collection_id();
        self.data::<metadata::Data>()
            ._set_attribute(id, name, value.into_bytes());
    }

    fn _get_attribute(&self, name: String) -> Option<String> {
        self.get_attribute(self._get_collection_id(), name)
    }

    fn _get_collection_id(&self) -> Id {
        self.data::<psp34::Data<enumerable::Balances>>()
            .collection_id()
    }

    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), PSP34Error> {
        self.data::<psp34::Data<enumerable::Balances>>()
            ._mint_to(to, id)?;
        return Ok(())
    }

    fn _emit_transfer_event(&mut self, to: AccountId, token_id: Id) {
        <Self as psp34::Internal>::_emit_transfer_event(self, None, Some(to), token_id);
    }

    fn _transfer_child_ownership(
        &self,
        to: AccountId,
        child_nft: ChildNft,
    ) -> Result<(), PSP34Error> {
        // TODO check child collection is approved by this (parent) collection
        // let collection = self.get_collection(child_nft.0)
        //      .ok_or(RmrkError::ChildContractNotApproved)?;

        PSP34Ref::transfer_builder(&child_nft.0, to, child_nft.1, Vec::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire()
            .unwrap()?;

        Ok(())
    }

    fn _get_owner_of(&self, parent_token_id: Id) -> Option<AccountId> {
        self.data::<psp34::Data<enumerable::Balances>>()
            .owner_of(parent_token_id)
    }
}
