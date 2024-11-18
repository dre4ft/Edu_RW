#ifndef CRYPTER_H
#define CRYPTER_H

#include <string>
#include <vector>

// Declaration of the Crypter class
class Crypter {
public:
    // Constructor that initializes the directory path and encryption key
    Crypter(std::string directoryPath, std::string key);
    
    // Public methods for encryption and decryption
    void encrypt();
    void decrypt();
    
    // Destructor
    ~Crypter();

private:
    // Private attributes
    std::string key_;                // Encryption key
    std::string directoryPath_;      // Directory path

    // Private methods
    bool isTargetExtension(const std::string& filePath);       // Checks if the file has a target extension
    std::string encryptAES(const std::string& input);          // Encrypts a string using AES
    std::string decryptAES(const std::string& encryptedText);  // Decrypts an AES-encrypted string
    std::string readFromFile(const std::string& filePath);     // Reads content from a file
    void writeToFile(const std::string& filePath, const std::string& content);  // Writes content to a file
    void clearMemory(void* ptr, size_t size);                  // Clears memory for security reasons
};

#endif // CRYPTER_H
