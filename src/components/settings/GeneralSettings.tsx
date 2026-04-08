import { createSignal, onMount, Show } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { open, ask, confirm } from "@tauri-apps/plugin-dialog";

const inputClass =
  "w-full rounded-lg border border-qtools-300 bg-white px-3 py-2 text-sm text-qtools-900 outline-none focus:border-qtools-500 dark:border-qtools-600 dark:bg-qtools-900 dark:text-qtools-100 dark:focus:border-qtools-400";

const labelClass =
  "mb-1 block text-sm font-medium text-qtools-700 dark:text-qtools-300";

export default function GeneralSettings() {
  const [dataDir, setDataDir] = createSignal("");
  const [newDataDir, setNewDataDir] = createSignal("");
  const [dataDirChanged, setDataDirChanged] = createSignal(false);
  const [saving, setSaving] = createSignal(false);
  const [migrating, setMigrating] = createSignal(false);
  const [migrateMsg, setMigrateMsg] = createSignal("");

  onMount(async () => {
    try {
      const dir = await invoke<string>("get_data_dir");
      setDataDir(dir);
      setNewDataDir(dir);
    } catch {
      /* use empty */
    }
  });

  const handleBrowse = async () => {
    const selected = await open({
      directory: true,
      title: "Select Data Directory",
      defaultPath: newDataDir() || undefined,
    });
    if (selected) {
      setNewDataDir(selected);
      setDataDirChanged(selected !== dataDir());
    }
  };

  const handleMigrate = async () => {
    const src = dataDir();
    const dst = newDataDir();
    if (!src || !dst || src === dst) return;

    const proceed = await ask(
      `Migrate data from:\n${src}\n\nTo:\n${dst}\n\nContinue?`,
      { title: "Migrate Data", kind: "info" },
    );
    if (!proceed) return;

    const deleteSource = await confirm(
      "Delete original data after migration?",
      { title: "Delete Original", kind: "warning" },
    );

    setMigrating(true);
    setMigrateMsg("");
    try {
      await invoke("migrate_data", {
        source: src,
        dest: dst,
        deleteSource,
      });
      setMigrateMsg("Migration complete. Restart to use the new location.");
    } catch (e) {
      setMigrateMsg(`Migration failed: ${e}`);
    } finally {
      setMigrating(false);
    }
  };

  const handleSave = async () => {
    if (!dataDirChanged()) return;
    setSaving(true);
    try {
      await invoke("set_data_dir", { path: newDataDir() });
      setDataDir(newDataDir());
      setDataDirChanged(false);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div class="space-y-5">
      <div>
        <label class={labelClass}>Data Directory</label>
        <div class="flex gap-2">
          <input
            type="text"
            value={newDataDir()}
            onInput={(e) => {
              setNewDataDir(e.currentTarget.value);
              setDataDirChanged(e.currentTarget.value !== dataDir());
            }}
            class={`${inputClass} flex-1`}
            placeholder="Path to data directory"
          />
          <button
            onClick={handleBrowse}
            class="shrink-0 rounded-lg border border-qtools-300 px-3 py-2 text-sm text-qtools-700 transition-colors hover:bg-qtools-100 dark:border-qtools-600 dark:text-qtools-300 dark:hover:bg-qtools-800"
          >
            Browse
          </button>
        </div>
        <Show when={dataDirChanged()}>
          <p class="mt-1 text-xs text-amber-600 dark:text-amber-400">
            Restart required after changing the data directory.
          </p>
        </Show>
      </div>

      <Show when={dataDirChanged() && dataDir()}>
        <div class="flex items-center gap-3">
          <button
            onClick={handleMigrate}
            disabled={migrating()}
            class="rounded-lg bg-qtools-600 px-3 py-1.5 text-sm font-medium text-white transition-colors hover:bg-qtools-700 disabled:opacity-50 dark:bg-qtools-500 dark:text-qtools-950 dark:hover:bg-qtools-400"
          >
            {migrating() ? "Migrating..." : "Migrate Data"}
          </button>
          <Show when={migrateMsg()}>
            <span class="text-xs text-qtools-600 dark:text-qtools-400">
              {migrateMsg()}
            </span>
          </Show>
        </div>
      </Show>

      <Show when={dataDirChanged()}>
        <div class="pt-2">
          <button
            onClick={handleSave}
            disabled={saving()}
            class="rounded-lg bg-qtools-500 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-qtools-600 disabled:opacity-50 dark:bg-qtools-400 dark:text-qtools-950 dark:hover:bg-qtools-300"
          >
            {saving() ? "Saving..." : "Save"}
          </button>
        </div>
      </Show>
    </div>
  );
}
