import { HttpService, Server } from "../server/index.ts";
import type {
  JsonValue,
  OneBotApiRequest,
  OneBotApiResponse,
} from "../server/index.ts";

type Args = Record<string, string | boolean>;

const args = parseArgs(Deno.args);

if (hasFlag(args, "help")) {
  printHelp();
  Deno.exit(0);
}

const selfId = numberArg(args, "self-id", 10000);
const hostname = stringArg(args, "hostname", "0.0.0.0");
const port = numberArg(args, "port", 5700);
const prefix = stringArg(args, "prefix", "/");

const server = new Server(
  new HttpService({
    hostname,
    port,
    prefix,
    accessToken: optionalStringArg(args, "access-token"),
  }),
);

registerDefaultApis(server, selfId);

server.setFallbackApi((_params, request) => ({
  status: "ok",
  retcode: 0,
  data: null,
  echo: request.echo,
}));

await server.startService();
console.log(`mock http api server listening on ${hostname}:${port}${prefix}`);

await waitForShutdown(() => server.stopService());

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
  console.log(`Usage: deno run --allow-net mock/mock/http.ts [options]

Options:
  --hostname <host>          Listen host, default 0.0.0.0
  --port <port>              Listen port, default 5700
  --prefix <path>            API route prefix, default /
  --access-token <token>     Optional OneBot access token
  --self-id <id>             Bot self id, default 10000
  --help                     Print this help`);
}
