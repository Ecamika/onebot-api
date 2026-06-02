# OneBot V11 Mock Server 使用文档

`mock/server` 提供一个 Deno TS 编写的 OneBot V11 mock
服务端框架。它的整体结构与主 crate 的 `Client + CommunicationService`
对应，但方向相反：

- `Server` 负责注册 API handler、发送事件、启动和停止服务。
- `CommunicationService` 负责具体通信协议。
- 内置
  `WsService`、`WsReverseService`、`HttpService`、`HttpPostService`、`SseService`
  五种服务实现。

## 快速开始

```ts
import { Server, WsService } from "../index.ts";

const server = new Server(
  new WsService({
    port: 8080,
    accessToken: "token",
  }),
);

server.onApi("send_private_msg", (params) => {
  console.log("send_private_msg params:", params);
  return {
    status: "ok",
    retcode: 0,
    data: {
      message_id: 1,
    },
  };
});

await server.startService();

await server.sendEvent({
  time: Math.floor(Date.now() / 1000),
  self_id: 10000,
  post_type: "meta_event",
  meta_event_type: "heartbeat",
  status: {},
  interval: 5000,
});
```

从项目根目录运行示例文件时，可使用：

```bash
deno run --allow-net mock/server/example.ts
```

## Server API

```ts
const server = new Server(service);
```

常用方法：

- `startService()`：启动底层通信服务。
- `stopService()`：停止底层通信服务。
- `restartService()`：重启底层通信服务。
- `changeService(service)`：切换通信服务，返回旧服务。
- `sendEvent(event)`：通过当前通信服务发送 OneBot 事件。
- `onApi(action, handler)`：注册 API handler。
- `offApi(action)`：移除 API handler。
- `hasApi(action)`：检查 action 是否已注册。
- `setFallbackApi(handler)`：设置未注册 action 的兜底 handler。
- `isServiceRunning()`：查询底层服务运行状态。
- `takeServiceRuntimeError()`：取出并清空底层服务记录的运行时错误。

`onApi` handler 的签名：

```ts
server.onApi("get_login_info", (params, request) => {
  return {
    status: "ok",
    retcode: 0,
    data: {
      user_id: 10000,
      nickname: "mock",
    },
    echo: request.echo,
  };
});
```

handler 可以同步返回，也可以返回 `Promise`。如果返回值没有设置 `echo`，`Server`
会自动使用请求中的 `echo` 补齐。

默认行为：

- 已注册 action：调用对应 handler。
- 未注册 action 且存在 fallback：调用 fallback。
- 未注册 action 且无 fallback：返回 `status = "failed"`、`retcode = 1404`。
- handler 抛出异常：返回 `status = "failed"`、`retcode = 1500`，`data.message`
  为错误信息。

## WsService

`WsService` 是 WebSocket 服务端，适合让 OneBot 客户端主动连接 mock server。

```ts
import { Server, WsService } from "../index.ts";

const server = new Server(
  new WsService({
    hostname: "0.0.0.0",
    port: 8080,
    path: "/",
    accessToken: "token",
    maxConnections: 1,
  }),
);

server.onApi("send_group_msg", () => ({
  status: "ok",
  retcode: 0,
  data: {
    message_id: 2,
  },
}));

await server.startService();
await server.sendEvent({
  post_type: "message",
  message_type: "group",
  group_id: 10001,
  user_id: 10000,
  message: [{ type: "text", data: { text: "hello" } }],
  raw_message: "hello",
});
```

鉴权规则：

- 优先接受 `Authorization: Bearer <token>`。
- 同时兼容 query 参数 `access_token=<token>`。
- 未配置 `accessToken` 时不校验。

## WsReverseService

`WsReverseService` 是反向 WebSocket 客户端，适合连接由被测客户端启动的 reverse
WebSocket 服务。

```ts
import { Server, WsReverseService } from "../index.ts";

const server = new Server(
  new WsReverseService({
    url: "ws://127.0.0.1:8080/",
    accessToken: "token",
    reconnectIntervalMs: 10_000,
  }),
);

server.onApi("get_login_info", () => ({
  status: "ok",
  retcode: 0,
  data: {
    user_id: 10000,
    nickname: "mock",
  },
}));

await server.startService();
```

