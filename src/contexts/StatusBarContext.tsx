import {
  createContext,
  createSignal,
  useContext,
  type ParentComponent,
} from "solid-js";
import type { StatusMessage, StatusSeverity } from "../types/status";

interface StatusBarContextValue {
  status: () => StatusMessage;
  setStatus: (text: string, severity?: StatusSeverity) => void;
  clearStatus: () => void;
}

const StatusBarContext = createContext<StatusBarContextValue>();

const DEFAULT_STATUS: StatusMessage = { text: "Ready", severity: "info" };

export const StatusBarProvider: ParentComponent = (props) => {
  const [status, setStatusSignal] = createSignal<StatusMessage>(DEFAULT_STATUS);

  const setStatus = (text: string, severity: StatusSeverity = "info") =>
    setStatusSignal({ text, severity });

  const clearStatus = () => setStatusSignal(DEFAULT_STATUS);

  return (
    <StatusBarContext.Provider value={{ status, setStatus, clearStatus }}>
      {props.children}
    </StatusBarContext.Provider>
  );
};

export function useStatusBar(): StatusBarContextValue {
  const ctx = useContext(StatusBarContext);
  if (!ctx) throw new Error("useStatusBar must be used within StatusBarProvider");
  return ctx;
}
