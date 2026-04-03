import {
  createContext,
  createEffect,
  createSignal,
  useContext,
  type ParentComponent,
} from "solid-js";

type ViewMode = "tools" | "agent";

interface ViewModeContextValue {
  viewMode: () => ViewMode;
  setViewMode: (mode: ViewMode) => void;
}

const ViewModeContext = createContext<ViewModeContextValue>();

function readStoredViewMode(): ViewMode {
  const stored = localStorage.getItem("qtools-view-mode");
  if (stored === "tools" || stored === "agent") return stored;
  return "tools";
}

export const ViewModeProvider: ParentComponent = (props) => {
  const [viewMode, setViewMode] = createSignal<ViewMode>(readStoredViewMode());

  createEffect(() => {
    localStorage.setItem("qtools-view-mode", viewMode());
  });

  return (
    <ViewModeContext.Provider value={{ viewMode, setViewMode }}>
      {props.children}
    </ViewModeContext.Provider>
  );
};

export function useViewMode(): ViewModeContextValue {
  const ctx = useContext(ViewModeContext);
  if (!ctx) throw new Error("useViewMode must be used within ViewModeProvider");
  return ctx;
}
