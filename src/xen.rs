use anyhow::Error;
use async_trait::async_trait;
use ethers::{
  prelude::{abigen, types::U256, Address},
  providers::Middleware,
};
use serde_json::to_string;

use crate::{ffi::TransactionJson, StakingTransactions};

pub struct XEN(U256);
pub struct Days(U256);

abigen!(
  XenContract,
  "https://etherscan.io/address/0x06450dEe7FD2Fb8E39061434BAbCFC05599a6Fb8"
);

#[async_trait]
impl<M: Middleware> StakingTransactions for XenContract<M> {
  type Token = XEN;
  type TermUnits = Days;
  fn stake(
    &self,
    address: Address,
    amount: Self::Token,
    term: Self::TermUnits,
  ) -> Result<TransactionJson, Error> {
    let json = to_string(&self.stake(amount.0, term.0).from(address).tx)?;
    Ok(TransactionJson { json })
  }
  fn withdraw(&self, address: Address) -> Result<TransactionJson, Error> {
    let json = to_string(&self.withdraw().from(address).tx)?;
    Ok(TransactionJson { json })
  }
}
