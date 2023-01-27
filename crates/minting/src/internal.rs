use crate::MintingData;

use ink_env::AccountId;
use rmrk_common::{implementations::psp34::Core, errors::RmrkError};


use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        Storage,
        String,
    },
};

/// Trait definitions for Minting internal functions.
pub trait Internal {
    fn _get_attribute(&self, name: String) -> Option<String>;

    fn _mint_next(&mut self, to: AccountId) -> Result<u64, PSP34Error>;

    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<(), PSP34Error>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error>;
}

/// Helper trait for Minting
impl<T> Internal for T
where
    T: Storage<MintingData> + Storage<psp34::Data<enumerable::Balances>> + Core<Id, PSP34Error>,
{
    default fn _get_attribute(&self, name: String) -> Option<String> {
        self._get_attribute(name)
    }
    /// Mint next available token for the caller
    default fn _mint_next(&mut self, to: AccountId) -> Result<u64, PSP34Error> {
        let token_id = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(1)
            .ok_or(PSP34Error::Custom(String::from(
                RmrkError::CollectionIsFull.as_str(),
            )))?;

        self._mint_to(to, Id::U64(token_id))?;

        self.data::<MintingData>().last_token_id += 1;

        self._emit_transfer_event(to, Id::U64(token_id));

        return Ok(token_id)
    }

    /// Check if the transferred mint values is as expected
    default fn _check_value(
        &self,
        transfered_value: u128,
        mint_amount: u64,
    ) -> Result<(), PSP34Error> {
        if let Some(value) =
            (mint_amount as u128).checked_mul(self.data::<MintingData>().price_per_mint)
        {
            if transfered_value == value {
                return Ok(())
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::BadMintValue.as_str(),
        )))
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<(), PSP34Error> {
        if mint_amount == 0 {
            return Err(PSP34Error::Custom(String::from(
                RmrkError::CannotMintZeroTokens.as_str(),
            )))
        }
        if let Some(amount) = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(mint_amount)
        {
            if amount <= self.data::<MintingData>().max_supply {
                return Ok(())
            }
        }
        return Err(PSP34Error::Custom(String::from(
            RmrkError::CollectionIsFull.as_str(),
        )))
    }
}
