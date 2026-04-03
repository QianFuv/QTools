import type { Conversation } from "../../types/agent";
import { useAgent } from "../../contexts/AgentContext";

interface ConversationItemProps {
  conversation: Conversation;
  active: boolean;
  onClick: () => void;
}

export default function ConversationItem(props: ConversationItemProps) {
  const { deleteConversation } = useAgent();

  return (
    <button
      onClick={props.onClick}
      class={`group flex w-full items-center justify-between rounded-lg px-3 py-2 text-left text-sm transition-colors ${
        props.active
          ? "bg-qtools-200 font-medium text-qtools-900 dark:bg-qtools-800 dark:text-qtools-50"
          : "text-qtools-700 hover:bg-qtools-200 dark:text-qtools-300 dark:hover:bg-qtools-800"
      }`}
    >
      <span class="truncate">{props.conversation.title}</span>
      <span
        onClick={(e) => {
          e.stopPropagation();
          deleteConversation(props.conversation.id);
        }}
        class="hidden shrink-0 rounded p-0.5 text-qtools-400 hover:text-qtools-600 group-hover:inline-block dark:text-qtools-500 dark:hover:text-qtools-300"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <line x1="18" y1="6" x2="6" y2="18" />
          <line x1="6" y1="6" x2="18" y2="18" />
        </svg>
      </span>
    </button>
  );
}
