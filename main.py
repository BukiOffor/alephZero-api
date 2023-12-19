from api.lib import Chain


chain = Chain()

phrase = chain.generate_phrase("password")
print(phrase)
addr = chain.get_wallet_address(phrase)
print(addr)
balance = chain.get_balance(addr);
print(balance)