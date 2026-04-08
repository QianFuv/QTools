import { For } from "solid-js";
import { useAgent } from "../../contexts/AgentContext";
import ConversationItem from "./ConversationItem";

export default function ConversationList() {
  const { conversations, activeConversation, selectConversation, createConversation } = useAgent();

  return (
    <div class="flex min-h-0 w-56 flex-col border-r border-qtools-200 bg-qtools-100 dark:border-qtools-700 dark:bg-qtools-900">
      <div class="p-2">
        <button
          onClick={() => createConversation()}
          class="flex w-full items-center justify-center rounded-lg bg-qtools-500 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-qtools-600"
        >
          New Chat
        </button>
      </div>
      <div class="flex flex-1 flex-col gap-1 overflow-y-auto p-2">
        <For each={conversations()}>
          {(conv) => (
            <ConversationItem
              conversation={conv}
              active={activeConversation()?.id === conv.id}
              onClick={() => selectConversation(conv.id)}
            />
          )}
        </For>
      </div>
    </div>
  );
}
