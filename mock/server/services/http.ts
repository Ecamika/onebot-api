import type { OneBotEvent } from "../types.ts";
import { UnsupportedCapabilityError } from "../types.ts";
import {
  BaseService,
  emptyResponse,
  isRecord,
  jsonResponse,
  normalizePath,
  verifyAccessToken,
} from "./shared.ts";

export interface HttpServiceOptions {
  hostname?: string;
  port: number;
  prefix?: string;
  accessToken?: string;
}

export class HttpService extends BaseService {
  #options: Required<Omit<HttpServiceOptions, "accessToken">> & {
    accessToken?: string;
  };
  #server?: Deno.HttpServer;

  constructor(options: HttpServiceOptions) {
    super();
    this.#options = {
      hostname: options.hostname ?? "0.0.0.0",
      port: options.port,
      prefix: normalizePath(options.prefix),
      accessToken: options.accessToken,
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
    }, async (request) => {
      try {
        if (!verifyAccessToken(request, this.#options.accessToken)) {
          return emptyResponse(403);
        }
        if (request.method !== "POST") {
          return emptyResponse(405);
        }

        const url = new URL(request.url);
        const action = this.#extractAction(url.pathname);
        if (action === undefined) {
          return emptyResponse(404);
        }

        const body = await request.text();
        const params = body === "" ? {} : JSON.parse(body);
        if (!isRecord(params)) {
          return emptyResponse(400);
        }

        const response = await context.handleApi({
          action,
          params,
        });
        return jsonResponse(response);
      } catch (error) {
        this.recordRuntimeError(error);
        return emptyResponse(500);
      }
    });
    this.running = true;
  }

  stop(): void {
    this.#server?.shutdown();
    this.#server = undefined;
    this.running = false;
  }

  async sendEvent(_event: OneBotEvent): Promise<void> {
    throw new UnsupportedCapabilityError(
      "HttpService does not support event sending",
    );
  }

  #extractAction(pathname: string): string | undefined {
    const prefix = this.#options.prefix;
    if (prefix === "/") {
      const action = pathname.slice(1);
      return action === "" ? undefined : decodeURIComponent(action);
    }
    if (!pathname.startsWith(`${prefix}/`)) {
      return undefined;
    }
    const action = pathname.slice(prefix.length + 1);
    return action === "" ? undefined : decodeURIComponent(action);
  }
}
