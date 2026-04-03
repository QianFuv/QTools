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
          ? "bg-qtools-200 font-medium text-qtools-900 dark:bg-qtools-800 dark:text-qtools-50"
          : "text-qtools-700 hover:bg-qtools-200 dark:text-qtools-300 dark:hover:bg-qtools-800"
      }`}
    >
      {props.tool.icon()}
      <span>{props.tool.name}</span>
    </button>
  );
}
