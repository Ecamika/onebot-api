# AGENTS.md

## 项目概述
- 实现 OneBot V11 协议的 Rust **库**。不是 workspace。
- 主 crate 为 `onebot-api`（版本 1.2.3），proc-macro crate `onebot-api-macros` 位于 `macros/` 目录下。
- Rust **edition 2024** — 要求工具链版本 >= 1.85。
- `examples/communication/` 包含使用示例。
- `changelogs/` 包含每日变更记录。

## 构建 / 检查 / 测试
```bash
cargo check          # 快速编译检查
cargo build          # 完整构建（默认 features = "full"）
cargo clippy         # 代码检查
cargo fmt            # 格式化（使用 rustfmt.toml）
cargo test           # 运行所有单元测试
```

未启用任何 feature 时无法编译；`default = ["full"]` 会启用所有协议模块。
要构建/测试单组 feature：
```bash
cargo check --no-default-features --features "websocket,http"
```

## 代码格式
- `rustfmt.toml` 配置使用 **硬制表符**，`tab_spaces = 2`，`edition = "2024"`。
- 提交前运行 `cargo fmt`。CI 不强制检查格式，但项目内部约定严格遵循。

## 架构
- `Client`（位于 `src/communication/utils.rs`）是唯一入口。它使用：
  - `flume`（mpsc）作为 API 请求通道（`InternalAPISender` / `InternalAPIReceiver`）
  - `tokio::sync::oneshot` + `Arc<Mutex<BTreeMap<String, _>>>` 用于 API **响应**路由 —— 基于 echo 键的注册表，非广播
  - `tokio::broadcast`（mpmc）用于事件分发（`PublicEventSender`）
  - 通过 `CommunicationService` trait 进行依赖注入，供协议实现使用
- `src/communication/` 下的协议模块均为 **feature-gated**：
  - `websocket` —— 单个手写的 `WsTransfer` Future 状态机（`ws/ws_transfer.rs`），替代旧的 split-sink/stream 双任务模型
  - `websocket-reverse`、`http`、`http-post`、`sse`
- 可选模块同样受 feature 控制：`combiner`、`quick_operation`、`selector`
- `src/main.rs` 被 **gitignore** —— 仅用于本地开发测试，不随库发布。

## 发布
- 推送到 `master` 分支时通过 `.github/workflows/publish-release.yml` 自动触发。
- 使用 `deno task publish-release`（Deno 脚本位于 `scripts/publish-release.ts`）创建 GitHub release 并执行 `cargo publish`。
- 需要密钥：`GITHUB_TOKEN`、`CARGO_PUBLISH`。

## 关键约定
- Client 设计为包装在 `Arc<Client>` 中使用 —— 无需 `Mutex`（通道已处理同步）。
- `text!` 宏（位于 `src/message/utils.rs`）创建单文本段消息；内部委托给 `format!`。
- 错误类型：`APIRequestError` 用于 API 失败，`ServiceStartError` 用于连接设置失败。均位于 `src/error.rs`。
- `Selector`（受 `selector` feature 控制）提供可链式调用的事件过滤，每个方法均有同步和异步变体。

## Changelog rule

**无论对项目做出何种修改（`src/`、`macros/`、`examples/`、`scripts/`、`.github/`、`AGENTS.md` 等），都必须在 `changelogs/` 文件夹下记录变更。**

### 文件命名规则
- 以日期命名：`YYYY-MM-DD.md`
- 同一天的所有变更合并到同一个文件中

### 记录流程
每次变更必须按以下步骤记录：

1. **定位文件**：在 `changelogs/` 文件夹下搜索以**今天日期**命名的 markdown 文件 `YYYY-MM-DD.md`。
2. **若文件不存在**：
   - 创建文件 `changelogs/YYYY-MM-DD.md`
   - 文件顶部写入一级标题 `# YYYY-MM-DD 变更记录`
   - 按照本规范的文件结构模板，写入完整的变更条目
3. **若文件已存在**：
   - **重新读取文件**：在追加前必须重新读取该文件的完整内容，确认文件未被其他会话修改（防止历史记录被覆盖）
   - 在文件末尾（最后一个条目之后）追加新的变更条目
   - **禁止修改、删除或覆盖文件中已有的任何历史条目**
4. **条目格式**：每个条目必须严格按照本规范的二级标题、三级标题、列表格式书写。

### 文件结构模板
```markdown
# YYYY-MM-DD 变更记录

## [变更类型] 变更简要说明（一句话概括）

### 变更内容
- 具体做了什么
- 关键代码/逻辑改动点

### 涉及文件
- `文件路径` [文件修改类型] 对该文件内修改的说明

### 影响
- 对调用方、行为或性能的影响

### 原因
- 变更的动机和背景
```

