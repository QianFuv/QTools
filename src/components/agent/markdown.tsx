import { openUrl } from "@tauri-apps/plugin-opener";
import {
  Show,
  createEffect,
  createSignal,
  onCleanup,
  splitProps,
  type JSX,
} from "solid-js";
import { Portal } from "solid-js/web";
import remarkGfm from "remark-gfm";
import type { SolidMarkdownComponents } from "solid-markdown";

export const agentRemarkPlugins = [remarkGfm];

export const agentMarkdownClass =
  "prose prose-sm prose-qtools leading-[1.5] prose-headings:mt-0 prose-headings:mb-4 prose-headings:leading-[1.5] prose-p:my-4 prose-p:leading-[1.5] prose-ul:my-4 prose-ol:my-4 prose-li:my-1 prose-li:leading-[1.5] prose-blockquote:my-4 prose-blockquote:leading-[1.5] prose-pre:my-4 prose-pre:rounded-xl prose-pre:px-4 prose-pre:py-3 prose-pre:leading-[1.5] prose-hr:my-6 prose-code:leading-[1.5] prose-code:font-medium prose-table:my-4 prose-table:w-full prose-th:text-left prose-th:font-semibold prose-th:leading-[1.5] prose-td:leading-[1.5] prose-a:no-underline hover:prose-a:text-qtools-500 prose-img:my-4 prose-img:rounded-lg max-w-none break-words dark:prose-invert";

const [pendingExternalUrl, setPendingExternalUrl] = createSignal<string | null>(null);
let pendingExternalUrlResolver: ((confirmed: boolean) => void) | undefined;

function resolveExternalUrl(href: string | undefined): string | null {
  if (!href) return null;
  if (href.startsWith("#")) return null;

  try {
    const url = new URL(href, window.location.href);
    if (["http:", "https:", "mailto:", "tel:"].includes(url.protocol)) {
      return url.toString();
    }
  } catch {
    return null;
  }

  return null;
}

function requestExternalLinkConfirmation(url: string): Promise<boolean> {
  if (pendingExternalUrlResolver) {
    pendingExternalUrlResolver(false);
  }

  setPendingExternalUrl(url);

  return new Promise<boolean>((resolve) => {
    pendingExternalUrlResolver = resolve;
  });
}

function settleExternalLinkConfirmation(confirmed: boolean) {
  pendingExternalUrlResolver?.(confirmed);
  pendingExternalUrlResolver = undefined;
  setPendingExternalUrl(null);
}

function MarkdownLink(
  props: JSX.AnchorHTMLAttributes<HTMLAnchorElement>,
) {
  const [local, rest] = splitProps(props, ["href", "children"]);

  const handleClick: JSX.EventHandler<HTMLAnchorElement, MouseEvent> = async (event) => {
    const url = resolveExternalUrl(local.href);
    if (!url) return;

    event.preventDefault();
    event.stopPropagation();

    const shouldOpen = await requestExternalLinkConfirmation(url);

    if (!shouldOpen) return;

    await openUrl(url);
  };

  return (
    <a
      {...rest}
      href={local.href}
      rel="noreferrer noopener"
      onClick={handleClick}
    >
      {local.children}
    </a>
  );
}

export const agentMarkdownComponents: SolidMarkdownComponents = {
  a: MarkdownLink,
};

export function ExternalLinkConfirmDialog() {
  let confirmButtonRef: HTMLButtonElement | undefined;

  createEffect(() => {
    if (!pendingExternalUrl()) return;

    queueMicrotask(() => confirmButtonRef?.focus());

    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        settleExternalLinkConfirmation(false);
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    onCleanup(() => window.removeEventListener("keydown", handleKeyDown));
  });

  return (
    <Show when={pendingExternalUrl()}>
      {(url) => (
        <Portal>
          <div class="fixed inset-0 z-50 flex items-center justify-center bg-qtools-950/55 p-4 backdrop-blur-sm">
            <div
              role="dialog"
              aria-modal="true"
              aria-labelledby="external-link-dialog-title"
              class="w-full max-w-md rounded-2xl border border-qtools-300 bg-qtools-50 p-6 shadow-2xl dark:border-qtools-700 dark:bg-qtools-900"
            >
              <div class="mb-3 flex items-center gap-3">
                <div class="flex h-10 w-10 items-center justify-center rounded-xl bg-qtools-200 text-qtools-700 dark:bg-qtools-800 dark:text-qtools-200">
                  <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M15 3h6v6" />
                    <path d="M10 14 21 3" />
                    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                  </svg>
                </div>
                <div>
                  <h2
                    id="external-link-dialog-title"
                    class="text-base font-semibold text-qtools-900 dark:text-qtools-50"
                  >
                    Open external link
                  </h2>
                  <p class="text-sm text-qtools-600 dark:text-qtools-300">
                    This link will be opened in your default browser.
                  </p>
                </div>
              </div>

              <div class="mb-5 rounded-xl border border-qtools-200 bg-white px-4 py-3 text-sm text-qtools-700 dark:border-qtools-700 dark:bg-qtools-950 dark:text-qtools-200">
                <p class="break-all font-mono">{url()}</p>
              </div>

              <div class="flex justify-end gap-3">
                <button
                  type="button"
                  onClick={() => settleExternalLinkConfirmation(false)}
                  class="rounded-xl border border-qtools-300 px-4 py-2 text-sm font-medium text-qtools-700 transition-colors hover:bg-qtools-100 dark:border-qtools-600 dark:text-qtools-200 dark:hover:bg-qtools-800"
                >
                  Cancel
                </button>
                <button
                  type="button"
                  ref={confirmButtonRef}
                  onClick={() => settleExternalLinkConfirmation(true)}
                  class="rounded-xl bg-qtools-600 px-4 py-2 text-sm font-medium text-white transition-colors hover:bg-qtools-700 dark:bg-qtools-500 dark:hover:bg-qtools-400 dark:hover:text-qtools-950"
                >
                  Open in browser
                </button>
              </div>
            </div>
          </div>
        </Portal>
      )}
    </Show>
  );
}
