import { Server, WsReverseService } from "../server/index.ts";
import type {
  JsonValue,
  OneBotApiRequest,
  OneBotApiResponse,
  OneBotEvent,
} from "../server/index.ts";

type Args = Record<string, string | boolean>;

const args = parseArgs(Deno.args);

if (hasFlag(args, "help")) {
  printHelp();
  Deno.exit(0);
}

const selfId = numberArg(args, "self-id", 10000);
const userId = numberArg(args, "user-id", 10001);
const groupId = numberArg(args, "group-id", 10002);
const message = stringArg(args, "message", "hello from mock reverse websocket");
const eventType = stringArg(args, "event", "private");
const intervalMs = numberArg(args, "interval-ms", 0);

const server = new Server(
  new WsReverseService({
    url: stringArg(args, "url", "ws://127.0.0.1:8080/"),
    accessToken: optionalStringArg(args, "access-token"),
    reconnectIntervalMs: numberArg(args, "reconnect-interval-ms", 10_000),
  }),
);

registerDefaultApis(server, selfId);

await server.startService();
console.log(
  `mock reverse websocket client connecting to ${
    stringArg(args, "url", "ws://127.0.0.1:8080/")
  }`,
);

const sendConfiguredEvent = async () => {
  await server.sendEvent(
    buildEvent(eventType, selfId, userId, groupId, message),
  );
};

if (hasFlag(args, "send-on-start")) {
  setTimeout(() => {
    sendConfiguredEvent().catch((error) =>
      console.error(error instanceof Error ? error.message : String(error))
    );
  }, numberArg(args, "send-delay-ms", 1000));
}

let timer: number | undefined;
if (intervalMs > 0) {
  timer = setInterval(() => {
    sendConfiguredEvent().catch((error) =>
      console.error(error instanceof Error ? error.message : String(error))
    );
  }, intervalMs);
}

await waitForShutdown(() => {
  if (timer !== undefined) {
    clearInterval(timer);
  }
  server.stopService();
});

function registerDefaultApis(server: Server, selfId: number): void {
  server.onApi("get_login_info", (_params, request) =>
    ok({
      user_id: selfId,
      nickname: "mock",
    }, request));
  server.onApi("get_status", (_params, request) =>
    ok({
      online: true,
      good: true,
    }, request));
  server.onApi("send_msg", (_params, request) =>
    ok({
      message_id: Date.now(),
    }, request));
  server.onApi("send_private_msg", (_params, request) =>
    ok({
      message_id: Date.now(),
    }, request));
  server.onApi("send_group_msg", (_params, request) =>
    ok({
      message_id: Date.now(),
    }, request));
}

function ok(data: JsonValue, request: OneBotApiRequest): OneBotApiResponse {
  return {
    status: "ok",
    retcode: 0,
    data,
    echo: request.echo,
  };
}

function buildEvent(
  eventType: string,
  selfId: number,
  userId: number,
  groupId: number,
  message: string,
): OneBotEvent {
  const base = {
    time: Math.floor(Date.now() / 1000),
    self_id: selfId,
  };
  if (eventType === "group") {
    return {
      ...base,
      post_type: "message",
      message_type: "group",
      sub_type: "normal",
      group_id: groupId,
      user_id: userId,
      message_id: Date.now(),
      message: [{ type: "text", data: { text: message } }],
      raw_message: message,
      font: 0,
    };
  }
  if (eventType === "heartbeat") {
    return {
      ...base,
      post_type: "meta_event",
      meta_event_type: "heartbeat",
      status: {
        online: true,
        good: true,
      },
      interval: 5000,
    };
  }
  return {
    ...base,
    post_type: "message",
    message_type: "private",
    sub_type: "friend",
    user_id: userId,
    message_id: Date.now(),
    message: [{ type: "text", data: { text: message } }],
    raw_message: message,
    font: 0,
  };
}

function parseArgs(values: string[]): Args {
  const result: Args = {};
  for (let index = 0; index < values.length; index += 1) {
    const value = values[index];
    if (!value.startsWith("--")) {
      continue;
    }
    const option = value.slice(2);
    const equalIndex = option.indexOf("=");
    if (equalIndex !== -1) {
      result[option.slice(0, equalIndex)] = option.slice(equalIndex + 1);
      continue;
    }
    const next = values[index + 1];
    if (next !== undefined && !next.startsWith("--")) {
      result[option] = next;
      index += 1;
    } else {
      result[option] = true;
    }
  }
  return result;
}

function hasFlag(args: Args, name: string): boolean {
  return args[name] === true;
}

function stringArg(args: Args, name: string, fallback: string): string {
  const value = args[name];
  return typeof value === "string" ? value : fallback;
}

function optionalStringArg(args: Args, name: string): string | undefined {
  const value = args[name];
  return typeof value === "string" && value !== "" ? value : undefined;
}

function numberArg(args: Args, name: string, fallback: number): number {
  const value = args[name];
  if (typeof value !== "string") {
    return fallback;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) ? parsed : fallback;
}

function waitForShutdown(cleanup: () => void): Promise<void> {
  return new Promise((resolve) => {
    const shutdown = () => {
      cleanup();
      resolve();
    };
    Deno.addSignalListener("SIGINT", shutdown);
  });
}

function printHelp(): void {
  console.log(`Usage: deno run --allow-net mock/mock/ws_reverse.ts [options]

Options:
  --url <url>                Reverse WebSocket URL, default ws://127.0.0.1:8080/
  --access-token <token>     Optional OneBot access token
  --reconnect-interval-ms <ms>
  --self-id <id>             Bot self id, default 10000
  --user-id <id>             Event user id, default 10001
  --group-id <id>            Event group id, default 10002
  --event <private|group|heartbeat>
  --message <text>           Text message for message events
  --send-on-start            Send one event after startup
  --send-delay-ms <ms>       Delay before --send-on-start event, default 1000
  --interval-ms <ms>         Send configured event repeatedly
  --help                     Print this help`);
}
