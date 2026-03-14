# 快速启动指南

## 🚀 一键启动

在项目根目录运行：

```bash
npm run tauri:dev
```

这会自动：
1. 启动前端 Vite 开发服务器
2. 编译并启动 Rust 后端
3. 打开 Tauri 应用窗口

## ⚠️ 重要提示

**必须在 Tauri 应用窗口中测试功能**，不能在浏览器中使用。

如果只在浏览器中打开 `http://localhost:5175`，点击"新建任务"等按钮不会有任何反应，因为 Tauri 的 `invoke()` 函数只能在 Tauri 应用环境中工作。

## 📱 如何使用

1. **启动应用**：运行 `npm run tauri:dev`
2. **等待窗口打开**：大约需要 10 秒编译时间
3. **测试功能**：在 Tauri 窗口中点击"新建任务"按钮
4. **查看日志**：在 PowerShell 终端中查看编译和运行日志

## 🔧 其他命令

```bash
# 构建生产版本
npm run tauri build

# 仅运行前端（无法测试 Tauri 功能）
npm run dev

# 构建前端
npm run build
```

## 🐛 调试

- **前端调试**：在 Tauri 窗口中按 `Ctrl + Shift + I` 打开开发者工具
- **后端调试**：查看 PowerShell 终端的 Rust 编译输出和错误日志

## ❓ 常见问题

### Q: 为什么在浏览器中不能工作？
A: Tauri 是桌面应用框架，需要完整的 Tauri 环境才能运行 Rust 代码。

### Q: 如何查看错误信息？
A: 在启动应用的 PowerShell 终端中查看详细日志。

### Q: 如何重启应用？
A: 修改代码后，应用会自动重新编译并重启。

---

详细文档请查看：
- [运行指南](RUN_GUIDE.md) - 完整的运行说明
- [最终验证](FINAL_VERIFICATION.md) - 应用启动成功的验证报告
- [项目说明](README.md) - 项目介绍和功能列表
