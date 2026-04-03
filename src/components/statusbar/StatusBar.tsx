import { useStatusBar } from "../../contexts/StatusBarContext";
import type { StatusSeverity } from "../../types/status";

const severityColor: Record<StatusSeverity, string> = {
  info: "bg-maroon-400 dark:bg-maroon-500",
  success: "bg-green-500",
  warning: "bg-yellow-500",
  error: "bg-red-500",
};

export default function StatusBar() {
  const { status } = useStatusBar();

  return (
    <footer class="flex h-7 items-center gap-2 border-t border-maroon-200 bg-maroon-100 px-3 dark:border-maroon-700 dark:bg-maroon-900">
      <span class={`size-2 shrink-0 rounded-full ${severityColor[status().severity]}`} />
      <span class="truncate text-xs text-maroon-600 dark:text-maroon-300">
        {status().text}
      </span>
    </footer>
  );
}
