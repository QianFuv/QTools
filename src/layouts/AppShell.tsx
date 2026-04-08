import { createSignal, Show } from "solid-js";
import { useViewMode } from "../contexts/ViewModeContext";
import Banner from "../components/banner/Banner";
import Sidebar from "../components/sidebar/Sidebar";
import ContentArea from "../components/content/ContentArea";
import AgentWorkspace from "../components/agent/AgentWorkspace";
import SettingsView from "../components/settings/SettingsView";
import StatusBar from "../components/statusbar/StatusBar";

export default function AppShell() {
  const { viewMode } = useViewMode();
  const [settingsOpen, setSettingsOpen] = createSignal(false);

  return (
    <div class="grid h-screen grid-rows-[auto_1fr_auto]">
      <Banner
        settingsOpen={settingsOpen()}
        onToggleSettings={() => setSettingsOpen((v) => !v)}
      />
      <Show
        when={!settingsOpen()}
        fallback={<SettingsView />}
      >
        <Show
          when={viewMode() === "tools"}
          fallback={<AgentWorkspace />}
        >
          <div class="grid min-h-0 grid-cols-[auto_1fr]">
            <Sidebar />
            <ContentArea />
          </div>
        </Show>
      </Show>
      <StatusBar />
    </div>
  );
}
