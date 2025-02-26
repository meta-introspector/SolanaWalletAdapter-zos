use std::{cell::RefCell, rc::Rc, str::FromStr};

use serde::Deserialize;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    system_instruction::transfer,
    transaction::{Transaction, TransactionError},
};
use wallet_adapter::{Cluster, SendOptions, WalletAdapter, WalletError, WalletResult};
use web_sys::Window;
use yew::{platform::spawn_local, Reducible, UseReducerHandle, UseStateHandle};

use super::{FetchReq, GlobalAction, GlobalAppState, NotificationInfo};

pub(crate) type AccountInfoState = UseReducerHandle<AccountInfoData>;

#[derive(Debug, Clone)]
pub(crate) struct AccountInfoAction {
    pub(crate) window: Window,
    pub(crate) address: String,
    pub(crate) endpoint: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub(crate) struct AccountInfoData {
    pub(crate) balance: RefCell<String>,
    pub(crate) token_accounts: RefCell<Vec<TokenAccountResponse>>,
    pub(crate) transactions: RefCell<Vec<SignaturesResponse>>,
}

impl Reducible for AccountInfoData {
    type Action = (UseStateHandle<bool>, AccountInfoAction, GlobalAppState);

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        *self.balance.borrow_mut() = String::default();
        *self.token_accounts.borrow_mut() = Vec::default();
        *self.transactions.borrow_mut() = Vec::default();

        web_sys::console::log_1(&"CALLED IN EFFECT".into());
        let self_inner = self.clone();

        spawn_local(async move {
            match account_info_fetcher(&action.1.address, &action.1.endpoint, &action.1.window)
                .await
            {
                Ok((balance, token_accounts, transactions)) => {
                    web_sys::console::log_1(&format!("INFO: {balance:?}").into());

                    *self_inner.balance.borrow_mut() = balance;
                    *self_inner.token_accounts.borrow_mut() = token_accounts;
                    *self_inner.transactions.borrow_mut() = transactions;

                    action.0.set(true);
                }
                Err(error) => {
                    web_sys::console::log_1(&format!("ERROR> INFO: {error:?}").into());

                    action
                        .2
                        .dispatch(GlobalAction::Message(NotificationInfo::error(format!(
                            "Encountered error when fetching account info: {error:?}"
                        ))));

                    action.0.set(true);
                }
            }

            action
                .2
                .dispatch(GlobalAction::LoadingFalse(action.0.clone()));
        });

        self
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHashResponseValue<'a> {
    #[serde(borrow)]
    pub blockhash: &'a str,
    pub last_valid_block_height: u64,
}

pub async fn get_balance(address: &str, endpoint: &str, window: &Window) -> WalletResult<String> {
    let balance_options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method": "getBalance",
        "params": [
            address
        ]
    }
    .to_string();

    let balance_response = FetchReq::new_for_rpc()?
        .set_body(&balance_options)
        .send(endpoint, window)
        .await?;

    let parsed_balance =
        serde_json::from_str::<RpcResponse<ResponseWithContext<u64>>>(&balance_response)
            .map_err(|error| WalletError::Op(error.to_string()))?;

    // WARNING: Do better financial math here
    Ok((parsed_balance.result.value as f64 / LAMPORTS_PER_SOL as f64).to_string())
}

pub async fn send_sol_req(
    recipient: &str,
    lamports: u64,
    endpoint: &str,
    adapter: &WalletAdapter,
    public_key_bytes: [u8; 32],
    cluster: Cluster,
    window: &Window,
) -> WalletResult<()> {
    let pubkey = Pubkey::new_from_array(public_key_bytes);
    let recipient = Pubkey::from_str(recipient).or(Err(WalletError::Op(
        "Invalid Recipient Address".to_string(),
    )))?;

    let send_sol_instruction = transfer(&pubkey, &recipient, lamports);
    let mut tx = Transaction::new_with_payer(&[send_sol_instruction], Some(&pubkey));
    let blockhash = get_blockhash(endpoint, window).await?;

    tx.message.recent_blockhash = blockhash;
    let tx_bytes = bincode::serialize(&tx).map_err(|error| WalletError::Op(error.to_string()))?;

    adapter
        .sign_and_send_transaction(&tx_bytes, cluster, SendOptions::default())
        .await?;

    Ok(())
}

