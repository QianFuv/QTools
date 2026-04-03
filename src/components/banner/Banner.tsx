import DarkModeToggle from "./DarkModeToggle";

export default function Banner() {
  return (
    <header class="flex h-11 items-center justify-between border-b border-qtools-700 bg-qtools-900 px-4 dark:border-qtools-800 dark:bg-qtools-950">
      <span class="text-sm font-bold tracking-wide text-qtools-50">
        QTools
      </span>
      <div class="flex items-center gap-1">
        <DarkModeToggle />
      </div>
    </header>
  );
}
