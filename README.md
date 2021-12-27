## voiceBanRust

Steps:

# 1.Download and install "Build Tools for Visual Studio:"

You can get it at this link: https://aka.ms/buildtools.
Run the installation file: vs_buildtools.exe.
Ensure the "Windows 10 SDK" component is included when installing the Visual C++ Build Tools.
Restart your computer.

# 2.Install Rust:

Detailed instructions are provided by the Rust Book and a quick reference is available at https://rustup.rs/ .

Download from: https://www.rust-lang.org/tools/install.
Run the installation file: rustup-init.exe for 32 or 64 bis as appropriate.
It shouldn't prompt you to install vs_buildtools since you did it in step 1.
Choose "Default Installation."
To get started, you need Cargo's bin directory (%USERPROFILE%\.cargo\bin) in your PATH environment variable. Future applications will automatically have the correct environment, but you may need to restart your current shell.

# 3.Run these commands in Command Prompt (CMD) to set up your Wasm Build Environment:

rustup update nightly
rustup update stable
rustup target add wasm32-unknown-unknown --toolchain nightly

# 4.Install LLVM: https://releases.llvm.org/download.html
# 5.Install OpenSSL with vcpkg using PowerShell:

mkdir C:\Tools
cd C:\Tools
git clone https://github.com/Microsoft/vcpkg.git --depth=1
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg.exe install openssl:x64-windows-static

# 6.Add OpenSSL to your System Variables using PowerShell:
$env:OPENSSL_DIR = 'C:\Tools\vcpkg\installed\x64-windows-static'
$env:OPENSSL_STATIC = 'Yes'
[System.Environment]::SetEnvironmentVariable('OPENSSL_DIR', $env:OPENSSL_DIR, [System.EnvironmentVariableTarget]::User)
[System.Environment]::SetEnvironmentVariable('OPENSSL_STATIC', $env:OPENSSL_STATIC, [System.EnvironmentVariableTarget]::User)

# 7.Install cmake: 
https://cmake.org/download/

# 8. Install make

This can be done using Chocolatey. First you need to install the Chocolatey package manager: https://chocolatey.org/install
Once Chocolatey installed you can install make:

choco install make


# 9. Once you have done the above you will also need to run:

rustup component add rust-src --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly

# 10.Installing The Substrate Contracts Node:
cargo install contracts-node --git https://github.com/paritytech/substrate-contracts-node.git --tag v0.3.0 --force --locked

# 11.Run this command in the project root folder
cargo +nightly update

# 12.Run this command to build the contract
cargo +nightly contract build

# 13.you need to interact with canvas UI to deploy your contract build

https://paritytech.github.io/canvas-ui/#/

# 14.You can find the abi of the contract in the target folder:
target->ink->login.contract put that file in the canvas that will deploy your contract 
