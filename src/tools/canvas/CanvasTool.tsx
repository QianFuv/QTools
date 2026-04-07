import { createSignal, onMount, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import type { CanvasData, CanvasSettings } from "./types";
import CourseView from "./CourseView";
import TimelineView from "./TimelineView";
import SettingsPanel from "./SettingsPanel";

type Tab = "courses" | "timeline" | "settings";

export default function CanvasTool() {
  const [tab, setTab] = createSignal<Tab>("timeline");
  const [data, setData] = createSignal<CanvasData | null>(null);
  const [settings, setSettings] = createSignal<CanvasSettings | null>(null);
  const [loading, setLoading] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [hideCompleted, setHideCompleted] = createSignal(false);

  const loadData = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<CanvasData>("fetch_canvas_data");
      setData(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const refreshData = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<CanvasData>("refresh_canvas_data");
      setData(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  const loadSettings = async () => {
    try {
      const s = await invoke<CanvasSettings>("get_canvas_settings");
      setSettings(s);
    } catch {
      /* use defaults */
    }
  };

  const toggleCompletion = async (assignmentId: number, completed: boolean) => {
    try {
      await invoke("set_task_completion", {
        assignmentId,
        completed,
      });
      setData((prev) => {
        if (!prev) return prev;
        return {
          ...prev,
          assignments: prev.assignments.map((a) =>
            a.id === assignmentId ? { ...a, manually_completed: completed } : a,
          ),
        };
      });
    } catch (e) {
      console.error("Failed to toggle completion:", e);
    }
  };

  onMount(async () => {
    await loadSettings();
    const s = settings();
    if (s && s.api_token) {
      await loadData();
    }
  });

  const tabs: { id: Tab; label: string }[] = [
    { id: "timeline", label: "Timeline" },
    { id: "courses", label: "Courses" },
    { id: "settings", label: "Settings" },
  ];

  const formatCacheTime = () => {
    const d = data();
    if (!d) return "";
    const date = new Date(d.fetched_at);
    return date.toLocaleTimeString();
  };

  return (
    <div class="flex min-h-0 flex-1 flex-col overflow-hidden">
      <div class="flex items-center justify-between border-b border-qtools-200 bg-qtools-50 px-4 py-2 dark:border-qtools-700 dark:bg-qtools-900">
        <div class="flex gap-1">
          {tabs.map((t) => (
            <button
              onClick={() => setTab(t.id)}
              class={`rounded-lg px-3 py-1.5 text-sm font-medium transition-colors ${
                tab() === t.id
                  ? "bg-qtools-500 text-white dark:bg-qtools-400 dark:text-qtools-950"
                  : "text-qtools-600 hover:bg-qtools-200 dark:text-qtools-300 dark:hover:bg-qtools-800"
              }`}
            >
              {t.label}
            </button>
          ))}
        </div>

        <div class="flex items-center gap-3">
          <Show when={tab() !== "settings"}>
            <button
              onClick={() => setHideCompleted(!hideCompleted())}
              class={`rounded-lg px-3 py-1.5 text-sm font-medium transition-colors ${
                hideCompleted()
                  ? "bg-qtools-500 text-white dark:bg-qtools-400 dark:text-qtools-950"
                  : "text-qtools-600 hover:bg-qtools-200 dark:text-qtools-300 dark:hover:bg-qtools-800"
              }`}
            >
              Hide Completed
            </button>
          </Show>
          <Show when={data()}>
            <span class="text-xs text-qtools-500 dark:text-qtools-400">
              Cached at {formatCacheTime()}
            </span>
          </Show>
          <button
            onClick={refreshData}
            disabled={loading()}
            class="rounded-lg bg-qtools-500 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-qtools-600 disabled:opacity-50 dark:bg-qtools-400 dark:text-qtools-950 dark:hover:bg-qtools-300"
          >
            {loading() ? "Loading..." : "Refresh"}
          </button>
        </div>
      </div>

      <Show when={error()}>
        <div class="mx-4 mt-3 rounded-lg border border-red-300 bg-red-50 px-4 py-2 text-sm text-red-700 dark:border-red-700 dark:bg-red-950 dark:text-red-300">
          {error()}
        </div>
      </Show>

      <div class="flex-1 overflow-y-auto">
        <Show when={tab() === "timeline"}>
          <TimelineView
            data={data()}
            loading={loading()}
            settings={settings()}
            hideCompleted={hideCompleted()}
            onToggleCompletion={toggleCompletion}
          />
        </Show>
        <Show when={tab() === "courses"}>
          <CourseView
            data={data()}
            loading={loading()}
            settings={settings()}
            hideCompleted={hideCompleted()}
            onToggleCompletion={toggleCompletion}
          />
        </Show>
        <Show when={tab() === "settings"}>
          <SettingsPanel
            settings={settings()}
            courses={data()?.courses ?? []}
            onSave={async (s) => {
              await invoke("save_canvas_settings", { settings: s });
              setSettings(s);
              await loadData();
            }}
          />
        </Show>
      </div>
    </div>
  );
}
