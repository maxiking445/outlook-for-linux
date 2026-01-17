# outlook-for-linux
An unofficial Linux desktop wrapper for Microsoft Outlook built with Tauri.

## Why Tauri?
This project wraps the existing Outlook web app in a lightweight Linux desktop client.
Unlike Electron, which tends to produce very large apps, Tauri keeps the app small and easy to build.


## Install

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli

## Start

cd client
npm run tauri


## build 

npm run tauri:build