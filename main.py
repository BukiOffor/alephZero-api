from api.lib import Signer
import aleph_api as api
import asyncio 


main_phrase = "force slight news atom long toward stumble sand coyote roof father flight"
second_wallet = "measure snow merge shove sample naive gown cheese garlic evolve unknown chuckle"


provider = "wss://aleph-zero-rpc.dwellir.com:443"
acc_details = api.get_account_details(second_wallet)
async def get_balance():
    balance = await api.get_account_balance(acc_details[0],provider)
    return (balance)

balance = asyncio.run(get_balance())
print(balance)

