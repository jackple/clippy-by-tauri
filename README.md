# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# 构建 icon

前提是当前目录下有`app-icon.png`文件

```sh
yarn tauri icon
```

# 构建 mac universal 包

```sh
yarn tauri build --target universal-apple-darwin
```

# 转换 tray icon

前提是安装了 imagemagick

```sh
magick src-tauri/icons/128x128.png -alpha copy -channel RGB -fill black -colorize 100% src-tauri/assets/tray.png
```
