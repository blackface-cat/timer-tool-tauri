# macOS 构建指南

## 概述

本项目使用 GitHub Actions 在真实的 macOS 环境中交叉编译，生成两个 dmg 安装包：
- **Intel 版本** (`计时器_Intel_v1.2.8.dmg`) — macOS 13 runner
- **Apple Silicon 版本** (`计时器_M_v1.2.8.dmg`) — macOS 14 runner（原生 M 芯片构建）

## 前提条件

需要一个 **GitHub 账号**，仓库 public 或 private 都可以（Actions 对两种都免费）。

## 步骤

### 1. 创建 GitHub 仓库

1. 打开 https://github.com/new
2. Repository name 填 `timer-tool-tauri`（或任意名称）
3. Private / Public 随意
4. **不要**勾选 "Add a README"、"gitignore" 等初始化选项（代码会单独推送）
5. 点击 "Create repository"

### 2. 初始化本地 Git 并推送

在 Windows 上打开 PowerShell，执行以下命令（把 `YOUR_USERNAME` 和 `REPO_NAME` 替换成你的实际值）：

```powershell
cd C:\Users\Administrator\timer-tool-tauri
git init
git add .
git commit -m "Initial commit"
git branch -M main
git remote add origin https://github.com/YOUR_USERNAME/REPO_NAME.git
git push -u origin main
```

### 3. 触发构建

1. 打开 https://github.com/YOUR_USERNAME/REPO_NAME/actions
2. 点击左侧 "Build macOS (Intel + Apple Silicon)"
3. 点击右上角 "Run workflow" → "Run workflow"
4. 等待两条 job 完成（约 10-15 分钟）

### 4. 下载 dmg 文件

构建完成后：

- 在 "build-intel" job 页面下载 `timer-intel/*.dmg` → **计时器_Intel_v1.2.8.dmg**
- 在 "build-arm" job 页面下载 `timer-arm/*.dmg` → **计时器_M_v1.2.8.dmg**

如果需要单个通用安装包（同时支持 Intel 和 M 芯片），可以从 "package-universal" job 下载 `计时器_通用版_v1.2.8.dmg`（如果 workflow 那步成功的话）。

## 后续更新

修改代码后，每次 `git push`，Actions 会自动重新构建。

## 注意事项

- GitHub Actions macOS runner **免费额度**：Linux 2000 min/月，macOS 180 min/月，足够用
- 首次构建会慢一些（需要编译 Rust 依赖），后续有缓存会快很多
- dmg 文件在 Actions 界面保留 90 天，建议及时下载备份
