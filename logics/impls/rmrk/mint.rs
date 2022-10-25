use crate::impls::rmrk::data;
pub use crate::traits::{
    errors::RmrkError,
    mint::{RmrkMintable, RmrkMintableRef},
};
use ink_prelude::string::{String, ToString};

use openbrush::{
    contracts::{psp34::*, reentrancy_guard::*},
    modifiers,
    traits::{AccountId, Balance, Storage},
};

impl<T> RmrkMintable for T
where
    T: Storage<data::Data>
        + Storage<psp34::Data>
        + Storage<reentrancy_guard::Data>
        + psp34::extensions::metadata::PSP34Metadata,
{
    /// Mint new tokens
    #[modifiers(non_reentrant)]
    default fn mint(&mut self, to: AccountId, mint_amount: u128) -> Result<(), RmrkError> {
        // self.data::<ownable::Data>().owner = _to;
        ink_env::debug_println!("####### mint RMRK contract amount:{:?}", mint_amount);
        if mint_amount == 0 {
            return Err(RmrkError::CannotMintZeroTokens);
        }
        if self.data::<psp34::Data>().total_supply() + mint_amount
            > self.data::<data::Data>().max_supply
        {
            ink_env::debug_println!("####### error CollectionFullOrLocked");
            return Err(RmrkError::CollectionFullOrLocked);
        }
        if Self::env().transferred_value() != mint_amount * self.data::<data::Data>().price_per_mint
        {
            ink_env::debug_println!("####### error MintUnderpriced");
            return Err(RmrkError::MintUnderpriced);
        }
        let next_to_mint = self.data::<psp34::Data>().total_supply() + 1; // first mint id is 1
        let mint_offset = next_to_mint + mint_amount;

        for mint_id in next_to_mint..mint_offset {
            ink_env::debug_println!("####### mint id:{:?}", mint_id);
            assert!(self
                .data::<psp34::Data>()
                ._mint_to(to, Id::U32(mint_id as u32))
                .is_ok());
        }

        Ok(())
    }

    /// Maximum amount of mintable tokens in this contract
    default fn max_supply(&self) -> u128 {
        self.data::<data::Data>().max_supply
    }

    /// The price to mint a single token in this contract
    default fn price_per_mint(&self) -> Balance {
        self.data::<data::Data>().price_per_mint
    }

    /// Get URI from token ID
    default fn token_uri(&self, token_id: u32) -> String {
        ink_env::debug_println!("####### get tokenUri for: {:?}", token_id);
        let value = self.get_attribute(
            self.data::<psp34::Data>().collection_id(),
            String::from("baseUri").into_bytes(),
        );
        let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &String::from(".json");
        ink_env::debug_println!("####### tokenUri is: [{:?}]", token_uri);
        return token_uri;
    }
}
