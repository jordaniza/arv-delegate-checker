use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

#[derive(Debug, Deserialize)]
pub struct AccountWithId {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct DelegateData {
    pub account: AccountWithId,
    pub valueExact: String,
}

#[derive(Debug, Deserialize)]
pub struct ERC20Balances {
    pub erc20Balances: Vec<DelegateData>,
}

#[derive(Debug, Deserialize)]
pub struct DelegateResponse {
    pub data: ERC20Balances,
}
// fetch list of ARV addresses from the graph
pub async fn fetch_addresses() -> Result<DelegateResponse, Box<dyn std::error::Error>> {
    // Define the query.
    let query = json!({
        "query": r#"
            {
              erc20Balances(
                where: {
                  contract_: {id: "0x069c0Ed12dB7199c1DdAF73b94de75AAe8061d33"}, 
                  value_not: "0",
                  account_not: null
                }
              ) {
                account {
                  id
                }
                valueExact
              }
            }
        "#,
    });

    let client = Client::new();

    // Send the request.
    let response = client
        .post("https://api.thegraph.com/subgraphs/name/jordaniza/auxo-staking")
        .json(&query)
        .send()
        .await?;

    let body = response.text().await?;

    // Get the JSON response.
    let res: DelegateResponse = serde_json::from_str(&body)?;

    Ok(res)
}
