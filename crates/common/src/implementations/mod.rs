use ink_env::AccountId;

use crate::types::ChildNft;

pub mod psp34;
use ink_prelude::string::String as PreludeString;
use openbrush::traits::String;

pub trait Core<Id, Error> {
    fn _mint_to(&mut self, to: AccountId, id: Id) -> Result<(), Error>;

    fn _get_collection_id(&self) -> Id;

    fn _get_attribute(&self, name: String) -> Option<String>;

    fn _set_attribute(&mut self, name: String, value: PreludeString);

    fn _emit_transfer_event(&mut self, to: AccountId, token_id: Id);

    fn _transfer_child_ownership(&self, to: AccountId, child_nft: ChildNft) -> Result<(), Error>;

    fn _get_owner_of(&self, parent_token_id: Id) -> Option<AccountId>;
}
