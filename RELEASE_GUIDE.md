# Release 页面配置完成

## ✅ 已配置功能

### 1. 自动生成 Release
当推送标签（如 `v1.0.0`）到 GitHub 时，GitHub Actions 会自动：
- 构建所有平台的安装包
- 创建 Draft Release
- 包含自动提取的 Changelog
- 上传所有安装包到 Release

### 2. Release 页面内容
每个 Release 包含：
- **版本信息**：版本号和标题
- **Changelog**：从 `CHANGELOG.md` 自动提取的变更内容
- **下载说明**：各平台的下载包说明
- **安装说明**：详细的安装步骤
- **完整链接**：指向 CHANGELOG.md 的链接

### 3. 支持的平台
- **Windows**: NSIS 安装程序 (.exe)
- **macOS Intel**: DMG 磁盘镜像
- **macOS Apple Silicon**: DMG 磁盘镜像
- **Linux**: AppImage 便携应用

## 📋 Release 页面结构示例

```
Release v1.0.0

[自动提取的 CHANGELOG 内容]

## 下载说明

选择适合你操作系统的安装包：

- **Windows**: 下载 `.exe` 安装程序
- **macOS Intel**: 下载 `.dmg` 文件
- **macOS Apple Silicon**: 下载 `.dmg` 文件
- **Linux**: 下载 `.AppImage` 文件

## 安装说明

### Windows
1. 下载 `.exe` 安装程序
2. 运行安装程序
3. 按照提示完成安装

### macOS
1. 下载 `.dmg` 文件
2. 打开文件
3. 将应用拖拽到 Applications 文件夹

### Linux
1. 下载 `.AppImage` 文件
2. 添加执行权限：`chmod +x 个人工具集-x86_64.AppImage`
3. 运行：`./个人工具集-x86_64.AppImage`

完整变更日志：[CHANGELOG.md](https://github.com/piggsoft/rust-tauri-tools/blob/main/CHANGELOG.md)

---

Assets (4)
📦 personal-tools_1.0.0_amd64.AppImage (Linux)
📦 个人工具集_1.0.0_x64.dmg (macOS Intel)
📦 个人工具集_1.0.0_aarch64.dmg (macOS Apple Silicon)
📦 个人工具集_1.0.0_x64-setup.exe (Windows)
```

## 🚀 创建新版本

### 方法一：使用 Git 标签（推荐）
```bash
# 1. 更新 CHANGELOG.md
# 添加新版本章节，记录变更

# 2. 提交更改
git add .
git commit -m "chore: prepare for release v1.0.0"

# 3. 推送到 GitHub
git push origin main

# 4. 创建并推送标签（触发自动构建）
git tag v1.0.0
git push origin v1.0.0
```

### 方法二：在 GitHub 手动创建
1. 访问仓库页面
2. 点击 "Releases" > "Create a new release"
3. 输入标签版本（如 `v1.0.0`）
4. GitHub Actions 会自动构建并上传文件
5. 完成后编辑 Release，补充说明

## 📝 更新 CHANGELOG

每次发布前更新 `CHANGELOG.md`：

```markdown
## [1.1.0] - 2026-03-20

### Added
- 新功能描述
- 另一个新功能

### Changed
- 改进的功能描述

### Fixed
- 修复的 Bug 描述
```

## 🎨 自定义 Release 说明

### 修改下载和安装说明
编辑 `.github/workflows/tauri-build.yml` 中的 `releaseBody` 部分：

```yaml
releaseBody: |
  ${{ steps.changelog.outputs.notes }}

  ## 自定义说明
  这里可以添加任何你想要的说明
```

### 修改 Changelog 提取
编辑 `scripts/extract-changelog.js` 脚本来自定义提取逻辑。

## 🔧 故障排查

### Release 没有自动创建
1. 检查标签格式是否正确（必须以 `v` 开头，如 `v1.0.0`）
2. 查看 GitHub Actions 日志，确认构建成功
3. 确认 workflow 中 `releaseDraft: true` 设置

### Changelog 没有显示
1. 检查 `CHANGELOG.md` 中是否有对应版本的章节
2. 确保版本号格式正确（如 `## [1.0.0]`）
3. 查看 Actions 日志中的 "Extract changelog" 步骤

### 安装包没有上传
1. 检查 `src-tauri/tauri.conf.json` 中的 `bundle.active` 是否为 `true`
2. 确认 `targets` 配置正确
3. 查看构建日志中是否有打包错误

## 📊 查看构建状态

### 访问 Actions 页面
```
https://github.com/piggsoft/rust-tauri-tools/actions
```

### 查看特定标签的构建
```
https://github.com/piggsoft/rust-tauri-tools/actions/runs/<run-id>
```

## 🎉 发布流程总结

1. ✅ 更新 `CHANGELOG.md`
2. ✅ 提交代码并推送到 main 分支
3. ✅ 创建并推送标签 `v1.0.0`
4. ✅ 等待 GitHub Actions 完成构建（约 15-30 分钟）
5. ✅ 访问 Releases 页面查看 Draft Release
6. ✅ 编辑 Release 说明（可选）
7. ✅ 点击 "Publish release" 发布

## 🔗 相关链接

- **Releases 页面**: https://github.com/piggsoft/rust-tauri-tools/releases
- **Actions 页面**: https://github.com/piggsoft/rust-tauri-tools/actions
- **CHANGELOG.md**: https://github.com/piggsoft/rust-tauri-tools/blob/main/CHANGELOG.md
- **发布检查清单**: [RELEASE_CHECKLIST.md](./RELEASE_CHECKLIST.md)
- **构建和发布指南**: [BUILD_AND_RELEASE.md](./BUILD_AND_RELEASE.md)

---

## 当前配置

- ✅ GitHub Actions 自动构建配置
- ✅ 自动创建 Draft Release
- ✅ 自动提取 Changelog
- ✅ 自动上传安装包
- ✅ 包含下载和安装说明
- ✅ 支持多平台打包

现在你可以直接创建标签来触发自动发布流程了！