注意：Deno 原生 WebSocket 客户端不支持自定义 `Authorization` header，因此
`accessToken` 会通过 query 参数 `access_token` 携带。

## HttpService

`HttpService` 是 HTTP API 服务端，适合被 OneBot 客户端通过 HTTP POST 调用 API。

```ts
import { HttpService, Server } from "../index.ts";

const server = new Server(
  new HttpService({
    hostname: "0.0.0.0",
    port: 5700,
    prefix: "/",
    accessToken: "token",
  }),
);

server.onApi("get_status", () => ({
  status: "ok",
  retcode: 0,
  data: {
    online: true,
    good: true,
  },
}));

await server.startService();
```

请求格式：

- 请求方法：`POST`
- 请求路径：`/<action>`，若设置 `prefix: "/api"`，则为 `/api/<action>`
- 请求体：API params JSON
- 响应体：OneBot API response JSON

`HttpService` 只处理 API 请求，不支持 `sendEvent()`。如果需要 HTTP POST
事件推送，请使用 `HttpPostService`。

## HttpPostService

`HttpPostService` 是事件推送客户端，适合把 mock server 生成的事件 POST
到被测客户端。

```ts
import { HttpPostService, Server } from "../index.ts";

const server = new Server(
  new HttpPostService({
    url: "http://127.0.0.1:5701/",
    secret: "secret",
  }),
);

await server.startService();

await server.sendEvent({
  post_type: "notice",
  notice_type: "group_upload",
  group_id: 10001,
  user_id: 10000,
  file: {
    id: "file-id",
    name: "demo.txt",
    size: 12,
  },
});
```

配置 `secret` 后，服务会添加 `X-Signature: sha1=<hmac>` 请求头。

`HttpPostService` 只负责事件推送，不会触发 `onApi()`。

## SseService

`SseService` 是 SSE 事件服务端，适合让被测客户端订阅事件流。

```ts
import { Server, SseService } from "../index.ts";

const server = new Server(
  new SseService({
    hostname: "0.0.0.0",
    port: 5702,
    path: "/",
    accessToken: "token",
  }),
);

await server.startService();

await server.sendEvent({
  post_type: "meta_event",
  meta_event_type: "heartbeat",
  interval: 5000,
});
```

事件会以标准 SSE 格式发送：

```txt
data: {"post_type":"meta_event","meta_event_type":"heartbeat","interval":5000}
```

`SseService` 只负责事件推送，不会触发 `onApi()`。

## 常见组合

OneBot V11 的 HTTP API 和 HTTP POST 事件通常是两条通道。当前 `Server`
一次只持有一个 `CommunicationService`，因此可创建两个 server 分别管理：

```ts
import { HttpPostService, HttpService, Server } from "../index.ts";

const apiServer = new Server(new HttpService({ port: 5700 }));
const eventServer = new Server(
  new HttpPostService({
    url: "http://127.0.0.1:5701/",
  }),
);

apiServer.onApi("send_msg", () => ({
  status: "ok",
  retcode: 0,
  data: {
    message_id: 3,
  },
}));

await apiServer.startService();
await eventServer.startService();

await eventServer.sendEvent({
  post_type: "message",
  message_type: "private",
  user_id: 10000,
  message: [{ type: "text", data: { text: "hello" } }],
  raw_message: "hello",
});
```

## 错误处理

底层 service 会记录部分异步运行时错误，可通过 `takeServiceRuntimeError()` 读取：

```ts
const error = server.takeServiceRuntimeError();
if (error !== undefined) {
  console.error(error);
}
```

常见错误类型：

- `TaskIsRunningError`：重复启动同一个 service。
- `NotInstalledError`：service 尚未安装到 `Server`。
- `UnsupportedCapabilityError`：当前通信方式不支持调用的能力。
- `NoActiveConnectionError`：需要连接后才能发送事件，但当前没有可用连接。
