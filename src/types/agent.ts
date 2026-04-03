export type ApiFormat = "openai_chat" | "openai_responses" | "anthropic";

export interface AgentSettings {
  base_url: string;
  api_key: string;
  model: string;
  api_format: ApiFormat;
  system_prompt: string;
}

export interface Conversation {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
}

export type MessageRole = "user" | "assistant";

export interface ChatMessage {
  id: string;
  conversation_id: string;
  role: MessageRole;
  content: string;
  created_at: string;
}

export type StreamEvent =
  | { event: "delta"; data: { content: string } }
  | { event: "done"; data: { message: ChatMessage } }
  | { event: "error"; data: { message: string } };
