import { For } from "solid-js";
import { useAgent } from "../../contexts/AgentContext";
import ConversationItem from "./ConversationItem";
import SettingsButton from "./SettingsButton";

interface ConversationListProps {
  onOpenSettings: () => void;
}

export default function ConversationList(props: ConversationListProps) {
  const { conversations, activeConversation, selectConversation, createConversation } = useAgent();

  return (
    <div class="flex min-h-0 w-56 flex-col border-r border-qtools-200 bg-qtools-100 dark:border-qtools-700 dark:bg-qtools-900">
      <div class="p-2">
        <button
          onClick={() => createConversation()}
          class="flex w-full items-center justify-center gap-2 rounded-lg bg-qtools-500 px-3 py-2 text-sm font-medium text-white transition-colors hover:bg-qtools-600"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="12" y1="5" x2="12" y2="19" />
            <line x1="5" y1="12" x2="19" y2="12" />
          </svg>
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
      <SettingsButton onClick={props.onOpenSettings} />
    </div>
  );
}
