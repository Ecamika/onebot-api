export type JsonValue =
  | null
  | boolean
  | number
  | string
  | JsonValue[]
  | { [key: string]: JsonValue };

export interface OneBotApiRequest {
  action: string;
  params?: Record<string, JsonValue>;
  echo?: string;
}

export interface OneBotApiResponse<T = JsonValue> {
  status: "ok" | "failed";
  retcode: number;
  data: T;
  echo?: string;
}

export type OneBotEvent = Record<string, JsonValue>;

export type ApiHandler = (
  params: Record<string, JsonValue>,
  request: OneBotApiRequest,
) => OneBotApiResponse | Promise<OneBotApiResponse>;

export interface CommunicationContext {
  handleApi(request: OneBotApiRequest): Promise<OneBotApiResponse>;
}

export interface CommunicationService {
  install(context: CommunicationContext): void;
  uninstall(): void;
  start(): Promise<void>;
  stop(): void;
  restart(): Promise<void>;
  isRunning(): boolean;
  takeRuntimeError(): Error | undefined;
  sendEvent(event: OneBotEvent): Promise<void>;
}

export interface ServerOptions {
  fallbackApi?: ApiHandler;
}

export class TaskIsRunningError extends Error {
  constructor() {
    super("communication service task is already running");
    this.name = "TaskIsRunningError";
  }
}

export class NotInstalledError extends Error {
  constructor() {
    super("communication service is not installed");
    this.name = "NotInstalledError";
  }
}

export class UnsupportedCapabilityError extends Error {
  constructor(
    message = "communication service does not support this capability",
  ) {
    super(message);
    this.name = "UnsupportedCapabilityError";
  }
}

export class NoActiveConnectionError extends Error {
  constructor(message = "communication service has no active connection") {
    super(message);
    this.name = "NoActiveConnectionError";
  }
}
