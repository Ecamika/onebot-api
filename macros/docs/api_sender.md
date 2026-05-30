# `#[api_sender]`

应用于 `impl` 块的属性宏，自动为标记了 `#[api(...)]` 的方法生成 OneBot API 调用体。

---

## 功能概述

在 OneBot 协议实现中，API 方法通常有固定的调用模式：构造 JSON 参数 → 发送请求 → 解析响应。`#[api_sender]` 宏通过自动代码生成消除了这些样板代码。

配合方法上的 `#[api(...)]` 属性，可以：

- 自动将方法参数转换为 JSON 请求体
- 自动调用 `send_and_parse()` 发送请求并解析响应
- 支持从响应中提取特定字段
- 支持参数名到 JSON 键名的映射

---

## 基本用法

### 1. 标记 `impl` 块

将 `#[api_sender]` 放在 `impl` 块上：

```rust
use onebot_api_macros::api_sender;

#[api_sender]
impl MyAPIClient {
    // 方法会自动生成 API 调用体
}
```

### 2. 在方法上使用 `#[api(...)]`

需要自动生成的方法上添加 `#[api(...)]` 属性：

```rust
#[api_sender]
impl MyAPIClient {
    #[api]
    pub async fn send_private_msg(&self, user_id: i64, message: String) -> Result<serde_json::Value, Error> {
        // 方法体会被自动生成，这里的代码会被替换
    }
}
```

实际生成的代码等价于：

```rust
impl MyAPIClient {
    pub async fn send_private_msg(&self, user_id: i64, message: String) -> Result<serde_json::Value, Error> {
        let params = serde_json::json!({
            "user_id": user_id,
            "message": message,
        });
        self.send_and_parse("send_private_msg", params).await
    }
}
```

---

## `#[api(...)]` 属性详解

### 基本属性（无参数）

```rust
#[api]
pub async fn method_name(&self, param1: Type1, param2: Type2) -> Result<T, E>;
```

- 方法名自动作为 API action 名称
- 所有参数自动序列化为 JSON 对象中的字段
- 键名与参数名相同

### `response = Type` — 指定响应类型

```rust
#[api(response = SendMsgResponse)]
pub async fn send_group_msg(&self, group_id: i64, message: String) -> Result<SendMsgResponse, Error>;
```

生成的代码：

```rust
let params = serde_json::json!({
    "group_id": group_id,
    "message": message,
});
self.send_and_parse::<SendMsgResponse>("send_group_msg", params).await
```

### `extract = "field"` — 提取响应字段

如果 API 返回的响应是一个对象，而你只需要其中某个字段，可以使用 `extract`：

```rust
#[api(response = SendMsgResponse, extract = "message_id")]
pub async fn send_group_msg(&self, group_id: i64, message: String) -> Result<i64, Error>;
```

生成的代码：

```rust
let params = serde_json::json!({
    "group_id": group_id,
    "message": message,
});
let response: SendMsgResponse = self.send_and_parse("send_group_msg", params).await?;
Ok(response.message_id)
```

> **注意**：使用 `extract` 时必须同时指定 `response`，因为需要先反序列化为中间类型才能提取字段。

### `map(...)` — 参数名映射

OneBot API 的 JSON 键名可能与 Rust 参数命名风格不同（如蛇形命名 vs API 命名）。使用 `map` 可以将参数映射到不同的 JSON 键名：

```rust
#[api(map(group_id = "groupId", message = "msg"))]
pub async fn send_group_msg(&self, group_id: i64, message: String) -> Result<serde_json::Value, Error>;
```

生成的 JSON：

```json
{
    "groupId": group_id,
    "msg": message
}
```

`map` 可以与其他属性组合使用：

```rust
#[api(
    response = SendMsgResponse,
    extract = "message_id",
    map(user_id = "userId", message = "msg_content")
)]
pub async fn send_private_msg(&self, user_id: i64, message: String) -> Result<i64, Error>;
```

---

## 完整示例

```rust
use onebot_api_macros::api_sender;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct SendMsgResponse {
    message_id: i64,
}

struct APIClient;

impl APIClient {
    async fn send_and_parse<T: serde::de::DeserializeOwned>(
        &self,
        action: &str,
        params: serde_json::Value,
    ) -> Result<T, APIError> {
        // 实际的网络请求逻辑
        todo!()
    }
}

#[api_sender]
impl APIClient {
    // 简单调用
    #[api]
    pub async fn get_login_info(&self) -> Result<serde_json::Value, APIError>;

    // 带参数映射
    #[api(map(user_id = "user_id"))]
    pub async fn get_stranger_info(&self, user_id: i64, no_cache: bool) -> Result<serde_json::Value, APIError>;

    // 带响应类型和字段提取
    #[api(response = SendMsgResponse, extract = "message_id")]
    pub async fn send_private_msg(&self, user_id: i64, message: String) -> Result<i64, APIError>;

    // 组合使用
    #[api(
        response = SendMsgResponse,
        extract = "message_id",
        map(group_id = "group_id", message = "message")
    )]
    pub async fn send_group_msg(&self, group_id: i64, message: String) -> Result<i64, APIError>;

    // 没有 #[api] 属性的方法保持不变
    pub async fn custom_method(&self) {
        // 手动实现
    }
}
```

---

## 工作原理

1. `#[api_sender]` 解析 `impl` 块中的所有方法
2. 检查每个方法是否有 `#[api(...)]` 属性
3. 对于有 `#[api]` 的方法：
   - 使用方法名作为 API action 名称
   - 扫描方法参数，生成 `serde_json::json!` 构造
   - 应用 `map` 重命名规则
   - 如果有 `extract`，先反序列化为 `response` 类型，再提取字段
   - 替换原方法体为生成的代码
4. 没有 `#[api]` 的方法原样保留

---

## 注意事项

- 目标类型必须有 `send_and_parse` 方法，签名大致为：
  ```rust
  async fn send_and_parse<T>(&self, action: &str, params: serde_json::Value) -> Result<T, E>
  ```
- `#[api]` 属性只能用于异步方法（`async fn`）
- 第一个参数必须是 `&self`（目前不支持 `&mut self`）
- `extract` 和 `response` 必须成对使用
- `map` 中的条目用逗号分隔，格式为 `param_name = "json_key"`
