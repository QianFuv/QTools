import { createSignal, Show } from "solid-js";
import { Dynamic } from "solid-js/web";
import tools from "../../tools/registry";

export default function ToolsSettings() {
  const toolsWithSettings = tools.filter((t) => t.settingsComponent);
  const [activeTab, setActiveTab] = createSignal(toolsWithSettings[0]?.id ?? "");

  const activeTool = () => toolsWithSettings.find((t) => t.id === activeTab());

  return (
    <Show
      when={toolsWithSettings.length > 0}
      fallback={
        <p class="text-sm text-qtools-500 dark:text-qtools-400">
          No tools have configurable settings.
        </p>
      }
    >
      <div class="flex flex-col gap-4">
        <div class="flex gap-1 border-b border-qtools-200 dark:border-qtools-700">
          {toolsWithSettings.map((tool) => (
            <button
              onClick={() => setActiveTab(tool.id)}
              class={`flex items-center gap-2 border-b-2 px-3 py-2 text-sm font-medium transition-colors ${
                activeTab() === tool.id
                  ? "border-qtools-500 text-qtools-900 dark:border-qtools-400 dark:text-qtools-50"
                  : "border-transparent text-qtools-500 hover:border-qtools-300 hover:text-qtools-700 dark:text-qtools-400 dark:hover:border-qtools-600 dark:hover:text-qtools-200"
              }`}
            >
              <span class="text-current">{tool.icon()}</span>
              {tool.name}
            </button>
          ))}
        </div>

        <Show when={activeTool()}>
          <Dynamic component={activeTool()!.settingsComponent!} />
        </Show>
      </div>
    </Show>
  );
}
