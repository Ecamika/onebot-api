import type {
  ApiHandler,
  CommunicationContext,
  CommunicationService,
  JsonValue,
  OneBotApiRequest,
  OneBotApiResponse,
  OneBotEvent,
  ServerOptions,
} from "./types.ts";

export class ServerBuilder {
  #service: CommunicationService;
  #fallbackApi?: ApiHandler;

  constructor(service: CommunicationService) {
    this.#service = service;
  }

  fallbackApi(handler: ApiHandler): this {
    this.#fallbackApi = handler;
    return this;
  }

  build(): Server {
    return new Server(this.#service, {
      fallbackApi: this.#fallbackApi,
    });
  }
}

export class Server {
  #service: CommunicationService;
  #handlers = new Map<string, ApiHandler>();
  #fallbackApi?: ApiHandler;
  #context: CommunicationContext;

  constructor(service: CommunicationService, options: ServerOptions = {}) {
    this.#service = service;
    this.#fallbackApi = options.fallbackApi;
    this.#context = {
      handleApi: (request) => this.#handleApi(request),
    };
    this.#service.install(this.#context);
  }

  static builder(service: CommunicationService): ServerBuilder {
    return new ServerBuilder(service);
  }

  startService(): Promise<void> {
    return this.#service.start();
  }

  stopService(): void {
    this.#service.stop();
  }

  restartService(): Promise<void> {
    return this.#service.restart();
  }

  changeService(service: CommunicationService): CommunicationService {
    service.install(this.#context);
    const oldService = this.#service;
    oldService.uninstall();
    this.#service = service;
    return oldService;
  }

  sendEvent(event: OneBotEvent): Promise<void> {
    return this.#service.sendEvent(event);
  }

  onApi(action: string, handler: ApiHandler): this {
    this.#handlers.set(action, handler);
    return this;
  }

  offApi(action: string): this {
    this.#handlers.delete(action);
    return this;
  }

  hasApi(action: string): boolean {
    return this.#handlers.has(action);
  }

  setFallbackApi(handler: ApiHandler): this {
    this.#fallbackApi = handler;
    return this;
  }

  isServiceRunning(): boolean {
    return this.#service.isRunning();
  }

  takeServiceRuntimeError(): Error | undefined {
    return this.#service.takeRuntimeError();
  }

  async #handleApi(request: OneBotApiRequest): Promise<OneBotApiResponse> {
    const params = request.params ?? {};
    const handler = this.#handlers.get(request.action) ?? this.#fallbackApi;
    if (handler === undefined) {
      return {
        status: "failed",
        retcode: 1404,
        data: null,
        echo: request.echo,
      };
    }

    try {
      const response = await handler(params, request);
      return withEcho(response, request.echo);
    } catch (error) {
      return {
        status: "failed",
        retcode: 1500,
        data: {
          message: error instanceof Error ? error.message : String(error),
        },
        echo: request.echo,
      };
    }
  }
}

function withEcho(
  response: OneBotApiResponse,
  echo: string | undefined,
): OneBotApiResponse {
  if (response.echo !== undefined || echo === undefined) {
    return response;
  }
  return {
    ...response,
    echo,
    data: response.data as JsonValue,
  };
}
