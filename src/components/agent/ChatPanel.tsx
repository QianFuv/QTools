import { For, Show, createEffect } from "solid-js";
import { useAgent } from "../../contexts/AgentContext";
import ChatMessage from "./ChatMessage";
import ChatInput from "./ChatInput";

export default function ChatPanel() {
  const { messages, activeConversation, sendMessage, isStreaming, streamingContent } = useAgent();
  let messagesEndRef: HTMLDivElement | undefined;

  const scrollToBottom = () => {
    messagesEndRef?.scrollIntoView({ behavior: "smooth" });
  };

  createEffect(() => {
    messages();
    streamingContent();
    scrollToBottom();
  });

  return (
    <div class="flex min-h-0 flex-1 flex-col bg-qtools-50 dark:bg-qtools-950">
      <Show
        when={activeConversation()}
        fallback={
          <div class="flex flex-1 items-center justify-center text-qtools-400 dark:text-qtools-600">
            <p class="text-lg">Select or create a conversation to start</p>
          </div>
        }
      >
        <div class="min-h-0 flex-1 overflow-y-auto p-4">
          <For each={messages()}>
            {(msg) => <ChatMessage message={msg} />}
          </For>
          <Show when={isStreaming() && streamingContent()}>
            <div class="mb-4 flex justify-start">
              <div class="max-w-[80%] rounded-2xl rounded-bl-sm bg-qtools-100 px-4 py-2 text-sm text-qtools-900 dark:bg-qtools-800 dark:text-qtools-100">
                <p class="whitespace-pre-wrap">{streamingContent()}</p>
              </div>
            </div>
          </Show>
          <Show when={isStreaming() && !streamingContent()}>
            <div class="mb-4 flex justify-start">
              <div class="flex items-center gap-1 rounded-2xl rounded-bl-sm bg-qtools-100 px-4 py-3 dark:bg-qtools-800">
                <span class="typing-dot h-2 w-2 rounded-full bg-qtools-400 dark:bg-qtools-500" />
                <span class="typing-dot animation-delay-200 h-2 w-2 rounded-full bg-qtools-400 dark:bg-qtools-500" />
                <span class="typing-dot animation-delay-400 h-2 w-2 rounded-full bg-qtools-400 dark:bg-qtools-500" />
              </div>
            </div>
          </Show>
          <div ref={messagesEndRef} />
        </div>
        <ChatInput
          onSend={(content) => sendMessage(content)}
          disabled={isStreaming()}
        />
      </Show>
    </div>
  );
}
