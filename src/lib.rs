pub mod xen;

use ethers::{abi::Address, prelude::EthEvent};
use async_trait::async_trait;

#[async_trait]
pub trait Stakeable {
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
