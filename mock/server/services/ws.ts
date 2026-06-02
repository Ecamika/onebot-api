import type { OneBotEvent } from "../types.ts";
import { NoActiveConnectionError } from "../types.ts";
import {
  BaseService,
  emptyResponse,
  handleApiMessage,
  normalizePath,
  sendJson,
  verifyAccessToken,
} from "./shared.ts";

export interface WsServiceOptions {
  hostname?: string;
  port: number;
  path?: string;
  accessToken?: string;
  maxConnections?: number;
}

export class WsService extends BaseService {
  #options: Required<Omit<WsServiceOptions, "accessToken">> & {
    accessToken?: string;
  };
  #server?: Deno.HttpServer;
  #connections = new Set<WebSocket>();

  constructor(options: WsServiceOptions) {
    super();
    this.#options = {
      hostname: options.hostname ?? "0.0.0.0",
      port: options.port,
      path: normalizePath(options.path),
      accessToken: options.accessToken,
      maxConnections: options.maxConnections ?? 1,
    };
  }

  async start(): Promise<void> {
    this.ensureNotRunning();
    const context = this.ensureInstalled();
    this.runtimeError = undefined;

    this.#server = Deno.serve({
      hostname: this.#options.hostname,
      port: this.#options.port,
      onListen: () => {},
    }, (request) => {
      try {
        const url = new URL(request.url);
        if (url.pathname !== this.#options.path) {
          return emptyResponse(404);
        }
        if (!verifyAccessToken(request, this.#options.accessToken)) {
          return emptyResponse(403);
        }
        if (this.#connections.size >= this.#options.maxConnections) {
          return emptyResponse(403);
        }

        const { socket, response } = Deno.upgradeWebSocket(request);
        this.#connections.add(socket);
        socket.onmessage = (event) => {
          if (typeof event.data === "string") {
            handleApiMessage(context, socket, event.data).catch((error) =>
              this.recordRuntimeError(error)
            );
          }
        };
        socket.onclose = () => this.#connections.delete(socket);
        socket.onerror = (event) => {
          this.recordRuntimeError(new Error(`websocket error: ${event.type}`));
          this.#connections.delete(socket);
        };
        return response;
      } catch (error) {
        this.recordRuntimeError(error);
        return emptyResponse(500);
      }
    });
    this.running = true;
  }

  stop(): void {
    for (const socket of this.#connections) {
      socket.close();
    }
    this.#connections.clear();
    this.#server?.shutdown();
    this.#server = undefined;
    this.running = false;
  }

  async sendEvent(event: OneBotEvent): Promise<void> {
    if (this.#connections.size === 0) {
      throw new NoActiveConnectionError();
    }
    for (const socket of this.#connections) {
      sendJson(socket, event);
    }
  }
}
