import type { OneBotEvent } from "../types.ts";
import { NoActiveConnectionError } from "../types.ts";
import { BaseService, handleApiMessage, sendJson } from "./shared.ts";

export interface WsReverseServiceOptions {
  url: string;
  accessToken?: string;
  reconnectIntervalMs?: number;
}

export class WsReverseService extends BaseService {
  #url: string;
  #accessToken?: string;
  #reconnectIntervalMs: number;
  #socket?: WebSocket;
  #reconnectTimer?: number;
  #stopping = false;

  constructor(options: WsReverseServiceOptions) {
    super();
    this.#url = options.url;
    this.#accessToken = options.accessToken;
    this.#reconnectIntervalMs = options.reconnectIntervalMs ?? 10_000;
  }

  async start(): Promise<void> {
    this.ensureNotRunning();
    this.ensureInstalled();
    this.runtimeError = undefined;
    this.#stopping = false;
    this.running = true;
    this.#connect();
  }

  stop(): void {
    this.#stopping = true;
    if (this.#reconnectTimer !== undefined) {
      clearTimeout(this.#reconnectTimer);
      this.#reconnectTimer = undefined;
    }
    this.#socket?.close();
    this.#socket = undefined;
    this.running = false;
  }

  async sendEvent(event: OneBotEvent): Promise<void> {
    const socket = this.#socket;
    if (socket === undefined || socket.readyState !== WebSocket.OPEN) {
      throw new NoActiveConnectionError();
    }
    sendJson(socket, event);
  }

  #connect(): void {
    const context = this.ensureInstalled();
    const socket = new WebSocket(this.#buildUrl());
    this.#socket = socket;

    socket.onmessage = (event) => {
      if (typeof event.data === "string") {
        handleApiMessage(context, socket, event.data).catch((error) =>
          this.recordRuntimeError(error)
        );
      }
    };
    socket.onclose = () => this.#scheduleReconnect(socket);
    socket.onerror = (event) => {
      this.recordRuntimeError(
        new Error(`reverse websocket error: ${event.type}`),
      );
    };
  }

  #scheduleReconnect(socket: WebSocket): void {
    if (this.#socket === socket) {
      this.#socket = undefined;
    }
    if (this.#stopping || !this.running) {
      return;
    }
    this.#reconnectTimer = setTimeout(() => {
      this.#reconnectTimer = undefined;
      this.#connect();
    }, this.#reconnectIntervalMs);
  }

  #buildUrl(): string {
    if (this.#accessToken === undefined) {
      return this.#url;
    }
    const url = new URL(this.#url);
    url.searchParams.set("access_token", this.#accessToken);
    return url.toString();
  }
}