pub async fn request_airdrop(
    lamports: u64,
    address: &str,
    endpoint: &str,
    window: &Window,
) -> WalletResult<()> {
    let options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method": "requestAirdrop",
        "params": [
            address,
            lamports
        ]
    }
    .to_string();

    let response = FetchReq::new_for_rpc()?
        .set_body(&options)
        .send(endpoint, window)
        .await?;

    serde_json::from_str::<RpcResponse<String>>(&response)
        .map_err(|error| WalletError::Op(error.to_string()))?;

    Ok(())
}

pub async fn account_info_fetcher(
    address: &str,
    endpoint: &str,
    window: &Window,
) -> WalletResult<(String, Vec<TokenAccountResponse>, Vec<SignaturesResponse>)> {
    let balance = crate::get_balance(address, endpoint, window).await?;

    let token_accounts_options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method": "getTokenAccountsByOwner",
        "params": [
            address,
            {
                "programId": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            },
            {
                "encoding": "jsonParsed"
            }
        ]
    }
    .to_string();

    let fetched_token_accounts = FetchReq::new_for_rpc()?
        .set_body(&token_accounts_options)
        .send(endpoint, window)
        .await?;

    let parsed_token_accounts = serde_json::from_str::<
        RpcResponse<ResponseWithContext<Vec<TokenAccountResponse>>>,
    >(&fetched_token_accounts)
    .map_err(|error| WalletError::Op(error.to_string()))?;

    let token_accounts = parsed_token_accounts.result.value;

    let get_signatures_options = jzon::object! {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSignaturesForAddress",
        "params": [
          address
        ]
    }
    .to_string();
    let fetched_signatures = FetchReq::new("POST")?
        .add_header("Content-Type", "application/json")?
        .set_body(&get_signatures_options)
        .send(endpoint, window)
        .await?;

    let parsed_signatures_response =
        serde_json::from_str::<RpcResponse<Vec<SignaturesResponse>>>(&fetched_signatures)
            .map_err(|error| WalletError::Op(error.to_string()))?;

    let signatures = parsed_signatures_response.result;

    Ok((balance, token_accounts, signatures.clone()))
}

async fn get_blockhash(endpoint: &str, window: &Window) -> WalletResult<solana_sdk::hash::Hash> {
    let options = jzon::object! {
        "id":1,
        "jsonrpc":"2.0",
        "method":"getLatestBlockhash",
        "params":[

        ]
    }
    .to_string();

    // NOTE: You can use Reqwest crate instead to fetch the blockhash but
    // this code shows how to use the browser `fetch` api

    let response_body = FetchReq::new("POST")?
        .add_header("content-type", "application/json")?
        .add_header("Accept", "application/json")?
        .set_body(&options)
        .send(endpoint, window)
        .await?;

    let deser = serde_json::from_str::<RpcResponse<ResponseWithContext<BlockHashResponseValue>>>(
        &response_body,
    )
    .unwrap();

    solana_sdk::hash::Hash::from_str(deser.result.value.blockhash)
        .map_err(|error| WalletError::Op(error.to_string()))
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u8,
    pub result: T,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignaturesResponse {
    pub block_time: Option<i64>,
    pub confirmation_status: Option<String>,
    pub err: Option<TransactionError>,
    pub signature: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseWithContext<O> {
    pub value: O,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAccountResponse {
    pub pubkey: String,
    pub account: Account,
}

impl TokenAccountResponse {
    pub fn mint(&self) -> String {
        self.account.data.parsed.info.mint.to_owned()
    }

    pub fn ata_address(&self) -> String {
        self.pubkey.to_owned()
    }

    pub fn balance(&self) -> String {
        self.account
            .data
            .parsed
            .info
            .token_amount
            .ui_amount_string
            .to_owned()
    }

    pub fn state(&self) -> String {
        self.account.data.parsed.info.state.to_uppercase()
    }
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub data: TokenData,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenAmount {
    pub amount: String,
    pub decimals: u8,
    pub ui_amount: f64,
    pub ui_amount_string: String,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseInfo {
    pub mint: String,
    pub state: String,
    pub token_amount: TokenAmount,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parsed {
    pub info: ParseInfo,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenData {
    pub parsed: Parsed,
}
