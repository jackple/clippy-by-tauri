![应用截图](https://github.com/jackple/clippy-by-tauri/blob/main/screenshot.jpg?raw=true)

# 本地配置

更新配置: `src-tauri/tauri.conf.json`

```sh
"pubkey": "xxx",
"endpoints": ["https://xx.com/clippy2/updater.json"]
```

# 本地启用

```sh
yarn tauri dev
```

<!--
# 更新版本号

```sh
./update/version.sh 0.1.11
``` -->

# 构建 mac universal 包

```sh
yarn tauri build --target universal-apple-darwin
```

<!--
# 发布包

```sh
./update/update.sh
``` -->

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