### 一级标题
统一格式为 `# YYYY-MM-DD 变更记录`，例如：
```markdown
# 2026-05-31 变更记录
```

### 二级标题
统一格式为 `## [变更类型] 变更简要说明`。

变更类型使用 **git commit message type**，允许多类型复用，破坏性变更追加 `[BREAKING]`：

| 类型 | 说明 |
|------|------|
| `[feat]` | 新增功能、模块、API、trait |
| `[fix]` | 修复 Bug、逻辑错误 |
| `[docs]` | 文档、注释、README、changelog 本身的修改 |
| `[style]` | 代码格式、缩进、分号、空行等不影响逻辑的修改 |
| `[refactor]` | 重构（行为不变，内部结构优化） |
| `[perf]` | 性能优化 |
| `[test]` | 新增或修改测试代码 |
| `[chore]` | 构建脚本、工具配置、依赖版本升级等杂项 |
| `[build]` | 构建系统或外部依赖的修改（如 Cargo.toml、Makefile） |
| `[ci]` | CI/CD 配置修改（如 `.github/workflows/`） |
| `[revert]` | 回滚之前的提交 |

**多类型复用示例：**
```markdown
## [perf][fix][BREAKING] 重构事件分发逻辑并修复内存泄漏
```

> `[BREAKING]` 标记必须放在所有类型之后。

### 三级标题（强制包含）
每个变更条目必须包含以下四个三级标题：

#### `### 变更内容`
- 用无序列表描述具体改动
- 关键代码逻辑、API 签名变更、数据结构变动等

#### `### 涉及文件`
- 文件路径统一以**项目根目录**开始，不使用 `./` 前缀
- 每个文件必须标注**文件修改类型**
- 必须附带对该文件内具体修改的说明

**文件修改类型：**

| 类型 | 说明 |
|------|------|
| `[Added]` | 新增文件 |
| `[Changed]` | 修改文件内容 |
| `[Removed]` | 删除文件 |
| `[Moved]` | 重命名或移动文件（建议注明来源和去向）|

**格式示例：**
```markdown
### 涉及文件
- `src/error.rs` [Changed] 扩展 ServiceRuntimeError 枚举，新增 9 个错误变体
- `src/communication/http.rs` [Changed] 将 anyhow::Result 替换为 ServiceRuntimeResult，移除 anyhow 导入
- `Cargo.toml` [Changed] 从 dependencies 中移除 anyhow
- `src/quick_operation.rs` [Added] 新增 6 个 quick operation trait 定义
- `src/event/old_handler.rs` [Removed] 删除已废弃的事件处理模块
- `src/communication/ws_transfer.rs` [Moved] 从 `src/communication/ws/ws_transfer.rs` 移动至此处，简化目录结构
```

#### `### 影响`
- 对现有功能、调用方或行为的影响
- 若有 Breaking change，必须明确说明迁移方式

#### `### 原因`
- 变更的动机和背景
- 解决了什么问题或满足了什么需求

### 会话隔离规则
**禁止修改已存在的历史 changelog 条目。当前会话只能向当天的 changelog 文件追加新条目，不得修改、删除或覆盖已有条目（无论日期）。若发现历史 changelog 有遗漏或错误，应在当天的新条目中说明纠正，而非直接修改已有文件。**

### 规范要点
1. **同一天多次变更**：在同个 `YYYY-MM-DD.md` 中按时间顺序追加 `## [类型] 标题` 条目，不要拆成多个文件
2. **文件路径**：所有路径必须用反引号 `` ` `` 包裹，以项目根目录为起点（如 `src/lib.rs`、`README.md`、`rustfmt.toml`）
3. **标题层级**：严格使用 `#` → `##` → `###`，禁止跨级或多余层级
4. **语言**：标题和正文统一使用中文，技术术语（如 API、trait、crate、git commit type）可保留英文
5. **粒度**：每个 `##` 条目对应一个独立的变更主题，避免一个条目涵盖多个无关修改

## 宏文档规则
**对 `macros/` 目录进行任何修改（新增、删除或更改过程宏及其行为）时，必须同步更新 `macros/docs/` 中的文档。**

- 新增过程宏：在 `macros/docs/` 下创建与宏同名的 markdown 文件（例如 `#[my_macro]` 对应 `my_macro.md`），记录其功能、用法和属性。
- 删除过程宏：从 `macros/docs/` 中删除对应的 markdown 文件。
- 修改宏行为或属性：更新已有 markdown 文件以反映新行为。
