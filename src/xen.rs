use async_trait::async_trait;
use ethers::prelude::abigen;
use ethers::prelude::types::U256;
use ethers::providers::Middleware;
use anyhow::Error;
use ethers::prelude::{Address, EthEvent};

use crate::Stakeable;

pub struct XEN(U256);
pub struct Days(U256);

abigen!(
  XenContract,
  "https://etherscan.io/address/0x06450dEe7FD2Fb8E39061434BAbCFC05599a6Fb8"
);

#[async_trait]
impl<M: Middleware> Stakeable for XenContract<M> {
  type Token = XEN;
  type TermUnits = Days;
  type Err = Error;
  async fn stake<Out: EthEvent>(
    &self,
    address: Address,
    amount: Self::Token,
    term: Self::TermUnits,
  ) -> Result<Out, Self::Err> {
    let receipt = self.stake(amount.0, term.0).await;
    // TODO: Don't execute here, return transaction instead
  }
  async fn withdraw<Out: EthEvent>(&self, address: Address) -> Result<Out, Self::Err> {
    unimplemented!();
  }
}
