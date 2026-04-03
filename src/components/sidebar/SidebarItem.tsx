import type { ToolDefinition } from "../../types/tool";

interface SidebarItemProps {
  tool: ToolDefinition;
  active: boolean;
  onClick: () => void;
}

export default function SidebarItem(props: SidebarItemProps) {
  return (
    <button
      onClick={props.onClick}
      class={`flex w-full items-center gap-3 rounded-lg px-3 py-2 text-sm transition-colors ${
        props.active
          ? "bg-maroon-200 font-medium text-maroon-900 dark:bg-maroon-800 dark:text-maroon-50"
          : "text-maroon-700 hover:bg-maroon-200 dark:text-maroon-300 dark:hover:bg-maroon-800"
      }`}
    >
      {props.tool.icon()}
      <span>{props.tool.name}</span>
    </button>
  );
}
