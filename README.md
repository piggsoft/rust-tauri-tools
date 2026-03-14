# 个人工具集

基于 Tauri + Rust + SQLx + SQLite + Vue 的个人工具集应用。

## 技术栈

### 后端
- **Tauri 2**: 跨平台桌面应用框架
- **Rust**: 系统编程语言，高性能后端
- **SQLx**: 异步 SQL 工具包，类型安全的数据库操作
- **SQLite**: 轻量级嵌入式数据库
- **Tokio**: 异步运行时

### 前端
- **Vue 3**: 渐进式 JavaScript 框架
- **TypeScript**: 类型安全的 JavaScript 超集
- **Pinia**: Vue 3 状态管理
- **Vite**: 快速的前端构建工具
- **@vueuse/core**: Vue Composition API 工具集

## 项目结构

```
personal-tools/
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs        # Tauri 入口
│   │   ├── models.rs      # 数据模型
│   │   ├── db.rs          # 数据库操作
│   │   └── handlers.rs    # Tauri command 处理
│   ├── Cargo.toml         # Rust 依赖配置
│   └── tauri.conf.json    # Tauri 配置
├── src/                   # Vue 前端
│   ├── api/               # API 封装
│   ├── components/        # Vue 组件
│   ├── views/             # 页面视图
│   ├── stores/            # Pinia 状态管理
│   ├── types/             # TypeScript 类型定义
│   ├── App.vue            # 根组件
│   └── main.ts            # 入口文件
├── package.json           # Node.js 依赖
├── vite.config.ts         # Vite 配置
├── tsconfig.json          # TypeScript 配置
└── README.md              # 项目说明
```

## 功能特性

### 已完成 ✅
- [x] 任务 CRUD（创建、读取、更新、删除）
- [x] 任务列表视图（按日期分组）
- [x] 任务筛选（状态、优先级、紧急度、标签、日期范围）
- [x] 批量操作（完成、删除、归档）
- [x] 任务表单（完整的任务属性编辑）

### 待实现 🚧
- [ ] 时间视图（日历 + 列表）
- [ ] 四象限视图（艾森豪威尔矩阵）
- [ ] 子任务功能
- [ ] 任务搜索
- [ ] 重复任务
- [ ] 任务历史记录
- [ ] 归档和恢复
- [ ] 数据导出/导入
- [ ] 数据备份/恢复

## 安装和运行

### 前置要求
- Node.js 18+
- Rust 1.70+
- Cargo

### 安装依赖

```bash
# 安装前端依赖
npm install
```

### 开发模式

```bash
# 启动 Tauri 开发服务器（推荐）
npm run tauri:dev

# 或使用完整命令
npm run tauri dev

# 或从 src-tauri 目录启动
cd src-tauri && npx tauri dev
```

**重要**：必须在 Tauri 应用窗口中测试功能，浏览器环境无法使用 Tauri 的 invoke() 函数。

### 构建生产版本

```bash
# 构建应用
npm run tauri build

# 或从 src-tauri 目录构建
cd src-tauri && npx tauri build
```

构建产物将位于 `src-tauri/target/release/bundle/` 目录。

## 数据库

### 数据库位置
- Windows: `%LOCALAPPDATA%\personal-tools\data\todo.db`
- macOS: `~/Library/Application Support/personal-tools/data/todo.db`
- Linux: `~/.local/share/personal-tools/data/todo.db`

### 数据表结构

#### tasks（任务表）
- id: 任务 ID
- title: 任务标题
- description: 任务描述
- priority: 优先级（high/medium/low）
- urgency: 紧急度（high/medium/low）
- status: 状态（pending/completed/archived）
- tags: 标签（JSON 数组）
- start_date: 开始日期
- due_date: 截止日期
- reminder_time: 提醒时间
- estimated_duration: 预估耗时（分钟）
- repeat_pattern: 重复模式
- repeat_until: 重复结束日期
- created_at: 创建时间
- updated_at: 更新时间
- completed_at: 完成时间

#### subtasks（子任务表）
- id: 子任务 ID
- task_id: 父任务 ID
- title: 子任务标题
- status: 状态（pending/completed）
- sort_order: 排序
- created_at: 创建时间

#### task_history（任务历史表）
- id: 历史记录 ID
- task_id: 任务 ID
- action: 操作类型（created/updated/completed/archived/restored）
- changes: 变更详情（JSON）
- created_at: 创建时间

#### tags（标签表）
- id: 标签 ID
- name: 标签名称
- created_at: 创建时间

## 开发指南

### 添加新的 Tauri Command

1. 在 `src-tauri/src/handlers.rs` 中添加处理函数
2. 在 `src-tauri/src/main.rs` 中注册 command
3. 在 `src/api/tasks.ts` 中添加 API 封装
4. 在 `src/stores/task.ts` 中添加状态管理方法

### 添加新的前端组件

1. 在 `src/components/` 中创建组件文件
2. 在相应的视图中引入和使用组件
3. 使用 Pinia store 管理状态

## 开发进度

- [x] 阶段一：基础框架
  - [x] 初始化 Tauri + Vue 项目
  - [x] 配置 SQLx + SQLite
  - [x] 创建数据库表结构
- [x] 阶段二：核心功能
  - [x] 任务 CRUD
  - [x] 时间视图（列表）
  - [x] 状态管理
- [ ] 阶段三：高级功能
  - [ ] 四象限视图
  - [ ] 子任务管理
  - [ ] 搜索和筛选
- [ ] 阶段四：完善功能
  - [ ] 重复任务
  - [ ] 历史记录
  - [ ] 批量操作
  - [ ] 归档和恢复
- [ ] 阶段五：数据功能
  - [ ] 数据导出/导入
  - [ ] 备份/恢复

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
