import { getCurrentWindow } from "@tauri-apps/api/window";
import AgentToggle from "./AgentToggle";
import DarkModeToggle from "./DarkModeToggle";

interface BannerProps {
  settingsOpen: boolean;
  onToggleSettings: () => void;
}

export default function Banner(props: BannerProps) {
  const appWindow = getCurrentWindow();

  return (
    <header
      onMouseDown={() => appWindow.startDragging()}
      class="flex h-11 items-center justify-between border-b border-qtools-700 bg-qtools-900 px-4 select-none dark:border-qtools-800 dark:bg-qtools-950"
    >
      <span class="text-sm font-bold tracking-wide text-qtools-50">
        QTools
      </span>
      <div
        class="flex items-center gap-1"
        onMouseDown={(e) => e.stopPropagation()}
      >
        <AgentToggle />
        <button
          onClick={props.onToggleSettings}
          class={`rounded-lg p-1.5 transition-colors ${
            props.settingsOpen
              ? "bg-qtools-600 text-white"
              : "text-qtools-200 hover:bg-qtools-700 hover:text-qtools-50"
          }`}
          title="Settings"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
            <circle cx="12" cy="12" r="3" />
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
          </svg>
        </button>
        <DarkModeToggle />
        <button
          onClick={() => appWindow.minimize()}
          class="rounded-lg p-1.5 text-qtools-200 transition-colors hover:bg-qtools-700 hover:text-qtools-50"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="5" y1="12" x2="19" y2="12" /></svg>
        </button>
        <button
          onClick={() => appWindow.toggleMaximize()}
          class="rounded-lg p-1.5 text-qtools-200 transition-colors hover:bg-qtools-700 hover:text-qtools-50"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><rect x="3" y="3" width="18" height="18" rx="2" /></svg>
        </button>
        <button
          onClick={() => appWindow.close()}
          class="rounded-lg p-1.5 text-qtools-200 transition-colors hover:bg-red-500 hover:text-white"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="18" y1="6" x2="6" y2="18" /><line x1="6" y1="6" x2="18" y2="18" /></svg>
        </button>
      </div>
    </header>
  );
}
