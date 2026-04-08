import ConversationList from "./ConversationList";
import ChatPanel from "./ChatPanel";

export default function AgentWorkspace() {
  return (
    <div class="flex min-h-0 h-full">
      <ConversationList />
      <ChatPanel />
    </div>
  );
}
