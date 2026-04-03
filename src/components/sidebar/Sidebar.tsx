import { For } from "solid-js";
import { useActiveTool } from "../../contexts/ToolContext";
import SidebarItem from "./SidebarItem";

export default function Sidebar() {
  const { activeTool, setActiveTool, tools } = useActiveTool();

  return (
    <nav class="flex w-56 flex-col gap-1 border-r border-qtools-200 bg-qtools-100 p-2 dark:border-qtools-700 dark:bg-qtools-900">
      <For each={tools}>
        {(tool) => (
          <SidebarItem
            tool={tool}
            active={activeTool().id === tool.id}
            onClick={() => setActiveTool(tool.id)}
          />
        )}
      </For>
    </nav>
  );
}
