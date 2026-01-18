# Outlook for Linux
An unofficial Linux desktop wrapper for Microsoft Outlook built with Tauri.

<p align="center">
  <img src="client/src-tauri/icons/Square310x310Logo.png" alt="Threema Chat Analyzer Logo" width="200"/>
</p>

<p align="center">
  <!-- GitHub Actions Badge -->
  <a href="https://github.com/maxiking445/outlook-for-linux/actions/workflows/CI.yml">
    <img src="https://github.com/maxiking445/outlook-for-linux/actions/workflows/CI.yml/badge.svg" alt="Tauri CI Build">
  </a>

  <!-- License + Latest Release Badges -->
  <br>
  <img src="https://img.shields.io/github/license/maxiking445/outlook-for-linux" alt="License">
  <img src="https://img.shields.io/github/v/release/maxiking445/outlook-for-linux" alt="Latest Release">
</p>

## Why Tauri?
This project wraps the existing Outlook web app in a lightweight Linux desktop client.
I choosed it because unlike Electron, Tauri keeps the app very small with less deps and is also easy to build.

## How to use

You can download the Linux version that matches your system from the **Releases** page. Available formats include:

- **.deb** → for Debian, Ubuntu, and derivatives  
- **.rpm** → for Fedora, openSUSE, and other RPM-based distributions  
- **AppImage** → a portable version that works on most Linux distributions



## Install
Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install tauri-cli
``` 
Node
```bash
sudo apt install nodejs npm
``` 
## Start

```bash
cd client
npm run build (once)
npm run tauri:dev
``` 

## Build 
```bash
npm run tauri:build
```

### generate logo
```bash
npx tauri icon Logo_outlook.png
```

## Notes
Under Linux you probably need these additional libs to run it properly.
```bash
sudo apt install libayatana-appindicator3-dev libgtk-3-dev
``` 
