use std::str::FromStr;
use aleph_client::api::runtime_types::sp_core::ed25519::Signature;
use aleph_client::api::runtime_types::sp_core::ed25519::Public;
use aleph_client::contract::ConvertibleValue;
use aleph_client::contract_transcode::Value;
use aleph_client::sp_core::H256;
use pyo3::prelude::*;
use aleph_client::{RawKeyPair, Pair, KeyPair, Connection, SignedConnection, AsSigned, AccountId};
use aleph_client::pallets::balances::BalanceUserApi;
//use aleph_client::AsConnection;
use aleph_client::utility::BlocksApi;
use aleph_client::pallets::system::SystemApi;
use pyo3::types::PyTuple;
use std::process;


// generate a seed phrase for a user
#[pyfunction]
fn generate_phrase(password:&str) -> PyResult<String> {
    let key = RawKeyPair::generate_with_phrase(Some(password));
    Ok(key.1)
}

// sign a statement
#[pyfunction]
fn sign<'a>(py:Python<'a> ,phrase:String, message:&[u8]) -> PyResult<&'a PyTuple> {
    let tuple: &'a PyTuple;
    let signer = KeyPair::from_str(&phrase).expect("signer could not be initialized");
    let sig = signer.signer().sign(message).0;
    tuple = PyTuple::new(py, sig);
    Ok(tuple)
}

fn verify_signature(sig:[u8; 64],message:&[u8],key:[u8;32],phrase:&str){
    let signer = KeyPair::from_str(phrase).expect("signer could not be initialized");
    let pubkey: Public = Public(key);
    let sig = Signature(sig);
    let verified = <RawKeyPair as Pair>::verify(&sig, message, &pubkey);

    
}


// generate a wallet address and account id for user
#[pyfunction]
fn get_account_details(phrase:&str) -> PyResult<(String,String)>{
    let signer = KeyPair::from_str(phrase).expect("signer could not be initialized");
    let account_id = signer.account_id().to_owned().to_string();
    let kpub = signer.signer().public().to_string();
    Ok((account_id, kpub))
}

// asynchronous function to get the user balance
async fn get_user_balance(wallet_address:String, rpc_url: &str)->Result<String,()>{
    let account: AccountId = ConvertibleValue(Value::Literal(wallet_address)).try_into().expect("Was unable to get Account Id");
    let rpc: Connection = Connection::new(rpc_url).await;
    //let rpc = rpc.as_client().tx().sign_and_submit_default(call, signer);
    let balance = rpc.get_free_balance(account, None).await;
    Ok(balance.to_string())
}

// gets the block has, given a block number
#[pyfunction]
pub fn get_block_hash(py: Python, rpc_url: String, block:u32) -> PyResult<&PyAny> {    
    pyo3_asyncio::async_std::future_into_py(py, async move {
        let rpc: Connection = Connection::new(&rpc_url).await;
        let block_hash = rpc.get_block_hash(block)
        .await
        .expect("Block hash could not be unwrapped")
        .unwrap();
    Ok(block_hash.to_string())
    })
}
#[pyfunction]
pub fn get_block_number(py: Python, rpc_url: String, value:String) -> PyResult<&PyAny> { 
    let block = H256::from_str(&value).expect("expected hash got unknown") ;  
    pyo3_asyncio::async_std::future_into_py(py, async move {
        let rpc: Connection = Connection::new(&rpc_url).await;
        let block_num = rpc.get_block_number(block)
        .await
        .expect("Block hash could not be unwrapped");
    if let Some(num) = block_num {
        Ok(num)
    } else {
        process::exit(0);
    }
    })
}

// get the user balance of a user
#[pyfunction]
pub fn get_account_balance(py: Python, wallet_address:String, rpc_url: String) -> PyResult<&PyAny> {
    pyo3_asyncio::async_std::future_into_py(py, async move {
        let balance = get_user_balance(wallet_address, rpc_url.as_str())
        .await
        .expect("something went wrong with getting user balance");
    Ok(balance)
    })
}

// conects a wallet the the blockchain
pub async fn get_signer(rpc_url: &str, phrase:&str ) -> Result<SignedConnection,()> {
    let rpc: Connection = Connection::new(rpc_url).await;
    let signer = KeyPair::from_str(phrase).expect("signer could not be initialized");
    let wallet = SignedConnection::from_connection(rpc, signer);
    Ok(wallet)                                                                                                                    
}

// transfers funds to a wallet
pub async fn transfer_azero(signer:SignedConnection, receiver:String, amount:u128) -> Result<String,()>{
    let status = aleph_client::TxStatus::Submitted;
    let dest: AccountId = ConvertibleValue(Value::Literal(receiver)).try_into().expect("Was unable to get Account Id");
    let tx = signer.as_signed().transfer(dest, amount, status).await;
    let tx_hash = tx.unwrap().tx_hash.to_string();
    Ok(tx_hash)
}

#[pyfunction]
pub fn sign_and_transfer_azero(py: Python, rpc_url: String, phrase:String, receiver:String, amount:u128) -> PyResult<&PyAny> {    
    pyo3_asyncio::async_std::future_into_py(py, async move {
        let signer = get_signer(&rpc_url,&phrase)
            .await
            .expect("Signer could not be initialized");
        let tx_hash = transfer_azero(signer, receiver, amount)
            .await
            .expect("transaction could not be sent");
        Ok(tx_hash)
    })
}


/// A Python module implemented in Rust.
#[pymodule]
fn aleph_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_phrase, m)?)?;
    m.add_function(wrap_pyfunction!(get_account_details, m)?)?; 
    m.add_function(wrap_pyfunction!(get_account_balance, m)?)?; 
    m.add_function(wrap_pyfunction!(sign_and_transfer_azero, m)?)?;   
    m.add_function(wrap_pyfunction!(get_block_hash, m)?)?;  
    m.add_function(wrap_pyfunction!(get_block_number, m)?)?;
    m.add_function(wrap_pyfunction!(sign, m)?)?;   
   
 

  
    Ok(())
}
