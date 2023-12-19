from api.lib import Chain


wallet = Chain()

phrase = wallet.generate_phrase("password")
print(phrase)
addr = wallet.get_wallet_address(phrase)
print(addr)
balance = wallet.get_balance(addr);
print(balance)