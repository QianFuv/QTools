import { createSignal } from "solid-js";

interface ChatInputProps {
  onSend: (content: string) => void;
  disabled: boolean;
}

export default function ChatInput(props: ChatInputProps) {
  const [input, setInput] = createSignal("");

  const send = () => {
    const content = input().trim();
    if (!content || props.disabled) return;
    props.onSend(content);
    setInput("");
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      send();
    }
  };

  return (
    <div class="border-t border-qtools-200 bg-qtools-50 p-4 dark:border-qtools-700 dark:bg-qtools-950">
      <div class="flex flex-col rounded-xl border border-qtools-300 bg-white dark:border-qtools-600 dark:bg-qtools-900 focus-within:border-qtools-500 dark:focus-within:border-qtools-400 transition-colors">
        <textarea
          value={input()}
          onInput={(e) => setInput(e.currentTarget.value)}
          onKeyDown={handleKeyDown}
          placeholder="Type a message..."
          rows={3}
          class="w-full resize-none rounded-t-xl bg-transparent px-4 pt-3 pb-2 text-sm text-qtools-900 placeholder-qtools-400 outline-none dark:text-qtools-100 dark:placeholder-qtools-500"
        />
        <div class="flex items-center justify-end px-3 pb-2">
          <button
            type="button"
            onClick={send}
            disabled={props.disabled || !input().trim()}
            class="rounded-lg bg-qtools-200 px-4 py-1 text-sm font-medium text-qtools-600 transition-colors hover:bg-qtools-300 disabled:cursor-not-allowed disabled:opacity-40 dark:bg-qtools-800 dark:text-qtools-300 dark:hover:bg-qtools-700"
          >
            Send
          </button>
        </div>
      </div>
    </div>
  );
}
