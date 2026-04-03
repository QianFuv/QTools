import {
  createContext,
  createMemo,
  createSignal,
  useContext,
  type ParentComponent,
} from "solid-js";
import type { ToolDefinition } from "../types/tool";
import tools from "../tools/registry";

interface ToolContextValue {
  activeTool: () => ToolDefinition;
  setActiveTool: (toolId: string) => void;
  tools: ToolDefinition[];
}

const ToolContext = createContext<ToolContextValue>();

export const ToolProvider: ParentComponent = (props) => {
  const [activeToolId, setActiveToolId] = createSignal(tools[0].id);

  const activeTool = createMemo(() => tools.find((t) => t.id === activeToolId()) ?? tools[0]);

  const setActiveTool = (toolId: string) => {
    if (tools.some((t) => t.id === toolId)) {
      setActiveToolId(toolId);
    }
  };

  return (
    <ToolContext.Provider value={{ activeTool, setActiveTool, tools }}>
      {props.children}
    </ToolContext.Provider>
  );
};

export function useActiveTool(): ToolContextValue {
  const ctx = useContext(ToolContext);
  if (!ctx) throw new Error("useActiveTool must be used within ToolProvider");
  return ctx;
}
