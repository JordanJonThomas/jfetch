# JFetch (Demo)
A simple fetching tool, inspired by [neofetch](https://github.com/dylanaraps/neofetch).

![image](https://github.com/user-attachments/assets/d92bac6f-0589-4cb6-8ae5-68da749a5a27)

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
