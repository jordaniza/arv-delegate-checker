use ethers::contract::Multicall;
use ethers::prelude::*;
use std::{str::FromStr, sync::Arc};
mod arv_balances;
mod erc20votes;
use arv_balances::fetch_addresses;
use erc20votes::ERC20Votes;

const RPC_URL: &str = "https://eth.llamarpc.com";
const ARV: &str = "0x069c0Ed12dB7199c1DdAF73b94de75AAe8061d33";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = fetch_addresses().await?;

    let provider = Arc::new(Provider::<Http>::try_from(RPC_URL)?);

    let addresses = res
        .data
        .erc20Balances
        .iter()
        .map(|x| x.account.id.clone())
        .collect::<Vec<String>>();

    let arv_address: Address = ARV.parse()?;
    let arv = ERC20Votes::new(arv_address, provider.clone());
    let mut multicall = Multicall::<Provider<_>>::new(provider.clone(), None).await?;

    for address in addresses.clone() {
        let account = Address::from_str(address.as_str()).unwrap();
        let call = arv.delegates(account);
        multicall.add_call(call, false);
    }

    let results = multicall.call_array::<Address>().await?;

    // log any addresses who are not self delegated
    for (address, delegate) in addresses.iter().zip(results) {
        let owner: Address = address.parse()?;
        if delegate != owner {
            println!("{:?} -> {:?}", owner, delegate);
        }
    }

    Ok(())
}
