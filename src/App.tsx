import { ThemeProvider } from "./contexts/ThemeContext";
import { ToolProvider } from "./contexts/ToolContext";
import { StatusBarProvider } from "./contexts/StatusBarContext";
import AppShell from "./layouts/AppShell";

export default function App() {
  return (
    <ThemeProvider>
      <ToolProvider>
        <StatusBarProvider>
          <AppShell />
        </StatusBarProvider>
      </ToolProvider>
    </ThemeProvider>
  );
}
