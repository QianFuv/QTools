import { ThemeProvider } from "./contexts/ThemeContext";
import { ViewModeProvider } from "./contexts/ViewModeContext";
import { ToolProvider } from "./contexts/ToolContext";
import { AgentProvider } from "./contexts/AgentContext";
import { StatusBarProvider } from "./contexts/StatusBarContext";
import AppShell from "./layouts/AppShell";
import { ExternalLinkConfirmDialog } from "./components/agent/markdown";

export default function App() {
  return (
    <ThemeProvider>
      <ViewModeProvider>
        <ToolProvider>
          <AgentProvider>
            <StatusBarProvider>
              <AppShell />
              <ExternalLinkConfirmDialog />
            </StatusBarProvider>
          </AgentProvider>
        </ToolProvider>
      </ViewModeProvider>
    </ThemeProvider>
  );
}
