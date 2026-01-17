# outlook-for-linux
An unofficial Linux desktop wrapper for Microsoft Outlook built with Tauri.

## Why Tauri?
This project wraps the existing Outlook web app in a lightweight Linux desktop client.
I choosed it because unlike Electron, Tauri keeps the app very small with less deps and is also easy to build.


## Install

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli

## Start

cd client
npm run tauri


## build 

npm run tauri:build

### generate logo

npx tauri icon Logo_outlook.png

## deps

sudo apt install libayatana-appindicator3-dev libgtk-3-dev