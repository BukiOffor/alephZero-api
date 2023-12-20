from api.lib import Chain
# import aleph_api as api
# import asyncio

chain = Chain()
seed = "elevator notable sword phrase doctor tell nice arrow melody judge bleak worth"

# generate phrase
#phrase = chain.generate_phrase("password")
#print(phrase)

# generate wallet address
# addr = chain.get_wallet_address(phrase)
# print(addr)

# get balance of address
# balance = chain.get_balance(addr);
# print(balance)

# get block hash
#block_hash = chain.get_block_hash(66335838)
#print(block_hash)

# get block number
#block_number = chain.get_block_number("0xa3c2290488a124a82a7fd57d7c54edaebd6b214d2330e6510291b40fc8887c9a")
#print(block_number)

# sign a statement with your keys
statement = "A legal binding contract ......"
signature = chain.sign(seed, statement)
print(signature)

# get public keys
kpub = chain.get_public_key(seed)
print(kpub)

# verify signed statement
verified = chain.verify_sig(signature,statement,kpub)
print(verified)
