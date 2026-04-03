import { Dynamic } from "solid-js/web";
import { useActiveTool } from "../../contexts/ToolContext";

export default function ContentArea() {
  const { activeTool } = useActiveTool();

  return (
    <main class="flex flex-1 overflow-auto bg-maroon-50 dark:bg-maroon-950">
      <Dynamic component={activeTool().component} />
    </main>
  );
}
