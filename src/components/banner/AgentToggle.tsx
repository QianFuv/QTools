import { useViewMode } from "../../contexts/ViewModeContext";

export default function AgentToggle() {
  const { viewMode, setViewMode } = useViewMode();

  const toggle = () => {
    setViewMode(viewMode() === "tools" ? "agent" : "tools");
  };

  return (
    <button
      onClick={toggle}
      class={`rounded-lg p-1.5 transition-colors ${
        viewMode() === "agent"
          ? "bg-qtools-600 text-qtools-50"
          : "text-qtools-200 hover:bg-qtools-700 hover:text-qtools-50"
      }`}
      title="Agent Workspace"
    >
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="18"
        height="18"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" />
      </svg>
    </button>
  );
}
