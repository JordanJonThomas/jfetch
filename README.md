# JFetch (Demo)
A simple fetching tool, inspired by [neofetch](https://github.com/dylanaraps/neofetch).

![image](https://github.com/user-attachments/assets/910adf6b-df22-4ec3-9c38-3fe1d6e49ece)

- Displays basic system information (OS, Kernel, Uptime, etc)
- Displays ascii art
- More coming soon!

## Usage
Clone and build the repository
```sh
git clone https://github.com/JordanJonThomas/jfetch.git
cd jfetch
cargo build --release
```
#### Option A. 
Run executable directly. 
```sh
.\target\release\jfetch.exe
```
#### Option B.
Add to shell start up profile for automatic launch! (example for powershell)
```sh
Add-Content -Path $PROFILE -Value '& "C:\path\to\jfetch\target\release\jfetch.exe"'
```
