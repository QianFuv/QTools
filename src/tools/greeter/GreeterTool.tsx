import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { useStatusBar } from "../../contexts/StatusBarContext";

export default function GreeterTool() {
  const [name, setName] = createSignal("");
  const [greetMsg, setGreetMsg] = createSignal("");
  const { setStatus } = useStatusBar();

  async function greet(e: SubmitEvent) {
    e.preventDefault();
    setStatus("Sending greet request...", "info");
    try {
      const msg = await invoke<string>("greet", { name: name() });
      setGreetMsg(msg);
      setStatus("Greet successful", "success");
    } catch (err) {
      setStatus(`Greet failed: ${err}`, "error");
    }
  }

  return (
    <div class="flex flex-1 flex-col items-center justify-center gap-6 p-8">
      <h2 class="text-2xl font-bold text-maroon-800 dark:text-maroon-100">
        Greeter
      </h2>
      <form class="flex gap-2" onSubmit={greet}>
        <input
          class="rounded-lg border border-maroon-300 bg-white px-3 py-2 text-maroon-900 outline-none focus:border-maroon-500 dark:border-maroon-600 dark:bg-maroon-800 dark:text-maroon-50 dark:focus:border-maroon-400"
          value={name()}
          onInput={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <button
          type="submit"
          class="rounded-lg bg-maroon-500 px-4 py-2 font-medium text-white transition-colors hover:bg-maroon-600 dark:bg-maroon-400 dark:text-maroon-950 dark:hover:bg-maroon-300"
        >
          Greet
        </button>
      </form>
      <p class="text-maroon-700 dark:text-maroon-200">{greetMsg()}</p>
    </div>
  );
}
