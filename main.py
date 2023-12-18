from api.lib import Signer

signer = Signer()
wallet = signer.generate_phrase("password")
print(wallet)
