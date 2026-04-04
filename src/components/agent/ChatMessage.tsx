import type { ChatMessage as ChatMessageType } from "../../types/agent";
import {
  AgentMarkdown,
  agentMarkdownClass,
} from "./markdown";

interface ChatMessageProps {
  message: ChatMessageType;
}

const bubbleClass = "w-full max-w-[48rem] min-w-0 rounded-2xl px-4 py-2 text-sm";

export default function ChatMessage(props: ChatMessageProps) {
  const isUser = () => props.message.role === "user";

  return (
    <div class={`mb-4 flex ${isUser() ? "justify-end" : "justify-start"}`}>
      <div
        class={`${bubbleClass} ${
          isUser()
            ? "rounded-br-sm bg-qtools-500 text-white"
            : "rounded-bl-sm bg-qtools-100 text-qtools-900 dark:bg-qtools-800 dark:text-qtools-100"
        }`}
      >
        {isUser() ? (
          <div class="min-w-0 overflow-x-auto">
            <p class="whitespace-pre-wrap break-words">{props.message.content}</p>
          </div>
        ) : (
          <div class="min-w-0 overflow-x-auto">
            <div class={agentMarkdownClass}>
              <AgentMarkdown content={props.message.content} renderingStrategy="memo" />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
