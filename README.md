# Edu_RW
Cybersecurity MSc final project study : 
A basic Ransomware levraging CVE-2023-38831


## **/!\Disclaimer/!\**

This project is intended **exclusively for educational purposes** and as a demonstration of technical competency in cybersecurity. The ransomware built here is not operational as a malicious tool. It was never intended for deployment in real-world environments and lacks the resilience or features of functional ransomware used by threat actors.  

Additionally, the project underscores the ethical responsibility tied to such work. Developing malware, even for research purposes, requires strict adherence to legal and ethical guidelines. Unauthorized use, testing, or distribution of such tools is illegal and unethical, and I strongly condemn any such activity.  

By sharing this proof-of-concept responsibly, I aim to contribute to cybersecurity education and research, helping organizations better understand and defend against emerging threats.

---

### **1. Introduction**

This project aims to develop a proof-of-concept (PoC) ransomware that demonstrates realistic attack techniques using concepts and methodologies observed in contemporary ransomware campaigns. The implementation leverages modern vulnerabilities and best practices in attack-chain development, providing a thorough exploration of the lifecycle of ransomware.

One critical design decision was the exploitation of **CVE-2023-38831**, a vulnerability that facilitates Remote Code Execution (RCE) via `.rar` archives, making it an ideal attack vector for payload delivery. Additionally, the ransomware employs a **dual-encryption scheme**, where victim files are encrypted using **AES-256** and then further secured by encrypting the AES key and algorithm with **RSA-2048**. This multi-layered encryption strategy minimizes the likelihood of key recovery via reverse engineering.

At a high level, the attack chain begins when the victim interacts with a malicious `.rar` archive. Inside the archive is a concealed batch script that, upon execution, downloads and runs the ransomware payload from a **command-and-control (C2) server**. This payload orchestrates the encryption workflow by deploying the AES encryption module (*Crypter*), encrypting files, and securely handling cryptographic keys. Once these processes are complete, the ransomware presents the victim with a **ransom note interface**, demanding payment to initiate the decryption process.

---

### **2. Development**

#### **2.1 Exploitation of CVE-2023-38831**

The ransomware’s delivery mechanism exploits **CVE-2023-38831**, a vulnerability in versions of WinRAR prior to 6.23. The flaw arises from improper filename handling, where a `.cmd` script within the archive is executed instead of the file the victim intends to open. This behavior is due to a conflict in handling files and folders with identical names.

The exploit is used to execute a **dropper script** that initiates the infection chain. The script first opens the legitimate file to avoid raising suspicion. It then proceeds to download the ransomware payload from the **C2 infrastructure** and executes it silently. Steps were taken to suppress visual indicators (e.g., command-line windows) during execution, increasing stealth and evasion from detection mechanisms.

---

#### **2.2 C2 Infrastructure**

The **command-and-control (C2)** infrastructure is hosted on a Debian-based virtual machine, utilizing an **Apache web server** paired with a Flask-based Python application. The infrastructure is designed with operational security in mind, employing obfuscation techniques and secure communication channels.

Key endpoints in the C2 server include:

1. **Payload Delivery Endpoint**  
   A pseudo-randomized endpoint path minimizes discovery and enumeration risks. This endpoint serves the initial ransomware payload.

2. **Synchronization Endpoint**  
   Using the HTTP POST method, this endpoint facilitates synchronization between the victim’s system and the C2. Upon receiving a request, the C2 generates a unique **session token** linked to the victim's IP address, storing this information in a database. Rate-limiting policies and token reuse restrictions enhance security and prevent exploitation. Concurrently, an **RSA key pair** is generated and associated with the session.

3. **Decryption Key Retrieval Endpoint**  
   This endpoint validates tokens before responding to requests. Depending on the identifier (`id`) included in the request, it either:
   - Returns an archive containing *Crypter*, the AES key, and the RSA public key (if `id` = 201).
   - Delivers the RSA private key for file decryption (if `id` = 202).  

The server operates as a **persistent service**, ensuring availability without manual intervention. This setup aligns with best practices for maintaining operational uptime and resilience.

---

#### **2.3 Payload Development**

The payload serves as the **primary orchestrator** of the ransomware’s functionality, handling tasks from key exchange to file encryption and victim interaction. **Rust** was selected for its performance, cross-platform compatibility, and memory safety features.

Key payload functions include:

1. **C2 Communication**  
   The payload manages secure communication with the C2 server via HTTP requests, ensuring proper token synchronization and file downloads. Rust's threading capabilities were critical for managing non-blocking HTTP requests and maintaining operational efficiency.

2. **File Encryption and Key Management**  
   The payload integrates **RSA** for secure key handling and deploys the AES module (*Crypter*) for encrypting victim files. RSA encryption functions were implemented to handle files in 245-bit blocks, balancing security and efficiency.

3. **GUI Implementation**  
   A minimalistic **ransom note interface** is included, guiding the victim through the ransom payment process. The GUI includes instructions, a text field for entering the payment transaction ID, and a button to trigger the decryption process.

---

#### **2.4 AES-256 Encryption Module: *Crypter***

The AES-256 module (*Crypter*) is the ransomware’s primary data encryption tool. Developed in **C++**, it was optimized for speed, cross-platform compatibility, and minimal memory footprint. After encountering compilation issues with the Crypto++ library on Windows, a custom implementation of AES-256 (sourced and adapted from Danillo Treffiletti’s open-source repository) was used.

**Operational Details:**
- *Crypter* operates in two modes: `"encrypt"` and `"decrypt"`.
- In encryption mode, it traverses a specified directory, targeting files with pre-defined extensions (e.g., `.docx`, `.png`, `.txt`) to focus on personal and sensitive data while avoiding critical system files.
- Files are read byte-by-byte, encrypted, and then rewritten with a distinguishing extension (e.g., `.kawaii`). Decryption follows the reverse process, restoring files to their original state and removing the appended extension.

The module employs **AES-256 in ECB mode**, chosen for its simplicity and speed. Unlike CBC mode, ECB does not require Initialization Vectors (IVs), reducing complexity during implementation. Memory management features in C++ were leveraged to clear the AES key from memory after use, minimizing the risk of recovery through forensic analysis.

---

### **3. Conclusion**

Through the development of this proof-of-concept ransomware, I enhanced several key competencies essential to the field of cybersecurity. These include:  

1. **Exploitation of Vulnerabilities:**  
   I gained hands-on experience in analyzing and leveraging CVEs, particularly **CVE-2023-38831**, to understand how real-world attackers use vulnerabilities to initiate attack chains.  

2. **Command-and-Control (C2) Development:**  
   I built a robust C2 infrastructure, improving my knowledge of secure communication protocols, operational security, and endpoint design to replicate adversarial tools effectively.  

3. **Advanced Cryptography:**  
   Implementing encryption schemes using **AES-256** and **RSA-2048** deepened my understanding of modern cryptographic methods, their practical application, and secure key management techniques.  

4. **Cross-Platform Payload Development:**  
   Developing a payload in **Rust** taught me how to create efficient, cross-platform binaries while addressing challenges like threading and memory safety, which are critical in malware simulation and Red Team engagements.  

5. **Red Team Tooling:**  
   This project sharpened my skills in building realistic tools that simulate adversarial behavior, from initial exploitation to end-user interaction. These tools are essential for identifying gaps in defensive security measures.

