import { createSignal, onMount, For, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import type { CanvasCourse, CanvasData, CanvasSettings } from "./types";

const inputClass =
  "w-full rounded-lg border border-qtools-300 bg-white px-3 py-2 text-sm text-qtools-900 outline-none transition-colors focus:border-qtools-500 dark:border-qtools-600 dark:bg-qtools-900 dark:text-qtools-100 dark:focus:border-qtools-400";

const labelClass =
  "block text-sm font-medium text-qtools-700 dark:text-qtools-300";

export default function SettingsPanel() {
  const [apiUrl, setApiUrl] = createSignal("https://canvas.uts.edu.au");
  const [apiToken, setApiToken] = createSignal("");
  const [cacheTtl, setCacheTtl] = createSignal(30);
  const [ignoredIds, setIgnoredIds] = createSignal<Set<number>>(new Set());
  const [courses, setCourses] = createSignal<CanvasCourse[]>([]);
  const [showToken, setShowToken] = createSignal(false);
  const [saving, setSaving] = createSignal(false);
  const [saved, setSaved] = createSignal(false);

  onMount(async () => {
    try {
      const s = await invoke<CanvasSettings>("get_canvas_settings");
      setApiUrl(s.api_url || "https://canvas.uts.edu.au");
      setApiToken(s.api_token);
      setCacheTtl(s.cache_ttl_minutes);
      setIgnoredIds(new Set(s.ignored_course_ids));
    } catch {
      /* defaults */
    }

    try {
      const data = await invoke<CanvasData>("fetch_canvas_data");
      setCourses(data.courses);
    } catch {
      /* no courses available */
    }
  });

  const toggleIgnore = (courseId: number) => {
    setIgnoredIds((prev) => {
      const next = new Set(prev);
      if (next.has(courseId)) {
        next.delete(courseId);
      } else {
        next.add(courseId);
      }
      return next;
    });
  };

  const handleSave = async () => {
    setSaving(true);
    setSaved(false);
    try {
      const settings: CanvasSettings = {
        api_url: apiUrl().replace(/\/+$/, ""),
        api_token: apiToken(),
        cache_ttl_minutes: cacheTtl(),
        ignored_course_ids: [...ignoredIds()],
      };
      await invoke("save_canvas_settings", { settings });
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="flex flex-col gap-4">
      <div>
        <label class={labelClass}>Canvas URL</label>
        <input
          type="text"
          value={apiUrl()}
          onInput={(e) => setApiUrl(e.currentTarget.value)}
          placeholder="https://canvas.uts.edu.au"
          class={`mt-1 ${inputClass}`}
        />
      </div>

      <div>
        <label class={labelClass}>API Token</label>
        <div class="relative mt-1">
          <input
            type={showToken() ? "text" : "password"}
            value={apiToken()}
            onInput={(e) => setApiToken(e.currentTarget.value)}
            placeholder="Enter your Canvas API token"
            class={`${inputClass} pr-10`}
          />
          <button
            type="button"
            onClick={() => setShowToken(!showToken())}
            class="absolute right-2 top-1/2 -translate-y-1/2 rounded p-1 text-qtools-400 hover:text-qtools-600 dark:text-qtools-500 dark:hover:text-qtools-300"
          >
            <Show
              when={showToken()}
              fallback={
                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-4">
                  <path d="M10 12.5a2.5 2.5 0 1 0 0-5 2.5 2.5 0 0 0 0 5Z" />
                  <path
                    fill-rule="evenodd"
                    d="M.664 10.59a1.651 1.651 0 0 1 0-1.186A10.004 10.004 0 0 1 10 3c4.257 0 7.893 2.66 9.336 6.41.147.381.146.804 0 1.186A10.004 10.004 0 0 1 10 17c-4.257 0-7.893-2.66-9.336-6.41ZM14 10a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
                    clip-rule="evenodd"
                  />
                </svg>
              }
            >
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="size-4">
                <path
                  fill-rule="evenodd"
                  d="M3.28 2.22a.75.75 0 0 0-1.06 1.06l14.5 14.5a.75.75 0 1 0 1.06-1.06l-1.745-1.745a10.029 10.029 0 0 0 3.3-4.38 1.651 1.651 0 0 0 0-1.185A10.004 10.004 0 0 0 9.999 3a9.956 9.956 0 0 0-4.744 1.194L3.28 2.22ZM7.752 6.69l1.092 1.092a2.5 2.5 0 0 1 3.374 3.373l1.092 1.092a4 4 0 0 0-5.558-5.558Z"
                  clip-rule="evenodd"
                />
                <path d="M10.748 13.93l2.523 2.523a9.987 9.987 0 0 1-3.27.547c-4.258 0-7.894-2.66-9.337-6.41a1.651 1.651 0 0 1 0-1.186A10.007 10.007 0 0 1 2.839 6.02L6.07 9.252a4 4 0 0 0 4.678 4.678Z" />
              </svg>
            </Show>
          </button>
        </div>
        <p class="mt-1 text-xs text-qtools-400 dark:text-qtools-500">
          Generate a token in Canvas: Account &gt; Settings &gt; New Access Token
        </p>
      </div>

      <div>
        <label class={labelClass}>Cache Duration (minutes)</label>
        <input
          type="number"
          min="1"
          max="1440"
          value={cacheTtl()}
          onInput={(e) => setCacheTtl(parseInt(e.currentTarget.value) || 30)}
          class={`mt-1 w-32 ${inputClass}`}
        />
      </div>

      <Show when={courses().length > 0}>
        <div>
          <label class={labelClass}>Courses</label>
          <p class="mb-2 text-xs text-qtools-400 dark:text-qtools-500">
            Uncheck courses to hide them from the views.
          </p>
          <div class="mt-1 flex flex-col gap-1 rounded-lg border border-qtools-200 bg-qtools-100 p-2 dark:border-qtools-700 dark:bg-qtools-900">
            <For each={courses()}>
              {(course) => (
                <label class="flex cursor-pointer items-center gap-2 rounded px-2 py-1.5 text-sm transition-colors hover:bg-qtools-100 dark:hover:bg-qtools-800">
                  <input
                    type="checkbox"
                    checked={!ignoredIds().has(course.id)}
                    onChange={() => toggleIgnore(course.id)}
                    class="accent-qtools-500"
                  />
                  <span class="text-qtools-800 dark:text-qtools-100">{course.name}</span>
                  <span class="text-xs text-qtools-400 dark:text-qtools-500">
                    {course.course_code}
                  </span>
                </label>
              )}
            </For>
          </div>
        </div>
      </Show>

      <div class="flex items-center gap-3">
        <button
          onClick={handleSave}
          disabled={saving()}
          class="rounded-lg bg-qtools-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-qtools-600 disabled:opacity-50 dark:bg-qtools-400 dark:text-qtools-950 dark:hover:bg-qtools-300"
        >
          {saving() ? "Saving..." : "Save"}
        </button>
        <Show when={saved()}>
          <span class="text-sm text-emerald-600 dark:text-emerald-400">Saved!</span>
        </Show>
      </div>
    </div>
  );
}
