# 运行指南 - 个人工具集

## 重要说明

**前端在浏览器中运行时，无法创建任务！**

当前 `http://localhost:5174` 只运行了前端 Vue.js 应用，但没有连接到 Rust 后端。Tauri 的 `invoke()` 函数只能在完整的 Tauri 应用环境中工作。

## 正确的运行方式

### 方式 1: 使用 Tauri 开发模式（推荐）

```bash
# 在项目根目录运行
npm run tauri:dev
```

或使用完整命令：

```bash
npm run tauri dev
```

这会同时启动：
1. Vite 前端开发服务器
2. Rust/Tauri 后端应用
3. 打开 Tauri 应用窗口

### 方式 2: 手动启动（如果方式1失败）

```bash
# 在项目根目录运行
cd src-tauri && npx tauri dev
```

这会打开 Tauri 应用窗口，在窗口中点击"新建任务"按钮才能正常工作。

## 当前问题

### 症状
点击"新建任务"按钮无反应

### 原因
- 前端运行在浏览器中（http://localhost:5174）
- 浏览器环境无法调用 Tauri 的 `invoke()` 函数
- 所有 `@tauri-apps/api` 的功能都需要在 Tauri 应用环境中

### 解决方法
**必须启动完整的 Tauri 应用**，而不仅仅是 Vite 开发服务器。

## 验证方法

启动 Tauri 应用后，应该看到：
1. ✅ Tauri 应用窗口打开（不是浏览器）
2. ✅ 点击"新建任务"按钮可以创建任务
3. ✅ 任务列表可以正常显示
4. ✅ 所有功能（搜索、筛选、数据管理等）都能正常工作

## 开发工作流

### 正常开发流程
```bash
# 1. 启动 Tauri 开发模式
npm run tauri dev

# 2. 修改前端代码
# 保存后 Vite 会自动热重载

# 3. 修改 Rust 代码
# 保存后 Cargo 会自动重新编译并重启应用
```

### 仅前端开发（不需要后端功能时）
```bash
# 使用 Mock 数据或独立 API 服务器
npm run dev
# 访问 http://localhost:5174
# 注意：无法使用任何 Tauri 功能
```

## 常见问题

### Q: 为什么浏览器中不能工作？
A: Tauri 是桌面应用框架，`invoke()` 函数通过 Tauri 的桥接机制连接前后端。浏览器没有这个桥接，所以无法调用 Rust 代码。

### Q: 如何调试前端问题？
A: 可以在 Tauri 应用窗口中打开开发者工具：
- Windows/Linux: `Ctrl + Shift + I`
- macOS: `Cmd + Option + I`

### Q: 如何调试 Rust 后端问题？
A: 在启动 Tauri 的终端查看 Rust 编译输出和错误日志。

## 总结

**重要**: 要测试或使用此应用，**必须运行完整的 Tauri 应用**（`npm run tauri dev` 或 `cd src-tauri && cargo run`），而不能只在浏览器中访问前端页面。

当前前端开发服务器（http://localhost:5174）仅用于开发调试前端样式和组件，无法测试任何实际功能。
