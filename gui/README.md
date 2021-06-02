# Development

Be sure to have all of tauri's dependencies installed: [linux](https://tauri.studio/en/docs/getting-started/setup-linux/) / [windows](https://tauri.studio/en/docs/getting-started/setup-windows/) / [macOS](https://tauri.studio/en/docs/getting-started/setup-macos/)

<!--1. Start the react dev server: "pnpm start"-->

Start tauri in dev mode: "pnpm tauri dev". This automatically runs "pnpm start" to start up vite's dev server. (Note: edit "before(Dev|Build)Command" in tauri.conf.json if you're not using pnpm)

# Build

<!--1. Build react app: "pnpm run build" (Optional, because 2. runs this command too)-->

Run "pnpm tauri build". This automatically runs "pnpm build" to start vite's build process. (Note: edit "before(Dev|Build)Command" in tauri.conf.json if you're not using pnpm)
