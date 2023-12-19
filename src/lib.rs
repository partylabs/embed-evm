pub mod xen;

use anyhow::Error;
use ethers::{
  abi::Address,
  prelude::{EthEvent, Transaction},
  types::{Signature, U256},
};

#[swift_bridge::bridge]
pub mod ffi {

  #[swift_bridge(swift_repr = "struct")]
  pub struct TransactionJson {
    json: String,
  }

  #[swift_bridge(swift_repr = "struct")]
  pub struct SignedTransactionJson {
    json: String,
    signature: String,
  }
}

pub trait StakingTransactions {
  type Token;
  type TermUnits;
  fn stake(
    &self,
    address: Address,
    amount: Self::Token,
    term: Self::TermUnits,
  ) -> Result<ffi::TransactionJson, Error>;
  fn withdraw(&self, address: Address) -> Result<ffi::TransactionJson, Error>;
}
