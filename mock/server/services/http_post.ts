import type { OneBotEvent } from "../types.ts";
import { BaseService } from "./shared.ts";

export interface HttpPostServiceOptions {
  url: string;
  secret?: string;
}

export class HttpPostService extends BaseService {
  #url: string;
  #secret?: string;

  constructor(options: HttpPostServiceOptions) {
    super();
    this.#url = options.url;
    this.#secret = options.secret;
  }

  async start(): Promise<void> {
    this.ensureNotRunning();
    this.ensureInstalled();
    this.runtimeError = undefined;
    this.running = true;
  }

  stop(): void {
    this.running = false;
  }

  async sendEvent(event: OneBotEvent): Promise<void> {
    const body = JSON.stringify(event);
    const headers = new Headers({
      "Content-Type": "application/json",
    });
    if (this.#secret !== undefined) {
      headers.set(
        "X-Signature",
        `sha1=${await hmacSha1Hex(this.#secret, body)}`,
      );
    }

    const response = await fetch(this.#url, {
      method: "POST",
      headers,
      body,
    });
    if (!response.ok) {
      throw new Error(`HTTP POST event failed with status ${response.status}`);
    }
  }
}

async function hmacSha1Hex(secret: string, body: string): Promise<string> {
  const encoder = new TextEncoder();
  const key = await crypto.subtle.importKey(
    "raw",
    encoder.encode(secret),
    {
      name: "HMAC",
      hash: "SHA-1",
    },
    false,
    ["sign"],
  );
  const signature = await crypto.subtle.sign("HMAC", key, encoder.encode(body));
  return [...new Uint8Array(signature)]
    .map((byte) => byte.toString(16).padStart(2, "0"))
    .join("");
}
