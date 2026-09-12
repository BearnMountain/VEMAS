# Vivado Environment Manager on Silicon Macos
This is a tool for installing Vivado on Arm-based Apple
Silicon MacOS. This 

# Features


# Installation
- git clone this repo
- `caffeinate -dim zsh setup.sh`
- `echo 'alias VEM="$HOME/path/to/repo/VEM/scripts/build/release/VEM"' >> ~/.zshrc`
- `docker compose up --build`

## Usage
For startup, use: 
``` bash
docker compose -f /path/to/compose.yaml up
# or from inside the install
docker compose up
```




## Notes

