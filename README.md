# rust-rat
# 🦀 RustRAT – Remote Access Trojan

**RustRAT** is my implementation of a Remote Access Trojan in Rust, designed purely for **educational**, **research**, and **defensive security** purposes. It demonstrates core concepts in networking, asynchronous programming, and client-server architecture with E2EE using the Rust programming language.

> 🚨 **Disclaimer**: This project is intended solely for educational use and **must not** be used for unauthorized access or malicious purposes.
You are responsible for ensuring that you use this tool in compliance with all applicable laws and regulations.  
Unauthorized use of this software to access or control devices without explicit permission is illegal and may result in severe criminal penalties. The developers are not responsible for any misuse or damage caused by this software.  

# Features

End-to-End Encryption: Secure communication between client and server using AES + RSA. :white_check_mark: 

File Transfer: Upload and download files between server and client. :white_check_mark: 

Screen Capture: Grab screenshots of the client’s desktop. :white_check_mark: 

Remote Shell: Execute shell commands on the client machine and receive the output. 🔴

Process Management: List, kill, and spawn processes remotely. 🔴

Keylogging: Capture keystrokes. 🔴

Cross-Platform: Works on Windows, Linux(currently only rat client). 🔴

RataTUI cli interface 🔴

> **Note**: Feature set may expand as the project evolves. Check the source code for the latest commands supported. 

# Architecture

RustRAT follows a classic client-server model:

Server (**rat_server**) listens on a configurable port for incoming connections.

Client (**rat**) connects back to the server, establishing a secure, encrypted channel.

**Server cli** accepts commands and dispatches them to the client.

Asynchronous Rust (**tokio**) powers non-blocking I/O for scalability and performance.

