from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric import rsa
from cryptography.hazmat.backends import default_backend

class RSAKeyGenerator:
    def __init__(self):
        # Initialize attributes to store private and public keys
        self.private_key = None
        self.public_key = None

    def generate_keys(self, private_key_path, public_key_path):
        # Generate a 2048-bit RSA key pair
        private_key = rsa.generate_private_key(
            public_exponent=65537,  # Public exponent
            key_size=2048,          # Key size in bits
            backend=default_backend()  # Default cryptographic backend
        )

        # Retrieve the public key from the private key
        public_key = private_key.public_key()

        # Convert the keys to PEM (Privacy-Enhanced Mail) format
        private_pem = private_key.private_bytes(
            encoding=serialization.Encoding.PEM,  # Encode in PEM
            format=serialization.PrivateFormat.TraditionalOpenSSL,  # Traditional OpenSSL format
            encryption_algorithm=serialization.NoEncryption()  # No encryption for the private key
        )
        public_pem = public_key.public_bytes(
            encoding=serialization.Encoding.PEM,  # Encode in PEM
            format=serialization.PublicFormat.SubjectPublicKeyInfo  # Subject Public Key Info format
        )

        # Store the keys in text files
        with open(private_key_path, 'wb') as private_key_file:
            private_key_file.write(private_pem)  # Write the private key to a file
        with open(public_key_path, 'wb') as public_key_file:
            public_key_file.write(public_pem)  # Write the public key to a file

        # Convert the keys to strings and store them in the class attributes
        self.private_key = private_pem.decode()  # Convert the private key to a string
        self.public_key = public_pem.decode()  # Convert the public key to a string
