import { createSignal, onMount, Show } from "solid-js";
import { useAgent } from "../../contexts/AgentContext";
import type { AgentSettings as AgentSettingsType, ApiFormat } from "../../types/agent";

const inputClass =
  "w-full rounded-lg border border-qtools-300 bg-white px-3 py-2 text-sm text-qtools-900 outline-none focus:border-qtools-500 dark:border-qtools-600 dark:bg-qtools-900 dark:text-qtools-100 dark:focus:border-qtools-400";

const labelClass =
  "mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300";

export default function AgentSettings() {
  const { settings, saveSettings } = useAgent();

  const [baseUrl, setBaseUrl] = createSignal("");
  const [apiKey, setApiKey] = createSignal("");
  const [model, setModel] = createSignal("");
  const [apiFormat, setApiFormat] = createSignal<ApiFormat>("openai_chat");
  const [systemPrompt, setSystemPrompt] = createSignal("");
  const [saving, setSaving] = createSignal(false);
  const [saved, setSaved] = createSignal(false);

  onMount(() => {
    const s = settings();
    setBaseUrl(s.base_url);
    setApiKey(s.api_key);
    setModel(s.model);
    setApiFormat(s.api_format);
    setSystemPrompt(s.system_prompt);
  });

  const handleSave = async () => {
    setSaving(true);
    setSaved(false);
    try {
      const updated: AgentSettingsType = {
        base_url: baseUrl(),
        api_key: apiKey(),
        model: model(),
        api_format: apiFormat(),
        system_prompt: systemPrompt(),
      };
      await saveSettings(updated);
      setSaved(true);
      setTimeout(() => setSaved(false), 2000);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="space-y-4">
      <div>
        <label class={labelClass}>API Format</label>
        <select
          value={apiFormat()}
          onChange={(e) => setApiFormat(e.currentTarget.value as ApiFormat)}
          class={inputClass}
        >
          <option value="openai_chat">OpenAI Chat Completions</option>
          <option value="openai_responses">OpenAI Responses</option>
          <option value="anthropic">Anthropic</option>
        </select>
      </div>

      <div>
        <label class={labelClass}>Base URL</label>
        <input
          type="text"
          value={baseUrl()}
          onInput={(e) => setBaseUrl(e.currentTarget.value)}
          placeholder="https://api.openai.com/v1"
          class={inputClass}
        />
      </div>

      <div>
        <label class={labelClass}>API Key</label>
        <input
          type="password"
          value={apiKey()}
          onInput={(e) => setApiKey(e.currentTarget.value)}
          placeholder="sk-..."
          class={inputClass}
        />
      </div>

      <div>
        <label class={labelClass}>Model</label>
        <input
          type="text"
          value={model()}
          onInput={(e) => setModel(e.currentTarget.value)}
          placeholder="gpt-4o"
          class={inputClass}
        />
      </div>

      <div>
        <label class={labelClass}>System Prompt</label>
        <textarea
          value={systemPrompt()}
          onInput={(e) => setSystemPrompt(e.currentTarget.value)}
          placeholder="You are a helpful assistant."
          rows={4}
          class={`${inputClass} resize-none`}
        />
      </div>

      <div class="flex items-center gap-3 pt-2">
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
