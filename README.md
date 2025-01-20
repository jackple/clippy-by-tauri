# 构建 mac universal 包

```sh
yarn tauri build --target universal-apple-darwin
```

# 构建 icon

前提是当前目录下有`app-icon.png`文件

```sh
yarn tauri icon
```

# 转换 tray icon

前提是安装了 imagemagick

```sh
magick src-tauri/icons/128x128.png -alpha copy -channel RGB -fill black -colorize 100% src-tauri/assets/tray.png
```
