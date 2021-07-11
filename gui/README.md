# Development

Be sure to have all of tauri's dependencies installed: [linux](https://tauri.studio/en/docs/getting-started/setup-linux/) / [windows](https://tauri.studio/en/docs/getting-started/setup-windows/) / [macOS](https://tauri.studio/en/docs/getting-started/setup-macos/)

1. Start the react dev server: "pnpm|yarn|npm start"
2. Start tauri in dev mode: "pnpm|yarn|npm tauri dev"

# Build

<!--1. Build react app: "pnpm run build" (Optional, because 2. runs this command too)-->

Run "pnpm|yarn|npm tauri build". This automatically runs "pnpm build" to start vite's build process. (Note: edit "beforeBuildCommand" in tauri.conf.json if you're not using pnpm)
