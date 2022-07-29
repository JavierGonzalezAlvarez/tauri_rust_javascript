# Tauri - rust & js

## install in linux
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

## create an app
$ npm create tauri-app
name of the app: desktop
Select: Vanilla.js

$ cd desktop
$ npm install

## launch frontend
$ dist/npm run tauri dev

## launch backend
$ src-tauri/cargo run


## place the index.html
dist/index.html

## to add
tauri.conf.json > build > "withGlobalTauri": true

## examples of commands
https://github.com/tauri-apps/tauri/blob/dev/examples/commands/main.rs