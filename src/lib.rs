use std::str::FromStr;
use std::result::Result;
use aleph_client::contract::ConvertibleValue;
use aleph_client::contract_transcode::Value;
use pyo3::prelude::*;
use aleph_client::{RawKeyPair, Pair, KeyPair, Connection, SignedConnection, AsSigned, AccountId};
use aleph_client::pallets::balances::BalanceUserApi;
// use aleph_client::api;
// use aleph_client::api::runtime_types::sp_runtime::multiaddress::MultiAddress;



#[pyfunction]
fn generate_phrase(password:&str) -> PyResult<String> {
    let key = RawKeyPair::generate_with_phrase(Some(password));
    //let address = key.0.public().to_string(); 
    Ok(key.1)
}

#[pyfunction]
fn get_account_details(phrase:&str) -> PyResult<(String,String)>{
    let signer = KeyPair::from_str(phrase).expect("signer could not be initialized");
    let account_id = signer.account_id().to_owned().to_string();
    let kpub = signer.signer().public().to_string();
    Ok((account_id, kpub))

}

#[pyfunction]
fn get_account_id(receiver:String)->PyResult<String>{
    let dest: AccountId = ConvertibleValue(Value::Literal(receiver)).try_into().expect("Was unable to get Account Id");
    Ok(dest.to_string())

}
                               
pub async fn get_signer(address: &str, phrase:&str ) -> Result<SignedConnection,()> {
    let rpc: Connection = Connection::new(address).await;
    let signer = KeyPair::from_str(phrase).expect("signer could not be initialized");
    let wallet = SignedConnection::from_connection(rpc, signer);
    Ok(wallet)                                                                                                                    
}

pub async  fn send_transaction(signer:SignedConnection,receiver:String, amount:u128) -> Result<String,()>{
    let status = aleph_client::TxStatus::Finalized;
    let dest: AccountId = ConvertibleValue(Value::Literal(receiver)).try_into().expect("Was unable to get Account Id");
    let tx = signer.as_signed().transfer(dest, amount, status).await;
    let tx_hash = tx.unwrap().tx_hash.to_string();
    // let tx = api::tx()
    // .balances()
    // .transfer(MultiAddress::Id(dest), amount);
    Ok(tx_hash)
}


/// A Python module implemented in Rust.
#[pymodule]
fn aleph_api(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(generate_phrase, m)?)?;
    m.add_function(wrap_pyfunction!(get_account_details, m)?)?; 
    m.add_function(wrap_pyfunction!(get_account_id, m)?)?;   
  

    Ok(())
}
