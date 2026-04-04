import { For, Show, createEffect } from "solid-js";
import { SolidMarkdown } from "solid-markdown";
import { useAgent } from "../../contexts/AgentContext";
import ChatMessage from "./ChatMessage";
import ChatInput from "./ChatInput";

export default function ChatPanel() {
  const { messages, activeConversation, sendMessage, isStreaming, streamingContent, toolCalls } = useAgent();
  let messagesEndRef: HTMLDivElement | undefined;

  const scrollToBottom = () => {
    messagesEndRef?.scrollIntoView({ behavior: "smooth" });
  };

  createEffect(() => {
    messages();
    streamingContent();
    toolCalls();
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
          <Show when={isStreaming() && toolCalls().length > 0}>
            <div class="mb-4">
              <For each={toolCalls()}>
                {(call) => (
                  <div class="mb-2 rounded-lg border border-qtools-200 bg-qtools-50 px-3 py-2 text-xs dark:border-qtools-700 dark:bg-qtools-900">
                    <div class="flex items-center gap-1.5 font-medium text-qtools-500 dark:text-qtools-400">
                      <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />
                      </svg>
                      {call.name}
                      <span class="font-normal text-qtools-400 dark:text-qtools-500">({call.args})</span>
                    </div>
                  </div>
                )}
              </For>
            </div>
          </Show>
          <Show when={isStreaming() && streamingContent()}>
            <div class="mb-4 flex justify-start">
              <div class="max-w-[80%] rounded-2xl rounded-bl-sm bg-qtools-100 px-4 py-2 text-sm text-qtools-900 dark:bg-qtools-800 dark:text-qtools-100">
                <div class="prose prose-sm max-w-none break-words dark:prose-invert">
                  <SolidMarkdown children={streamingContent()} renderingStrategy="reconcile" />
                </div>
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
