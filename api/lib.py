import aleph_api as aleph
import asyncio 


class Chain:
    
    def __init__(self) -> None:
        self.provider = "wss://aleph-zero-rpc.dwellir.com:443"
        pass

    def generate_phrase(self, password):
        return aleph.generate_phrase(password)
    
    def get_balance(self,address,provider="wss://aleph-zero-rpc.dwellir.com:443"):
        balance = asyncio.run(self._get_balance(address,provider))
        return balance
    
    async def _get_balance(self,address,provider):
        balance = await aleph.get_account_balance(address,provider)
        return (balance)

    def get_wallet_address(self,phrase):
        addr = aleph.get_account_details(phrase)
        return addr[0]


    def send_azero(self,phrase,receiver,amount,provider="wss://aleph-zero-rpc.dwellir.com:443"):
        tx_hash = asyncio.run(self._send_azero(phrase,receiver,amount,provider))
        return tx_hash


    async def _send_azero(self,phrase,receiver,amount,provider):
        hash = await aleph.sign_and_transfer_azero(provider,phrase,receiver,amount)
        return hash

    async def _getBlockHash(self,index, provider):
        block_hash = await aleph.get_block_hash(provider,index)
        return str(block_hash)
    
    def get_block_hash(self,index,provider="wss://aleph-zero-rpc.dwellir.com:443"):
        block_hash = asyncio.run(self._getBlockHash(int(index),provider))
        return block_hash
    
    async def _getBlockNumber(self,hash, provider):
        block_num = await aleph.get_block_number(provider,hash)
        return str(block_num)
    
    def get_block_number(self,hash,provider="wss://aleph-zero-rpc.dwellir.com:443"):
        block_number = asyncio.run(self._getBlockNumber(hash,provider))
        return block_number