


# ALEPH-API

A Python Api that interacts with the Aleph Zero blockchain


## Installation
### Prerequisite

* Rust
* Maturin
* Python > 3.8

Enter the base directory of the project and activate the virtual environment with the following command and install the required python packages.

```bash
source env/bin/activate
pip install -r requirements.txt
```

To compile the code, you can run the following command. This command builds a python wheel for the rust code.
```bash
maturin develop
```

## Usage
You can create a new python file in the base folder or you can edit the `main.py` in the base folder to run your code.
`api/lib.py` contains a python module that interacts with the aleph zero blockchain. To use the module, you can import the `Chain` class from api/lib.py. 

```python
from api.lib import Chain
```
You can generate a seed phrase programmatically by running. The ```generate_phrase()``` method takes a password for the phrase.

```python
chain = Chain()
password = "horse-staple-zenith"
phrase = chain.generate_phrase(password)
print(phrase)
>>> unable brief apple enhance wool host join service sample kiss blossom iron
```

To generate the address of a phrase, we can call the ```get_wallet_address()```  method passing the seed phrase as an arguement.
```python
phrase = "unable brief enhance .... "
address = chain.get_wallet_address(phrase)
print(address)
>>> 5G6LVo8aJmFiXTdTioDDt49ZseJ5eh9UQRga2MYf1hzdyhUJ
```
To check a wallet address, we can use the ```get_balance()``` exposed by the api. This method takes an address and a provider. if a provider is not given, it uses the default public mainnet provider.

```python
addr = "5G6LVo8aJmFiXTdTioDDt49ZseJ5eh9UQRga2MYf1hzdyhUJ"
provider = "wss://aleph-zero-rpc.dwellir.com:443"
balance = chain.get_balance(addr,provider)
print(balance)
>>> 0
```
To sign a statement with your secret key, we can use the sign method exposed by the api that takes a statement and seed, then returns a signature in bytes.
```python
statement = "A legal binding contract ......"
signature = chain.sign(phrase, statement)
print(signature)
>>> (0,210,166,165,128,235 ......)
```
Also a signed statement can be verified to be `True` or `False` using the `verify_sig()` method
```python
verified = chain.verify_sig(signature,statement,kpub)
print(verified)
>>> True
```
## OTHER METHOD'S IMPLEMENTED 

- `get_block_hash()` : gets block has when given a block number 
- `get_block_number()` : get block number when given a hash
- `sign_and_transfer_azero()` : transfers azero to a given wallet address
- `get_public_key()` : generates the given public key in bytes, this is useful for verifying signatures.