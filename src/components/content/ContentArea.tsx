import { Dynamic } from "solid-js/web";
import { useActiveTool } from "../../contexts/ToolContext";

export default function ContentArea() {
  const { activeTool } = useActiveTool();

  return (
    <main class="flex min-h-0 flex-1 overflow-hidden bg-qtools-50 dark:bg-qtools-950">
      <Dynamic component={activeTool().component} />
    </main>
  );
}
