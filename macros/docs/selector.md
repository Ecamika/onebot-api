# `#[derive(Selector)]`

为结构体或枚举生成链式事件过滤器（`Selector`）相关方法。

> 此宏生成的所有代码都带有 `#[cfg(feature = "selector")]`，因此需要在 `Cargo.toml` 中启用 `selector` feature 才能使用。

---

## 功能概述

`Selector` 是一种包裹器模式，允许对事件/数据进行链式过滤和类型匹配。使用此派生宏，你可以：

- **对结构体字段进行链式过滤**（按字段值筛选）
- **对枚举变体进行模式匹配**（提取特定变体的内部数据）
- **通过属性扩展**快捷判定和子选择器功能

---

## 基本用法

### 结构体

```rust
use onebot_api_macros::Selector;

#[derive(Selector)]
struct Message {
    message_type: String,
    user_id: i64,
    content: String,
}
```

这会自动生成：

```rust
impl Message {
    pub fn selector(&'_ self) -> Selector<'_, Self>;
}

impl<'a> Selector<'a, Message> {
    // 基础过滤方法
    pub fn filter(&mut self, f: impl FnOnce(&Message) -> bool);
    pub fn and_filter(mut self, f: impl FnOnce(&Message) -> bool) -> Self;
    pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&Message) -> bool);
    pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&Message) -> bool) -> Self;

    // 按字段过滤的方法（每个字段都会生成一组）
    pub fn filter_message_type(&mut self, f: impl FnOnce(&str) -> bool);
    pub fn and_filter_message_type(mut self, f: impl FnOnce(&str) -> bool) -> Self;
    pub async fn filter_message_type_async(&mut self, f: impl AsyncFnOnce(&str) -> bool);
    pub async fn and_filter_message_type_async(mut self, f: impl AsyncFnOnce(&str) -> bool) -> Self;

    // 其他字段类似...
}
```

### 使用示例

```rust
let msg = Message {
    message_type: "private".to_string(),
    user_id: 123456,
    content: "hello".to_string(),
};

// 链式过滤
let result = msg
    .selector()
    .and_filter_message_type(|t| t == "private")
    .and_filter_user_id(|id| *id == 123456);

// result.data 为 Some(&msg) 表示通过过滤
// result.data 为 None 表示被过滤掉
```

---

## 字段属性

### `#[selector(variants(...))]`

为字段生成快捷的变体判定方法。适用于字段值具有明确变体的情况（如枚举字段）。

```rust
#[derive(Selector)]
struct Event {
    #[selector(variants(Group, Private))]
    post_type: String,
    user_id: i64,
}
```

这会为 `post_type` 字段额外生成：

```rust
impl<'a> Selector<'a, Event> {
    // 判定是否为 "Group" 变体
    pub fn group(&mut self);
    pub fn and_group(mut self) -> Self;
    pub fn not_group(&mut self);
    pub fn and_not_group(mut self) -> Self;

    // 判定是否为 "Private" 变体
    pub fn private(&mut self);
    pub fn and_private(mut self) -> Self;
    pub fn not_private(&mut self);
    pub fn and_not_private(mut self) -> Self;
}
```

> 注意：变体名称使用 `snake_case` 转换。宏假设字段类型有一个 `is_xxx()` 方法（如 `String` 没有，但自定义枚举会有）。

### `#[selector(through = "method_name")]`

生成一个返回子选择器的方法，用于链式深入访问嵌套结构。

```rust
#[derive(Selector)]
struct Event {
    #[selector(through = "message")]
    message: Message,
}

#[derive(Selector)]
struct Message {
    content: String,
}
```

使用方式：

```rust
event
    .selector()
    .and_message()              // 进入 message 子选择器
    .and_filter_content(|c| c == "hello");
```

生成的子选择器方法签名：

```rust
impl<'a> Selector<'a, Event> {
    pub fn message(&self) -> Selector<'a, Message>;
}
```

---

## 枚举支持

`#[derive(Selector)]` 也支持枚举，但仅对**单字段无命名字段变体**（`Variant(Type)`）生成方法。

```rust
#[derive(Selector)]
enum Event {
    Message(MessageData),
    Notice(NoticeData),
    Request(RequestData),
}
```

这会自动生成：

```rust
impl Event {
    pub fn selector(&'_ self) -> Selector<'_, Self>;

    // 模式匹配方法
    pub fn match_message(&self) -> Option<&MessageData>;
    pub fn match_notice(&self) -> Option<&NoticeData>;
    pub fn match_request(&self) -> Option<&RequestData>;

    // 处理函数方法
    pub fn on_message<T>(&self, handler: impl FnOnce(&MessageData) -> T) -> Option<T>;
    pub fn on_notice<T>(&self, handler: impl FnOnce(&NoticeData) -> T) -> Option<T>;
    pub fn on_request<T>(&self, handler: impl FnOnce(&RequestData) -> T) -> Option<T>;

    // async 版本
    pub async fn on_message_async<T>(&self, handler: impl AsyncFnOnce(&MessageData) -> T) -> Option<T>;
    // ...
}

impl<'a> Selector<'a, Event> {
    pub fn filter(&mut self, f: impl FnOnce(&Event) -> bool);
    pub fn and_filter(mut self, f: impl FnOnce(&Event) -> bool) -> Self;
    pub async fn filter_async(&mut self, f: impl AsyncFnOnce(&Event) -> bool);
    pub async fn and_filter_async(mut self, f: impl AsyncFnOnce(&Event) -> bool) -> Self;

    // 变体子选择器
    pub fn message(&self) -> Selector<'a, MessageData>;
    pub fn notice(&self) -> Selector<'a, NoticeData>;
    pub fn request(&self) -> Selector<'a, RequestData>;
}
```

### 使用示例

```rust
let event = Event::Message(msg_data);

// 链式匹配特定变体并过滤内部数据
let result = event
    .selector()
    .message()
    .and_filter(|m| m.user_id == 123456);
```

---

## 字段类型处理

宏会自动根据字段类型决定过滤参数类型和字段访问方式：

| 字段类型 | 过滤参数类型 | 字段访问方式 |
|---------|------------|------------|
| `String` | `&str` | `&data.field` |
| 原始类型 (`i32`, `bool` 等) | 值本身 | `data.field`（复制） |
| `Box<T>` | `&T` | `&*data.field`（解引用） |
| 其他类型 | `&T` | `&data.field` |

---

## 限制

- 不支持 `union` 类型
- 枚举只支持单字段的无名变体（`Variant(Type)`）
- `variants` 属性假设字段类型有对应的 `is_xxx()` 方法
- 所有生成代码都受 `#[cfg(feature = "selector")]` 条件编译控制
