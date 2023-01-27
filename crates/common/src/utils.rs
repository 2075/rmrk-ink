//! Set of functions commonly used with PSP34 contract

use ink_prelude::string::String as PreludeString;

use crate::{
    errors::RmrkError,
    implementations::Core,
    types::{
        Id,
        PSP34Error,
    },
};

use openbrush::{
    contracts::{
        ownable::*,
        reentrancy_guard::*,
    },
    modifiers,
    traits::{
        AccountId,
        Storage,
        String,
    },
};

/// Trait definitions for Utils functions
#[openbrush::trait_definition]
pub trait Utils {
    /// Set new value for the baseUri.
    #[ink(message)]
    fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error>;

    /// Withdraw contract's balance.
    #[ink(message)]
    fn withdraw(&mut self) -> Result<(), PSP34Error>;

    /// Ensure that token exists
    fn ensure_exists_and_get_owner(&self, id: &Id) -> Result<AccountId, PSP34Error>;

    /// Ensure that the caller is the token owner
    fn ensure_token_owner(&self, token_owner: AccountId) -> Result<(), PSP34Error>;
}

impl<T> Utils for T
where
    T: Storage<reentrancy_guard::Data> + Storage<ownable::Data> + Core<Id, PSP34Error>,
{
    /// Set new value for the baseUri
    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: PreludeString) -> Result<(), PSP34Error> {
        self._set_attribute(String::from("baseUri"), uri);
        Ok(())
    }

    /// Withdraw contract's balance
    #[modifiers(only_owner)]
    default fn withdraw(&mut self) -> Result<(), PSP34Error> {
        let balance = Self::env().balance();
        let current_balance = balance
            .checked_sub(Self::env().minimum_balance())
            .unwrap_or_default();
        Self::env()
            .transfer(self.data::<ownable::Data>().owner(), current_balance)
            .map_err(|_| PSP34Error::Custom(String::from(RmrkError::WithdrawalFailed.as_str())))?;
        Ok(())
    }
    /// Check if token is minted. Return the owner
    default fn ensure_exists_and_get_owner(&self, id: &Id) -> Result<AccountId, PSP34Error> {
        let token_owner = self
            ._get_owner_of(id.clone())
            .ok_or(PSP34Error::TokenNotExists)?;
        Ok(token_owner)
    }

    /// Ensure that the caller is the token owner
    default fn ensure_token_owner(&self, token_owner: AccountId) -> Result<(), PSP34Error> {
        let caller = Self::env().caller();
        if caller != token_owner {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::NotTokenOwner.as_str(),
            )))
        }
        Ok(())
    }
}
