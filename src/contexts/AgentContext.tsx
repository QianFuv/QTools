import {
  createContext,
  createSignal,
  onMount,
  useContext,
  type ParentComponent,
} from "solid-js";
import { invoke, Channel } from "@tauri-apps/api/core";
import type {
  AgentSettings,
  ChatMessage,
  Conversation,
  StreamEvent,
} from "../types/agent";

const DEFAULT_SETTINGS: AgentSettings = {
  base_url: "https://api.openai.com/v1",
  api_key: "",
  model: "gpt-4o",
  api_format: "openai_chat",
  system_prompt: "You are a helpful assistant.",
};

interface AgentContextValue {
  conversations: () => Conversation[];
  activeConversation: () => Conversation | null;
  messages: () => ChatMessage[];
  settings: () => AgentSettings;
  isStreaming: () => boolean;
  streamingContent: () => string;
  loadConversations: () => Promise<void>;
  selectConversation: (id: string) => Promise<void>;
  createConversation: () => Promise<void>;
  deleteConversation: (id: string) => Promise<void>;
  sendMessage: (content: string) => Promise<void>;
  saveSettings: (settings: AgentSettings) => Promise<void>;
}

const AgentContext = createContext<AgentContextValue>();

export const AgentProvider: ParentComponent = (props) => {
  const [conversations, setConversations] = createSignal<Conversation[]>([]);
  const [activeConversation, setActiveConversation] = createSignal<Conversation | null>(null);
  const [messages, setMessages] = createSignal<ChatMessage[]>([]);
  const [settings, setSettings] = createSignal<AgentSettings>(DEFAULT_SETTINGS);
  const [isStreaming, setIsStreaming] = createSignal(false);
  const [streamingContent, setStreamingContent] = createSignal("");

  const loadConversations = async () => {
    try {
      const convs = await invoke<Conversation[]>("get_conversations");
      setConversations(convs);
    } catch {
      setConversations([]);
    }
  };

  const selectConversation = async (id: string) => {
    const conv = conversations().find((c) => c.id === id);
    if (!conv) return;
    setActiveConversation(conv);
    try {
      const msgs = await invoke<ChatMessage[]>("get_messages", { conversationId: id });
      setMessages(msgs);
    } catch {
      setMessages([]);
    }
  };

  const createConversation = async () => {
    try {
      const conv = await invoke<Conversation>("create_conversation");
      setConversations((prev) => [conv, ...prev]);
      setActiveConversation(conv);
      setMessages([]);
    } catch {
      /* handled by status bar */
    }
  };

  const deleteConversation = async (id: string) => {
    try {
      await invoke("delete_conversation", { conversationId: id });
      setConversations((prev) => prev.filter((c) => c.id !== id));
      if (activeConversation()?.id === id) {
        setActiveConversation(null);
        setMessages([]);
      }
    } catch {
      /* handled by status bar */
    }
  };

  const sendMessage = async (content: string) => {
    const conv = activeConversation();
    if (!conv) return;

    const userMsg: ChatMessage = {
      id: crypto.randomUUID(),
      conversation_id: conv.id,
      role: "user",
      content,
      created_at: new Date().toISOString(),
    };
    setMessages((prev) => [...prev, userMsg]);
    setIsStreaming(true);
    setStreamingContent("");

    try {
      const channel = new Channel<StreamEvent>();
      channel.onmessage = (event: StreamEvent) => {
        if (event.event === "delta") {
          setStreamingContent((prev) => prev + event.data.content);
        } else if (event.event === "done") {
          setMessages((prev) => [...prev, event.data.message]);
          setStreamingContent("");
          setIsStreaming(false);
        } else if (event.event === "error") {
          setStreamingContent("");
          setIsStreaming(false);
        }
      };
      await invoke("send_message", {
        conversationId: conv.id,
        content,
        onEvent: channel,
      });
    } catch {
      setStreamingContent("");
      setIsStreaming(false);
    }
  };

  const loadSettings = async () => {
    try {
      const s = await invoke<AgentSettings>("get_agent_settings");
      setSettings(s);
    } catch {
      setSettings(DEFAULT_SETTINGS);
    }
  };

  const saveSettings = async (updated: AgentSettings) => {
    await invoke("save_agent_settings", { settings: updated });
    setSettings(updated);
  };

  onMount(() => {
    loadConversations();
    loadSettings();
  });

  return (
    <AgentContext.Provider
      value={{
        conversations,
        activeConversation,
        messages,
        settings,
        isStreaming,
        streamingContent,
        loadConversations,
        selectConversation,
        createConversation,
        deleteConversation,
        sendMessage,
        saveSettings,
      }}
    >
      {props.children}
    </AgentContext.Provider>
  );
};

export function useAgent(): AgentContextValue {
  const ctx = useContext(AgentContext);
  if (!ctx) throw new Error("useAgent must be used within AgentProvider");
  return ctx;
}
