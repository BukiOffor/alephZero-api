from api.lib import Chain
# import aleph_api as api
# import asyncio

chain = Chain()

phrase = chain.generate_phrase("password")
print(phrase)
# addr = chain.get_wallet_address(phrase)
# print(addr)
# balance = chain.get_balance(addr);
# print(balance)
#block_hash = chain.get_block_hash(66335838)
#print(block_hash)
#block_number = chain.get_block_number("0xa3c2290488a124a82a7fd57d7c54edaebd6b214d2330e6510291b40fc8887c9a")
#print(block_number)
statement = "A legal binding contract ......"
signature = chain.sign(phrase, statement)
print(signature)
