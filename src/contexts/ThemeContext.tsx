import {
  createContext,
  createEffect,
  createSignal,
  useContext,
  type ParentComponent,
} from "solid-js";

type Theme = "light" | "dark";

interface ThemeContextValue {
  theme: () => Theme;
  setTheme: (theme: Theme) => void;
  toggleTheme: () => void;
}

const ThemeContext = createContext<ThemeContextValue>();

function readStoredTheme(): Theme {
  const stored = localStorage.getItem("qtools-theme");
  return stored === "dark" ? "dark" : "light";
}

export const ThemeProvider: ParentComponent = (props) => {
  const [theme, setTheme] = createSignal<Theme>(readStoredTheme());

  createEffect(() => {
    const current = theme();
    localStorage.setItem("qtools-theme", current);
    document.documentElement.classList.toggle("dark", current === "dark");
  });

  const toggleTheme = () => setTheme((prev) => (prev === "dark" ? "light" : "dark"));

  return (
    <ThemeContext.Provider value={{ theme, setTheme, toggleTheme }}>
      {props.children}
    </ThemeContext.Provider>
  );
};

export function useTheme(): ThemeContextValue {
  const ctx = useContext(ThemeContext);
  if (!ctx) throw new Error("useTheme must be used within ThemeProvider");
  return ctx;
}
