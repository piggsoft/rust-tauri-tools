# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changes
- 准备首次发布

## [0.1.0] - 2026-03-15

### Added
- 待办事项管理模块
  - 任务创建、编辑、删除功能
  - 任务优先级和紧急度设置
  - 任务状态管理（待办、完成、归档）
  - 子任务支持和嵌套
  - 任务历史记录
  - 任务批量操作（完成、删除、归档）
- 多视图展示
  - 列表视图（按日期分组）
  - 日历视图（可视化日期任务）
  - 四象限视图（艾森豪威尔矩阵）
  - 搜索筛选面板
- 密码本管理模块
  - 密码创建、编辑、删除功能
  - 分类和子分类管理
  - 密码复制到剪贴板
  - 密码显示/隐藏切换
  - 网站链接跳转
  - 密码搜索功能
- 统一导航侧边栏
  - 可折叠侧边栏（hover 展开）
  - 首页、Todo、密码本三个导航项
  - 当前页面高亮显示
  - 响应式设计支持
- 首页功能选择界面
  - 功能卡片展示
  - 快速导航到各模块
  - 美观的渐变背景设计

### Improved
- 侧边栏展开/收起布局同步
- 首页导航白屏问题修复
- SQL 注入防护（参数化查询）
- 代码质量和可维护性优化
  - Rust 文档注释完善
  - TypeScript 类型安全增强
  - Vue 组件文档化
  - 错误处理和重试机制

### Technical
- Tauri 2 框架集成
- Rust + SQLx + SQLite 后端
- Vue 3 + TypeScript + Pinia 前端
- GitHub Actions 自动构建和发布
- 跨平台支持（Windows、macOS、Linux）

[Unreleased]: https://github.com/your-username/tools/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/your-username/tools/releases/tag/v1.0.0
