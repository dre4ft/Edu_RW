#include <iostream>
#include "crypter.h"

int main(int argc, char *argv[]) {
    if (argc < 4) {
        std::cerr << "Usage: " << argv[0] << " <command> <key> <directory>" << std::endl;
        return 1;
    }

    Crypter* crypter = new Crypter(argv[3],argv[2]);

    std::string command = argv[1];
    if (command == "encrypt") {
        crypter->encrypt();
    } else if (command == "decrypt") {
        crypter->decrypt();
    } else {
        std::cerr << "Unknown command: " << command << std::endl;
        return 1;
    }
    delete crypter;
    return 0;
}