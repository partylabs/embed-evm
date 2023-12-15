use ethers::prelude::abigen;

abigen!(Xen, "https://etherscan.io/address/0x06450dEe7FD2Fb8E39061434BAbCFC05599a6Fb8");

fn stake() {
  Xen::new();
}


