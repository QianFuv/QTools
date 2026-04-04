import { SolidMarkdown } from "solid-markdown";
import type { ChatMessage as ChatMessageType } from "../../types/agent";
import { agentRemarkPlugins } from "./markdown";

interface ChatMessageProps {
  message: ChatMessageType;
}

export default function ChatMessage(props: ChatMessageProps) {
  const isUser = () => props.message.role === "user";

  return (
    <div class={`mb-4 flex ${isUser() ? "justify-end" : "justify-start"}`}>
      <div
        class={`max-w-[80%] rounded-2xl px-4 py-2 text-sm ${
          isUser()
            ? "rounded-br-sm bg-qtools-500 text-white"
            : "rounded-bl-sm bg-qtools-100 text-qtools-900 dark:bg-qtools-800 dark:text-qtools-100"
        }`}
      >
        {isUser() ? (
          <p class="whitespace-pre-wrap">{props.message.content}</p>
        ) : (
          <div class="overflow-x-auto">
            <div class="prose prose-sm max-w-none break-words dark:prose-invert">
              <SolidMarkdown
                children={props.message.content}
                renderingStrategy="memo"
                remarkPlugins={agentRemarkPlugins}
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
