#!/bin/sh

# 把版本号写到updater.json的version字段
version=$(node -pe "require('./package.json').version")
sed -i '' "s/\"version\": \".*\"/\"version\": \"$version\"/" update/updater.json

# 把当前时间按照RFC 3339格式写到updater.json的pub_date字段
pub_date=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
sed -i '' "s/\"pub_date\": \".*\"/\"pub_date\": \"$pub_date\"/" update/updater.json

# 把sig内容写到updater.json的signature字段
sig=$(cat src-tauri/target/universal-apple-darwin/release/bundle/macos/clippy2.app.tar.gz.sig)
sed -i '' "s/\"signature\": \".*\"/\"signature\": \"$sig\"/" update/updater.json

tool=./update/ossutilmac64

$tool cp -r -u src-tauri/target/universal-apple-darwin/release/bundle/dmg/ oss://yim-sft-download/clippy2/ --config-file ~/.yim.ossutilconfig --include *.dmg
$tool cp -r -u src-tauri/target/universal-apple-darwin/release/bundle/macos/clippy2.app.tar.gz oss://yim-sft-download/clippy2/ --config-file ~/.yim.ossutilconfig
# $tool cp -r -u update/updater.json  oss://yim-sft-download/clippy2 --config-file ~/.yim.ossutilconfig

