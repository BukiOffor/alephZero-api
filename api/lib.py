import aleph_api as aleph


class Signer:
    
    def __init__(self) -> None:
        pass

    def generate_phrase(self, password):
        return aleph.generate_phrase(password)