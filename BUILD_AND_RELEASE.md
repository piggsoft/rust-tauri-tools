# Tauri Build & Release Guide

## 项目发布到 GitHub

### 1. 准备工作

#### 1.1 配置应用图标
Tauri 需要以下格式的图标文件，放在 `src-tauri/icons/` 目录下：
- `32x32.png` - Linux 小图标
- `128x128.png` - Linux 图标
- `128x128@2x.png` - Linux 高分辨率图标 (256x256)
- `icon.icns` - macOS 图标
- `icon.ico` - Windows 图标（已存在）

**图标生成方法：**
- 使用 [ImageMagick](https://imagemagick.org/) 从源图标生成多尺寸图标
- 在线工具：[favicon.io](https://favicon.io/) 或 [cloudconvert](https://cloudconvert.com/)
- 推荐图标尺寸：至少 1024x1024 PNG 源图

#### 1.2 版本号管理
更新 `src-tauri/tauri.conf.json` 中的版本号：
```json
{
  "version": "1.0.0"
}
```

### 2. GitHub 设置

#### 2.1 推送代码到 GitHub
```bash
# 初始化 Git 仓库（如果还未初始化）
git init

# 添加所有文件
git add .

# 提交
git commit -m "Initial commit"

# 添加远程仓库
git remote add origin https://github.com/your-username/tools.git

# 推送到 main 分支
git push -u origin main
```

#### 2.2 创建 GitHub Personal Access Token（可选）
如果需要在 CI 中推送标签或其他操作：
1. 访问 GitHub Settings > Developer settings > Personal access tokens
2. 创建新 Token，勾选 `repo` 权限
3. 添加到 GitHub 仓库的 Secrets: `GITHUB_TOKEN`（自动提供）或自定义 token

### 3. 创建 Release

#### 方式一：通过 Git Tag 触发自动发布
```bash
# 创建并推送标签（会触发 GitHub Actions 构建并创建 Release）
git tag v1.0.0
git push origin v1.0.0
```

#### 方式二：在 GitHub 手动创建 Release
1. 访问仓库页面
2. 点击 "Releases" > "Create a new release"
3. 输入标签版本（如 `v1.0.0`）
4. 填写发布说明
5. 点击 "Publish release"

### 4. GitHub Actions 工作流程

#### 4.1 自动构建平台
配置的 workflow 会自动构建以下平台的安装包：
- **Ubuntu**: `x86_64-unknown-linux-gnu` - AppImage, DEB
- **macOS**: `x86_64-apple-darwin` - DMG, APP (Intel)
- **macOS**: `aarch64-apple-darwin` - DMG, APP (Apple Silicon)
- **Windows**: `x86_64-pc-windows-msvc` - NSIS, MSI

#### 4.2 触发条件
- **push 到 main 分支**：构建并上传 artifacts
- **push tag (v*)**：构建并创建 draft release
- **pull request 到 main**：构建验证

#### 4.3 构建产物
构建完成后，安装包会自动上传到：
1. **GitHub Artifacts**：每次 push 后可在 Actions 页面下载（保留 30 天）
2. **GitHub Release**：标签推送后自动关联到对应的 Release

### 5. 下载安装包

#### 从 Actions 页面下载
1. 访问仓库 "Actions" 标签
2. 选择最近的 workflow run
3. 在 "Artifacts" 部分下载对应平台的安装包

#### 从 Release 页面下载
1. 访问仓库 "Releases" 标签
2. 选择对应的版本
3. 下载适合你操作系统的安装包

### 6. 常见问题

#### 6.1 macOS 签名问题
如果需要代码签名，需要在 GitHub Secrets 中添加：
- `APPLE_CERTIFICATE`: Base64 编码的证书
- `APPLE_CERTIFICATE_PASSWORD`: 证书密码
- `APPLE_ID`: Apple ID
- `APPLE_PASSWORD`: App 专用密码
- `APPLE_TEAM_ID`: 开发者团队 ID

#### 6.2 Windows 签名问题
添加以下 Secrets：
- `WINDOWS_CERTIFICATE`: Base64 编码的证书文件
- `WINDOWS_CERTIFICATE_PASSWORD`: 证书密码

#### 6.3 构建失败
- 检查 `tauri.conf.json` 中的 `bundle.active` 是否为 `true`
- 确认所有图标文件存在
- 查看 Actions 日志中的具体错误信息

### 7. 优化建议

#### 7.1 减小构建体积
在 `Cargo.toml` 中启用 LTO（链接时优化）：
```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = "s"
strip = true
```

#### 7.2 更新版本号
建议遵循语义化版本规范：
- `MAJOR.MINOR.PATCH`（如 1.0.0, 1.1.0, 1.1.1）
- MAJOR：不兼容的 API 变更
- MINOR：向下兼容的功能新增
- PATCH：向下兼容的问题修复

#### 7.3 更新 CHANGELOG
每次发布时更新 `CHANGELOG.md`，记录变更内容。

### 8. 快速开始命令

```bash
# 1. 本地测试构建
npm run tauri:build

# 2. 推送代码触发构建
git add .
git commit -m "feat: ready for release v1.0.0"
git push origin main

# 3. 创建并推送标签
git tag v1.0.0
git push origin v1.0.0

# 4. 访问 GitHub 查看构建状态和下载安装包
```

## 当前状态

- ✅ GitHub Actions workflow 已配置
- ✅ Tauri bundle 已启用
- ⚠️ 需要准备完整图标文件（.png, .icns）
- ⚠️ 版本号需要在发布前更新

## 下一步操作

1. 准备应用图标文件（如果需要自定义）
2. 更新 `tauri.conf.json` 中的版本号和描述
3. 推送代码到 GitHub
4. 创建标签触发首次构建
