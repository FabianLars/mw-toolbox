# Development

If you're on Windows, make sure you have WebView2 installed.

1. Start the react dev server: "yarn start" (Optional, because 2. runs this command too)
2. Start tauri in dev mode: "yarn tauri dev"

# Build

0. Make sure the gui/build/ folder is present. If it's not, create it or let 1. handle it. (2. crashes without it as of now) 
1. Build react app: "yarn run build" (Optional if 0. is taken care of, because 2. runs this command too)
2. Build tauri app: "yarn tauri build"
