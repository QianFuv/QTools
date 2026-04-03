import DarkModeToggle from "./DarkModeToggle";

export default function Banner() {
  return (
    <header class="flex h-11 items-center justify-between border-b border-maroon-700 bg-maroon-900 px-4 dark:border-maroon-800 dark:bg-maroon-950">
      <span class="text-sm font-bold tracking-wide text-maroon-50">
        QTools
      </span>
      <div class="flex items-center gap-1">
        <DarkModeToggle />
      </div>
    </header>
  );
}
