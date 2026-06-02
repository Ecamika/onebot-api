import type {
  CommunicationContext,
  CommunicationService,
  OneBotApiRequest,
  OneBotEvent,
} from "../types.ts";
import {
  NoActiveConnectionError,
  NotInstalledError,
  TaskIsRunningError,
} from "../types.ts";

export abstract class BaseService implements CommunicationService {
  protected context?: CommunicationContext;
  protected running = false;
  protected runtimeError?: Error;

  install(context: CommunicationContext): void {
    this.context = context;
  }

  uninstall(): void {
    this.stop();
    this.context = undefined;
  }

  async restart(): Promise<void> {
    this.stop();
    await this.start();
  }

  isRunning(): boolean {
    return this.running;
  }

  takeRuntimeError(): Error | undefined {
    const error = this.runtimeError;
    this.runtimeError = undefined;
    return error;
  }

  abstract start(): Promise<void>;
  abstract stop(): void;
  abstract sendEvent(event: OneBotEvent): Promise<void>;

  protected ensureInstalled(): CommunicationContext {
    if (this.context === undefined) {
      throw new NotInstalledError();
    }
    return this.context;
  }

  protected ensureNotRunning(): void {
    if (this.running) {
      throw new TaskIsRunningError();
    }
  }

  protected recordRuntimeError(error: unknown): void {
    this.runtimeError = error instanceof Error
      ? error
      : new Error(String(error));
  }
}

export function normalizePath(path: string | undefined): string {
  if (path === undefined || path === "") {
    return "/";
  }
  return path.startsWith("/") ? path : `/${path}`;
}

export function jsonResponse(value: unknown, status = 200): Response {
  return new Response(JSON.stringify(value), {
    status,
    headers: {
      "Content-Type": "application/json",
    },
  });
}

export function emptyResponse(status: number): Response {
  return new Response(null, { status });
}

export function verifyAccessToken(
  request: Request,
  accessToken: string | undefined,
): boolean {
  if (accessToken === undefined) {
    return true;
  }
  const authorization = request.headers.get("Authorization");
  if (authorization === `Bearer ${accessToken}`) {
    return true;
  }
  const url = new URL(request.url);
  return url.searchParams.get("access_token") === accessToken;
}

export function parseApiRequest(data: string): OneBotApiRequest | undefined {
  try {
    const value = JSON.parse(data) as Partial<OneBotApiRequest>;
    if (typeof value.action !== "string") {
      return undefined;
    }
    const params = isRecord(value.params) ? value.params : {};
    const echo = typeof value.echo === "string" ? value.echo : undefined;
    return {
      action: value.action,
      params,
      echo,
    };
  } catch {
    return undefined;
  }
}

export async function handleApiMessage(
  context: CommunicationContext,
  socket: WebSocket,
  data: string,
): Promise<void> {
  const request = parseApiRequest(data);
  if (request === undefined) {
    return;
  }
  const response = await context.handleApi(request);
  socket.send(JSON.stringify(response));
}

export function sendJson(socket: WebSocket, value: unknown): void {
  if (socket.readyState !== WebSocket.OPEN) {
    throw new NoActiveConnectionError();
  }
  socket.send(JSON.stringify(value));
}

export function isRecord(value: unknown): value is Record<string, never> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

export function text(data: string): Uint8Array {
  return new TextEncoder().encode(data);
}
