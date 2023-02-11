use crate::MintingData;

use rmrk_common::errors::{
    Result,
    RmrkError,
};

use ink_env::AccountId;
use ink_prelude::string::{
    String as PreludeString,
    ToString,
};

use openbrush::{
    contracts::psp34::extensions::enumerable::*,
    traits::{
        Storage,
        String,
    },
};

/// Trait definitions for Minting internal functions.
pub trait Internal {
    /// Check if the transferred mint values is as expected.
    fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<()>;

    /// Check amount of tokens to be minted.
    fn _check_amount(&self, mint_amount: u64) -> Result<()>;

    /// Mint next token to specified account
    fn _mint(&mut self, to: AccountId) -> Result<Id>;

    /// Mint many tokens to specified account
    fn _mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)>;

    fn _token_uri(&self, token_id: u64) -> Result<PreludeString>;
}

/// Helper trait for Minting
impl<T> Internal for T
where
    T: Storage<MintingData>
        + psp34::extensions::metadata::PSP34Metadata
        + psp34::Internal
        + Storage<psp34::Data<enumerable::Balances>>,
{
    /// Check if the transferred mint values is as expected
    default fn _check_value(&self, transfered_value: u128, mint_amount: u64) -> Result<()> {
        if let Some(value) =
            (mint_amount as u128).checked_mul(self.data::<MintingData>().price_per_mint)
        {
            if transfered_value == value {
                return Ok(())
            }
        }
        return Err(RmrkError::BadMintValue.into())
    }

    /// Check amount of tokens to be minted
    default fn _check_amount(&self, mint_amount: u64) -> Result<()> {
        if mint_amount == 0 {
            return Err(RmrkError::CannotMintZeroTokens.into())
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
        return Err(RmrkError::CollectionIsFull.into())
    }

    /// Mint next token to specified account
    default fn _mint(&mut self, to: AccountId) -> Result<Id> {
        let token_id = self
            .data::<MintingData>()
            .last_token_id
            .checked_add(1)
            .ok_or(RmrkError::CollectionIsFull)?;

        self._mint_to(to, Id::U64(token_id))?;

        self.data::<MintingData>().last_token_id = token_id;

        Ok(Id::U64(token_id))
    }

    /// Mint many tokens to specified account
    default fn _mint_many(&mut self, to: AccountId, mint_amount: u64) -> Result<(Id, Id)> {
        let next_to_mint = self.data::<MintingData>().last_token_id + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for _ in next_to_mint..mint_offset {
            self._mint(to)?;
        }

        Ok((Id::U64(next_to_mint), Id::U64(mint_offset - 1)))
    }

    default fn _token_uri(&self, token_id: u64) -> Result<PreludeString> {
        let uri: PreludeString;
        match self
            .data::<MintingData>()
            .nft_metadata
            .get(Id::U64(token_id))
        {
            Some(token_uri) => {
                uri = PreludeString::from_utf8(token_uri).unwrap();
            }
            None => {
                let value = self.get_attribute(
                    self.data::<psp34::Data<enumerable::Balances>>()
                        .collection_id(),
                    String::from("baseUri"),
                );
                let token_uri = PreludeString::from_utf8(value.unwrap()).unwrap();
                uri = token_uri + &token_id.to_string() + &PreludeString::from(".json");
            }
        }
        Ok(uri)
    }
}
