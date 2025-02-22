#!/bin/bash

# 检查参数
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <new_version>"
    echo "Example: $0 0.1.9"
    exit 1
fi

NEW_VERSION=$1

# 验证版本号格式是否符合语义化版本规范
if ! echo "$NEW_VERSION" | grep -qE '^(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$'; then
    echo "Error: Version number must follow semantic versioning format (MAJOR.MINOR.PATCH)"
    echo "Examples: 0.1.0, 1.0.0, 2.10.3"
    exit 1
fi

# 获取脚本所在目录的上级目录的绝对路径（项目根目录）
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# 检查是否是 git 仓库
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    echo "Error: $PROJECT_ROOT is not a git repository"
    exit 1
fi

# 定义用于处理版本号的正则表达式匹配模式
VERSION_PATTERN='[0-9][0-9]*\.[0-9][0-9]*\.[0-9][0-9]*'

# 更新 package.json 的版本号
sed -i '' "s/\"version\": \"${VERSION_PATTERN}\"/\"version\": \"$NEW_VERSION\"/" "$PROJECT_ROOT/package.json"

# 更新 Cargo.toml 的版本号
sed -i '' "s/^version = \"${VERSION_PATTERN}\"/version = \"$NEW_VERSION\"/" "$PROJECT_ROOT/src-tauri/Cargo.toml"

# 更新 tauri.conf.json 的版本号
sed -i '' "s/\"version\": \"${VERSION_PATTERN}\"/\"version\": \"$NEW_VERSION\"/" "$PROJECT_ROOT/src-tauri/tauri.conf.json"

echo "Version updated to $NEW_VERSION in all files successfully!"

# 等待 1 秒，确保 Cargo.lock 更新完成
sleep 1

# 切换到项目根目录并执行 git 命令
cd "$PROJECT_ROOT"
git add package.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json
git commit -m "chore: bump version to $NEW_VERSION"

echo "Changes committed to git successfully!"