import type { OneBotEvent } from "../types.ts";
import {
  BaseService,
  emptyResponse,
  normalizePath,
  text,
  verifyAccessToken,
} from "./shared.ts";

export interface SseServiceOptions {
  hostname?: string;
  port: number;
  path?: string;
  accessToken?: string;
}

export class SseService extends BaseService {
  #options: Required<Omit<SseServiceOptions, "accessToken">> & {
    accessToken?: string;
  };
  #server?: Deno.HttpServer;
  #clients = new Set<ReadableStreamDefaultController<Uint8Array>>();

  constructor(options: SseServiceOptions) {
    super();
    this.#options = {
      hostname: options.hostname ?? "0.0.0.0",
      port: options.port,
      path: normalizePath(options.path),
      accessToken: options.accessToken,
    };
  }

  async start(): Promise<void> {
    this.ensureNotRunning();
    this.ensureInstalled();
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

        let client:
          | ReadableStreamDefaultController<Uint8Array>
          | undefined;
        const stream = new ReadableStream<Uint8Array>({
          start: (controller) => {
            client = controller;
            this.#clients.add(controller);
            controller.enqueue(text(": connected\n\n"));
          },
          cancel: () => {
            if (client !== undefined) {
              this.#clients.delete(client);
            }
          },
        });
        return new Response(stream, {
          headers: {
            "Content-Type": "text/event-stream",
            "Cache-Control": "no-cache",
            "Connection": "keep-alive",
          },
        });
      } catch (error) {
        this.recordRuntimeError(error);
        return emptyResponse(500);
      }
    });
    this.running = true;
  }

  stop(): void {
    for (const client of this.#clients) {
      try {
        client.close();
      } catch {
        // already closed
      }
    }
    this.#clients.clear();
    this.#server?.shutdown();
    this.#server = undefined;
    this.running = false;
  }

  async sendEvent(event: OneBotEvent): Promise<void> {
    const payload = text(`data: ${JSON.stringify(event)}\n\n`);
    for (const client of [...this.#clients]) {
      try {
        client.enqueue(payload);
      } catch (error) {
        this.recordRuntimeError(error);
        this.#clients.delete(client);
      }
    }
  }
}
