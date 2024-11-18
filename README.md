# **Edu_RW: Ransomware Anatomy 101**  

## 🚨 **Disclaimer** 🚨  
**This project is purely for educational and research purposes.**  
The ransomware created here is a **non-functional proof-of-concept (PoC)** meant to showcase how ransomware works, not to be used maliciously.  
- It lacks the robustness and stealth of real-world malware.  
- Unauthorized use, testing, or distribution of tools like this is **illegal and unethical**.  
- The goal? **Learning to defend against ransomware by understanding how it works.**  

---

## **1. Introduction**  

**Edu_RW** is my MSc final project: a hands-on study of ransomware mechanics. It explores how ransomware attacks are designed, built, and executed, focusing on **CVE-2023-38831** as the entry point.  

Here’s the big picture:  
1. The ransomware tricks users into opening a malicious `.rar` file.  
2. A script inside secretly delivers the payload.  
3. Files are encrypted with **AES-256**, and the key is locked up using **RSA-2048**.  
4. Victims are greeted with a ransom note, demanding payment for decryption.  

This project doesn’t just simulate the attack—it breaks it down step by step, showing how the techniques can be defended against in real-world scenarios.  

---

## **2. Project Highlights**  

### **2.1 Attack Chain Breakdown**  
- **Step 1: Exploitation**  
   Leveraging **CVE-2023-38831**, a vulnerability in WinRAR (pre-v6.23), the attack uses a `.cmd` script disguised in a `.rar` archive.  
- **Step 2: Payload Delivery**  
   The script downloads and runs the ransomware payload silently.  
- **Step 3: File Encryption**  
   Files are encrypted using **AES-256**, and the encryption key is secured using **RSA-2048**.  
- **Step 4: Ransom Note**  
   A minimal GUI displays instructions for paying the ransom to unlock files.  

---

### **2.2 Encryption: The Power of AES-256 & RSA-2048**  
**How it works:**  
- **AES-256**: Encrypts the victim’s files for speed and security.  
- **RSA-2048**: Encrypts the AES key to make recovery almost impossible without the ransom.  

This dual-layer encryption protects the ransomware from being easily reversed.  

**Crypter Module:**  
- Written in **C++**, the AES module encrypts files efficiently.  
- Targets specific file types (e.g., `.docx`, `.png`, `.txt`) while avoiding critical system files.  
- Uses the **ECB mode** for simplicity, and clears keys from memory after use to resist forensic analysis.  

---

### **2.3 Command-and-Control (C2) Server**  
The **C2 server** orchestrates the attack, hosted on a **Debian-based virtual machine** with:  
- **Apache Web Server** for payload delivery.  
- **Flask-based Python App** to manage endpoints securely.  

Endpoints include:  
1. **Payload Delivery**: Dynamically delivers the ransomware payload via randomized URLs.  
2. **Session Management**: Handles victim connections using unique tokens and generates RSA keys for encryption.  
3. **Key Retrieval**: Provides the decryption key once the ransom is “paid.”  

**Security Features:**  
- Obfuscation techniques to evade detection.  
- Rate limiting to prevent exploitation.  

---

### **2.4 Built With Rust for Payloads**  
The ransomware payload is the attack's heart, built in **Rust** for:  
- **Performance**: Fast and reliable across platforms.  
- **Memory Safety**: Reduces vulnerabilities attackers (or defenders!) could exploit.  
- **Efficiency**: Handles secure file encryption and stealthy C2 communication.  

---

## **3. Learning Goals**  

This project sharpened my skills in several areas critical to cybersecurity:  
1. **Understanding Vulnerabilities**  
   - Deep dive into **CVE-2023-38831**, including how attackers exploit software flaws.  
2. **Building C2 Infrastructure**  
   - Designing secure, scalable, and stealthy command-and-control systems.  
3. **Advanced Cryptography**  
   - Implementing **AES-256** and **RSA-2048** to secure file data and encryption keys.  
4. **Malware Development**  
   - Crafting realistic ransomware simulations to study how attackers evade detection.  
5. **Red Team Tactics**  
   - Using offensive tools to identify and close gaps in defensive measures.  

---

## **4. Development Status**  
🚧 **What’s working?**  
- Proof-of-concept payload and C2 infrastructure.  
🚀 **What’s next?**  
- Adding more stealth features for evasion.  
- Expanding encryption support for additional file types.  
- Enhancing C2 server robustness.  

---

## **5. Ethical Considerations**  

Creating ransomware—even as a proof-of-concept—comes with **serious ethical responsibilities**. This project was developed with strict adherence to legal guidelines, with one goal: **education and defense**.  

If you’re here to learn, welcome aboard. If you’re here for malicious purposes, you’re in the wrong place.  

---

## **6. Shoutouts**  
Thanks to the open-source community for inspiration and tools, including:  
- **Danillo Treffiletti’s AES-256 implementation**: https://github.com/Urban82/Aes256  

---

## **7. Final Thoughts**  
Edu_RW is an opportunity to explore the complexities of ransomware in a safe, controlled way. By studying how attacks work, we can build stronger defenses—and keep the bad guys at bay.  

**Remember**: Knowledge is power, but it’s also responsibility. Use it wisely.  

