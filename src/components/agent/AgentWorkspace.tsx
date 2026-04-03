import { createSignal } from "solid-js";
import ConversationList from "./ConversationList";
import ChatPanel from "./ChatPanel";
import SettingsPanel from "./SettingsPanel";

export default function AgentWorkspace() {
  const [settingsOpen, setSettingsOpen] = createSignal(false);

  return (
    <div class="flex min-h-0 h-full">
      <ConversationList
        onOpenSettings={() => setSettingsOpen(true)}
      />
      {settingsOpen() ? (
        <SettingsPanel onClose={() => setSettingsOpen(false)} />
      ) : (
        <ChatPanel />
      )}
    </div>
  );
}
