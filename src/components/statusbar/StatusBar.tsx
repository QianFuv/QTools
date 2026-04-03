import { useStatusBar } from "../../contexts/StatusBarContext";
import type { StatusSeverity } from "../../types/status";

const severityColor: Record<StatusSeverity, string> = {
  info: "bg-qtools-400 dark:bg-qtools-500",
  success: "bg-green-500",
  warning: "bg-yellow-500",
  error: "bg-red-500",
};

export default function StatusBar() {
  const { status } = useStatusBar();

  return (
    <footer class="flex h-7 items-center gap-2 border-t border-qtools-200 bg-qtools-100 px-3 dark:border-qtools-700 dark:bg-qtools-900">
      <span class={`size-2 shrink-0 rounded-full ${severityColor[status().severity]}`} />
      <span class="truncate text-xs text-qtools-600 dark:text-qtools-300">
        {status().text}
      </span>
    </footer>
  );
}
