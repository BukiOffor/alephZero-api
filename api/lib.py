import aleph_api as aleph
import asyncio 


class Chain:
    
    def __init__(self) -> None:
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


    def send_transaction():
        pass

    