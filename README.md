# goal.wallpaper

`goal.wallpaper` turns your desktop into a productivity dashboard by converting weekly goals, daily tasks, and notes into a custom wallpaper.

## What problem this solves

Many people lose focus because their goals are scattered across apps, sticky notes, and browser tabs. This app keeps your priorities visible all day by placing your goals directly on your desktop wallpaper.

## What it does

- Collects `Weekly Goals`, `Daily Tasks`, and `Notes` in one dashboard
- Saves your list to `data.json`
- Renders a polished 1920x1080 wallpaper image from a styled HTML template
- Applies the image to the Windows desktop automatically
- Keeps the app running in the system tray and hides instead of closing
- Supports quick `Ctrl+S` save + apply workflow

## How it works

1. Edit your goals, tasks, or notes in the sidebar.
2. Adjust progress sliders for weekly and daily completion.
3. Click `Apply Wallpaper` or press `Ctrl+S`.
4. The frontend uses `html-to-image` and a wallpaper template to render a PNG.
5. The Rust backend saves `data.json` and applies `output.png` as the desktop wallpaper.

## Key implementation details

- Frontend: vanilla HTML, CSS, and JavaScript in `src/index.html`
- Wallpaper template: `src/template.html`
- Backend: Tauri + Rust in `src-tauri/src/main.rs`
- Native wallpaper application via the `wallpaper` Rust crate
- Data persistence using `data.json`

## Run locally

```bash
npm install
npm run tauri dev
```

## Build for distribution

```bash
npm run tauri build
```

## Windows release guide

A published Windows installer is available on GitHub Releases at:

https://github.com/Abdul-wahab113/Goals-Wallpaper-Dekstop-App/releases/tag/v1.0.0

### How non-developers can install

1. Visit the release page above.
2. Download the `.exe` or installer file from the release assets.
3. Run the downloaded file.
4. If Windows blocks execution, choose `More info` and then `Run anyway`.
5. Follow the installer prompts to complete installation.

This release is built with Tauri and does not require Node.js or Rust on the target machine.

### Using the app after install

1. Open `goal.wallpaper` from the Start menu.
2. Add or update your weekly goals, daily tasks, and notes.
3. Click `Apply Wallpaper` or press `Ctrl+S`.
4. The app saves your list and updates the desktop wallpaper automatically.

### Notes for Windows users

- Windows may warn that the app has no recognized publisher license. This is normal for early releases and safe to continue if you trust the source.
- The app stores settings in `data.json` and writes `output.png` as the generated wallpaper.
- Closing the app window hides it to the system tray so the wallpaper feature stays active.

## Notes

- The current implementation targets Windows wallpaper application.
- The app uses Tauri for a lightweight native desktop experience without Electron.
