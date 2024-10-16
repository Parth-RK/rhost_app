









from cryptography.fernet import Fernet

# Generate a key and write it to a file
key = Fernet.generate_key()
with open('secret.key', 'wb') as key_file:
    key_file.write(key)

# Load the key and encrypt a file
def encrypt_file(file_name):
    with open('secret.key', 'rb') as key_file:
        key = key_file.read()
    fernet = Fernet(key)

    with open(file_name, 'rb') as file:
        original = file.read()

    encrypted = fernet.encrypt(original)

    with open(f"{file_name}.enc", 'wb') as encrypted_file:
        encrypted_file.write(encrypted)

encrypt_file('example.txt')
