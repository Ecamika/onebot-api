# onebot-api TODO List

> 基于当前仓库实现、文档状态和测试结果整理。优先级越高，越建议越早处理。

## P0 立即处理

- [✅] 修复 `ws_reverse` 的连接状态回收问题
  目前连接建立后会将 `connected` 置为 `true`，但连接结束后没有稳定地回收为 `false`，可能导致首个连接断开后后续重连一直被拒绝。

- [ ] 统一各 `CommunicationService` 的运行状态与退出语义
  `http`、`sse`、`ws_reverse` 当前在任务异常退出后可能仍保持 `is_running = true`，并且后台任务错误会被静默吞掉，影响重启、排障和调用方判断。

- [ ] 清理协议服务中的 panic 路径
  对外暴露的 HTTP Post / Reverse WebSocket 入口里存在 `unwrap()`，面对非法 Header、非法 JSON、异常连接时不应 panic，而应返回明确的状态码或结构化错误。

- [ ] 修复已经失效的 doctest 与内联示例
  当前 `cargo test` 下 doctest 已失败，说明示例代码和真实 API 已经漂移。需要同步修正 `src/communication/utils.rs`、`src/communication/combiner.rs` 以及 README 中的示例。

## P1 短期内完成

- [ ] 补齐主 crate 的测试体系
  目前主 crate 几乎没有覆盖核心通信层的测试，建议优先补：
  - `Client` 的 echo 路由与超时行为
  - `raw_event_processor` 的事件/响应分流逻辑
  - `ws` / `ws_reverse` / `http` / `sse` 的启动、停止、重启行为
  - 断线重连与通道关闭后的边界场景

- [ ] 统一公共 API 中的 ID 类型
  当前 `user_id` / `group_id` / 部分响应模型在 `i32` 与 `i64` 间混用，建议统一策略，尽量收敛到与协议数据更一致的类型，避免调用方频繁转换。

- [ ] 统一文档与实现对事件分发模型的描述
  README 目前将 `Client` 描述为直接使用 `tokio::broadcast` 分发事件，但实现中公开事件通道实际仍是 `flume`，`broadcast` 由装饰器提供，需要校正文档表述。

- [ ] 为服务错误补充更可诊断的错误信息
  当前不少地方使用 `Unknown` 或直接忽略错误，建议将监听失败、握手失败、反序列化失败、连接关闭原因等细分并透出，方便库使用者定位问题。

## P2 中期优化

- [ ] 提升 `Selector` 与宏系统的测试覆盖和稳定性
  `selector` 是偏易用性功能，但目前更像“能用的生成器”，建议补充：
  - 结构体/枚举多层 selector 的快照测试
  - 错误输入下的宏编译失败测试
  - 文档中对 `variants` / `through` 的边界行为说明

- [ ] 梳理 `ClientBuilder` / `Client` 构造 API，减少重载歧义
  当前 `new` / `new_with_timeout` / `new_with_union_channel_cap` / `new_with_options` 并存，示例也因此容易过期。可以考虑进一步收敛到 builder 优先的风格。

- [ ] 改善通道和任务的可观测性
  建议逐步加入日志埋点或可选 tracing 支持，至少覆盖：
  - 服务启动/停止/重启
  - 连接建立/断开/重连
  - API 请求发送与响应超时
  - 事件丢弃或反序列化失败

- [ ] 评估 `std::sync::Mutex` 在异步路径中的使用范围
  当前 echo 注册表等位置仍依赖同步锁，虽然临界区较短，但后续可评估是否需要进一步封装、降低锁竞争，或者明确并发语义。

## P3 后续发展方向

- [ ] 建立更完整的 feature 组合验证矩阵
  这个库强依赖 feature-gated 模块，后续应保证默认特性、最小特性组合、协议单独启用、TLS 后端切换都能持续通过检查与测试。

- [ ] 设计并补齐集成测试基建
  长远看建议引入本地 mock OneBot 服务端，针对 WebSocket / HTTP / SSE / Reverse WS 做端到端测试，避免只靠单元测试验证协议行为。

- [ ] 提升发布质量门槛
  发布前建议至少自动执行：
  - `cargo fmt`
  - `cargo clippy`
  - `cargo test`
  - `cargo test --doc`
  - 关键 feature 组合的 `cargo check --no-default-features --features "..."`

- [ ] 为外部扩展实现预留更清晰的能力边界
  `CommunicationService` 是很好的抽象入口，后续可以继续明确哪些行为由协议层负责、哪些行为由 `Client` 负责，并为第三方自定义 service 提供更完整的实现指南。

- [ ] 补充更贴近真实场景的示例工程
  除了当前通信示例，后续可以增加：
  - 基础 bot 收发消息示例
  - HTTP + SSE 组合器示例
  - selector 事件过滤示例
  - quick operation 示例
  - 自定义 `CommunicationService` 示例

## 推荐执行顺序

1. 先修复 P0 的生命周期、panic、doctest 问题，保证项目“能稳定跑、文档能信”。
2. 然后完成 P1 的测试补强和公共 API 一致性整理，降低后续改动成本。
3. 最后推进 P2/P3 的可维护性、生态扩展和发布质量建设。
