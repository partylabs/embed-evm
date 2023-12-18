pub mod xen;

use ethers::{abi::Address, prelude::{EthEvent, Transaction}, types::{Signature, U256}};
use async_trait::async_trait;

#[swift_bridge::bridge]
pub mod ffi {

  #[swift_bridge(swift_repr="struct")]
  pub struct TransactionJson {
    json: String,
  }

  #[swift_bridge(swift_repr="struct")]
  pub struct SignedTransactionJson {
    json: String,
    signature: String,
  }
}

#[async_trait]
pub trait StakingTransaction {
  type Token;
  type TermUnits;
  type Err;
  async fn stake<Out: EthEvent>(
    &self,
    address: Address,
    amount: Self::Token,
    term: Self::TermUnits,
  ) -> Result<Out, Self::Err>;
  async fn withdraw<Out: EthEvent>(&self, address: Address) -> Result<Out, Self::Err>;
}
