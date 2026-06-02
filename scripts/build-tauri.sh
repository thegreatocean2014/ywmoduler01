#!/bin/bash

echo "🔨 Building Vben Admin Tauri Application..."

# 清理旧的构建文件
echo "Cleaning previous builds..."
pnpm run clean

# 安装依赖
echo "Installing dependencies..."
pnpm install

# 构建Web前端
echo "Building web-antd frontend..."
pnpm run build:antd

# 构建Tauri应用
echo "Building Tauri application..."
pnpm tauri build

echo "✅ Build completed! Check ./src-tauri/target/release/ for binaries"


