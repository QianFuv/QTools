import { createSignal, type JSX } from "solid-js";
import { Dynamic } from "solid-js/web";
import GeneralSettings from "./GeneralSettings";
import ToolsSettings from "./ToolsSettings";
import AgentSettings from "./AgentSettings";

interface Page {
  id: string;
  label: string;
  icon: () => JSX.Element;
  component: () => JSX.Element;
}

const pages: Page[] = [
  {
    id: "general",
    label: "General",
    icon: () => (
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
      </svg>
    ),
    component: GeneralSettings,
  },
  {
    id: "tools",
    label: "Tools",
    icon: () => (
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z" />
      </svg>
    ),
    component: ToolsSettings,
  },
  {
    id: "agent",
    label: "Agent",
    icon: () => (
      <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12 2a2 2 0 0 1 2 2c0 .74-.4 1.39-1 1.73V7h1a7 7 0 0 1 7 7h1a1 1 0 0 1 1 1v3a1 1 0 0 1-1 1h-1.27a7 7 0 0 1-12.46 0H6a1 1 0 0 1-1-1v-3a1 1 0 0 1 1-1h1a7 7 0 0 1 7-7h1V5.73c-.6-.34-1-.99-1-1.73a2 2 0 0 1 2-2z" />
        <circle cx="9" cy="13" r="1" />
        <circle cx="15" cy="13" r="1" />
        <path d="M9 17c.85.63 1.88 1 3 1s2.15-.37 3-1" />
      </svg>
    ),
    component: AgentSettings,
  },
];

export default function SettingsView() {
  const [activePage, setActivePage] = createSignal("general");

  const currentPage = () => pages.find((p) => p.id === activePage()) ?? pages[0];

  return (
    <div class="flex min-h-0 h-full">
      <nav class="flex w-56 shrink-0 flex-col border-r border-qtools-200 bg-qtools-100 dark:border-qtools-700 dark:bg-qtools-900">
        <div class="flex flex-col gap-1 p-2">
          {pages.map((page) => (
            <button
              onClick={() => setActivePage(page.id)}
              class={`flex items-center gap-2.5 rounded-lg px-3 py-2 text-sm font-medium transition-colors ${
                activePage() === page.id
                  ? "bg-qtools-500 text-white dark:bg-qtools-400 dark:text-qtools-950"
                  : "text-qtools-700 hover:bg-qtools-200 dark:text-qtools-300 dark:hover:bg-qtools-800"
              }`}
            >
              {page.icon()}
              {page.label}
            </button>
          ))}
        </div>
      </nav>

      <main class="flex flex-1 flex-col overflow-y-auto bg-qtools-50 p-6 dark:bg-qtools-950">
        <div class="mx-auto w-full max-w-xl">
          <h3 class="mb-5 text-base font-semibold text-qtools-900 dark:text-qtools-50">
            {currentPage().label}
          </h3>
          <Dynamic component={currentPage().component} />
        </div>
      </main>
    </div>
  );
}
