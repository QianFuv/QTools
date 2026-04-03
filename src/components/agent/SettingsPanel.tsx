import { createSignal, onMount } from "solid-js";
import { useAgent } from "../../contexts/AgentContext";
import type { AgentSettings, ApiFormat } from "../../types/agent";

interface SettingsPanelProps {
  onClose: () => void;
}

export default function SettingsPanel(props: SettingsPanelProps) {
  const { settings, saveSettings } = useAgent();

  const [baseUrl, setBaseUrl] = createSignal("");
  const [apiKey, setApiKey] = createSignal("");
  const [model, setModel] = createSignal("");
  const [apiFormat, setApiFormat] = createSignal<ApiFormat>("openai_chat");
  const [systemPrompt, setSystemPrompt] = createSignal("");
  const [saving, setSaving] = createSignal(false);

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
    try {
      const updated: AgentSettings = {
        base_url: baseUrl(),
        api_key: apiKey(),
        model: model(),
        api_format: apiFormat(),
        system_prompt: systemPrompt(),
      };
      await saveSettings(updated);
      props.onClose();
    } finally {
      setSaving(false);
    }
  };

  const inputClass =
    "w-full rounded-lg border border-qtools-300 bg-white px-3 py-2 text-sm text-qtools-900 outline-none focus:border-qtools-500 dark:border-qtools-600 dark:bg-qtools-900 dark:text-qtools-100 dark:focus:border-qtools-400";

  return (
    <div class="flex flex-1 flex-col bg-qtools-50 dark:bg-qtools-950">
      <div class="flex items-center justify-between border-b border-qtools-200 px-6 py-4 dark:border-qtools-700">
        <h2 class="text-lg font-semibold text-qtools-900 dark:text-qtools-50">
          Agent Settings
        </h2>
        <button
          onClick={props.onClose}
          class="rounded-lg p-1.5 text-qtools-500 transition-colors hover:bg-qtools-200 dark:text-qtools-400 dark:hover:bg-qtools-800"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <line x1="18" y1="6" x2="6" y2="18" />
            <line x1="6" y1="6" x2="18" y2="18" />
          </svg>
        </button>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        <div class="mx-auto max-w-lg space-y-5">
          <div>
            <label class="mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300">
              API Format
            </label>
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
            <label class="mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300">
              Base URL
            </label>
            <input
              type="text"
              value={baseUrl()}
              onInput={(e) => setBaseUrl(e.currentTarget.value)}
              placeholder="https://api.openai.com/v1"
              class={inputClass}
            />
          </div>

          <div>
            <label class="mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300">
              API Key
            </label>
            <input
              type="password"
              value={apiKey()}
              onInput={(e) => setApiKey(e.currentTarget.value)}
              placeholder="sk-..."
              class={inputClass}
            />
          </div>

          <div>
            <label class="mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300">
              Model
            </label>
            <input
              type="text"
              value={model()}
              onInput={(e) => setModel(e.currentTarget.value)}
              placeholder="gpt-4o"
              class={inputClass}
            />
          </div>

          <div>
            <label class="mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300">
              System Prompt
            </label>
            <textarea
              value={systemPrompt()}
              onInput={(e) => setSystemPrompt(e.currentTarget.value)}
              placeholder="You are a helpful assistant."
              rows={4}
              class={`${inputClass} resize-none`}
            />
          </div>

          <div class="flex gap-3 pt-2">
            <button
              onClick={handleSave}
              disabled={saving()}
              class="rounded-lg bg-qtools-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-qtools-600 disabled:opacity-50"
            >
              {saving() ? "Saving..." : "Save"}
            </button>
            <button
              onClick={props.onClose}
              class="rounded-lg border border-qtools-300 px-4 py-2 text-sm font-medium text-qtools-700 transition-colors hover:bg-qtools-100 dark:border-qtools-600 dark:text-qtools-300 dark:hover:bg-qtools-800"
            >
              Cancel
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
