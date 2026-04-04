import markdownItAbbr from "markdown-it-abbr";
import markdownItContainer from "markdown-it-container";
import markdownItDeflist from "markdown-it-deflist";
import markdownItFootnote from "markdown-it-footnote";
import markdownItIns from "markdown-it-ins";
import markdownItMark from "markdown-it-mark";
import markdownItSub from "markdown-it-sub";
import markdownItSup from "markdown-it-sup";
import { openUrl } from "@tauri-apps/plugin-opener";
import { fullEmoji } from "@mdit/plugin-emoji";
import { katex } from "@mdit/plugin-katex";
import MarkdownIt from "markdown-it";
import { Show, createEffect, createMemo, createSignal, createUniqueId, onCleanup, type JSX } from "solid-js";
import { Portal } from "solid-js/web";

interface AgentMarkdownProps {
  content: string;
  renderingStrategy?: "memo" | "reconcile";
}

export const agentMarkdownClass =
  "prose prose-sm prose-qtools leading-[1.5] prose-headings:mt-0 prose-headings:mb-4 prose-headings:leading-[1.5] prose-p:my-4 prose-p:leading-[1.5] prose-ul:my-4 prose-ol:my-4 prose-li:my-1 prose-li:leading-[1.5] prose-blockquote:my-4 prose-blockquote:leading-[1.5] prose-pre:my-4 prose-pre:overflow-x-auto prose-pre:rounded-b-xl prose-pre:rounded-t-none prose-pre:px-4 prose-pre:py-3 prose-pre:leading-[1.5] prose-hr:my-6 prose-code:leading-[1.5] prose-code:font-medium prose-table:my-4 prose-table:w-full prose-th:text-left prose-th:font-semibold prose-th:leading-[1.5] prose-td:leading-[1.5] prose-a:no-underline prose-img:my-4 prose-img:rounded-lg max-w-none break-words dark:prose-invert";

const copyButtonResetTimers = new WeakMap<HTMLButtonElement, number>();
const [pendingExternalUrl, setPendingExternalUrl] = createSignal<string | null>(null);
let pendingExternalUrlResolver: ((confirmed: boolean) => void) | undefined;
const containerVariants = [
  { name: "warning", label: "Warning" },
  { name: "info", label: "Info" },
  { name: "tip", label: "Tip" },
  { name: "success", label: "Success" },
  { name: "danger", label: "Danger" },
  { name: "note", label: "Note" },
] as const;

function escapeHtml(value: string): string {
  return value
    .replace(/&/gu, "&amp;")
    .replace(/</gu, "&lt;")
    .replace(/>/gu, "&gt;")
    .replace(/"/gu, "&quot;")
    .replace(/'/gu, "&#39;");
}

function resolveExternalUrl(href: string | undefined): string | null {
  if (!href || href.startsWith("#")) return null;

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

async function copyText(text: string): Promise<void> {
  if (navigator.clipboard?.writeText) {
    await navigator.clipboard.writeText(text);
    return;
  }

  const textarea = document.createElement("textarea");
  textarea.value = text;
  textarea.setAttribute("readonly", "");
  textarea.style.position = "fixed";
  textarea.style.opacity = "0";
  textarea.style.pointerEvents = "none";
  document.body.append(textarea);
  textarea.focus();
  textarea.select();

  const copied = document.execCommand("copy");
  textarea.remove();

  if (!copied) {
    throw new Error("Failed to copy code block content.");
  }
}

function showCopiedState(button: HTMLButtonElement) {
  const existingTimer = copyButtonResetTimers.get(button);
  if (existingTimer) {
    window.clearTimeout(existingTimer);
  }

  const defaultLabel = button.dataset.defaultLabel ?? button.textContent ?? "Copy";
  button.dataset.defaultLabel = defaultLabel;
  button.dataset.state = "copied";
  button.textContent = "Copied";

  const timer = window.setTimeout(() => {
    if (!button.isConnected) return;

    button.dataset.state = "idle";
    button.textContent = button.dataset.defaultLabel ?? "Copy";
  }, 1600);

  copyButtonResetTimers.set(button, timer);
}

function createContainerRenderer(
  name: (typeof containerVariants)[number]["name"],
  label: string,
): MarkdownRenderRule {
  return (tokens, idx) => {
    if (tokens[idx].nesting === 1) {
      const info = tokens[idx].info.trim();
      const title = info.slice(name.length).trim() || label;

      return [
        `<div class="agent-markdown-container agent-markdown-container-${name}">`,
        `<div class="agent-markdown-container-title">${escapeHtml(title)}</div>`,
        '<div class="agent-markdown-container-body">',
      ].join("\n");
    }

    return "</div>\n</div>\n";
  };
}

const markdownRenderer = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: true,
})
  .use(fullEmoji)
  .use(katex, {
    delimiters: "all",
    mathFence: true,
    throwOnError: false,
    strict: false,
    output: "htmlAndMathml",
  })
  .use(markdownItSub)
  .use(markdownItSup)
  .use(markdownItIns)
  .use(markdownItMark)
  .use(markdownItFootnote)
  .use(markdownItDeflist)
  .use(markdownItAbbr);

const defaultFenceRenderer = markdownRenderer.renderer.rules.fence;
const defaultInlineCodeRenderer = markdownRenderer.renderer.rules.code_inline;
const defaultLinkOpenRenderer = markdownRenderer.renderer.rules.link_open;
type MarkdownRenderRule = NonNullable<typeof defaultFenceRenderer>;

const renderFence: MarkdownRenderRule = (tokens, idx, options, env, self) => {
  const token = tokens[idx];
  const info = token.info.trim();
  const language = info.split(/\s+/u)[0] ?? "";
  const renderedFence = defaultFenceRenderer
    ? defaultFenceRenderer(tokens, idx, options, env, self)
    : `<pre><code>${escapeHtml(token.content)}</code></pre>\n`;
  const languageLabel = language || "text";

  return [
    '<div class="agent-markdown-code-block">',
    '<div class="agent-markdown-code-toolbar">',
    `<span class="agent-markdown-code-language">${escapeHtml(languageLabel)}</span>`,
    '<button type="button" class="agent-markdown-copy-button" data-copy-code data-default-label="Copy" data-state="idle">Copy</button>',
    "</div>",
    renderedFence,
    "</div>",
  ].join("");
};

const renderInlineCode: MarkdownRenderRule = (tokens, idx, options, env, self) => {
  tokens[idx].attrJoin("class", "agent-markdown-inline-code");

  return defaultInlineCodeRenderer
    ? defaultInlineCodeRenderer(tokens, idx, options, env, self)
    : self.renderToken(tokens, idx, options);
};

const renderLinkOpen: MarkdownRenderRule = (tokens, idx, options, env, self) => {
  tokens[idx].attrJoin("class", "agent-markdown-link");
  tokens[idx].attrJoin("rel", "noreferrer noopener");

  return defaultLinkOpenRenderer
    ? defaultLinkOpenRenderer(tokens, idx, options, env, self)
    : self.renderToken(tokens, idx, options);
};

markdownRenderer.renderer.rules.fence = renderFence;
markdownRenderer.renderer.rules.code_inline = renderInlineCode;
markdownRenderer.renderer.rules.link_open = renderLinkOpen;

for (const variant of containerVariants) {
  markdownRenderer.use(markdownItContainer, variant.name, {
    render: createContainerRenderer(variant.name, variant.label),
  });
}

export function AgentMarkdown(props: AgentMarkdownProps) {
  const docId = createUniqueId();
  const renderedHtml = createMemo(() => markdownRenderer.render(props.content ?? "", { docId }));

  const handleClick: JSX.EventHandler<HTMLDivElement, MouseEvent> = async (event) => {
    const target = event.target;
    if (!(target instanceof Element)) return;

    const copyButton = target.closest<HTMLButtonElement>("[data-copy-code]");
    if (copyButton && event.currentTarget.contains(copyButton)) {
      event.preventDefault();
      event.stopPropagation();

      const codeElement = copyButton.closest(".agent-markdown-code-block")?.querySelector("code");
      const code = codeElement?.textContent ?? "";
      if (!code) return;

      await copyText(code);
      showCopiedState(copyButton);
      return;
    }

    const anchor = target.closest<HTMLAnchorElement>("a[href]");
    if (!anchor || !event.currentTarget.contains(anchor)) return;

    const url = resolveExternalUrl(anchor.getAttribute("href") ?? undefined);
    if (!url) return;

    event.preventDefault();
    event.stopPropagation();

    const shouldOpen = await requestExternalLinkConfirmation(url);
    if (!shouldOpen) return;

    await openUrl(url);
  };

  return <div class="agent-markdown-content" onClick={handleClick} innerHTML={renderedHtml()} />;
}

export function ExternalLinkConfirmDialog() {
  let confirmButtonRef: HTMLButtonElement | undefined;

  createEffect(() => {
    const currentUrl = pendingExternalUrl();
    if (!currentUrl) return;

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
          <div
            class="fixed inset-0 z-50 flex items-center justify-center bg-qtools-950/55 p-4 backdrop-blur-sm"
            onClick={(event) => {
              if (event.target === event.currentTarget) {
                settleExternalLinkConfirmation(false);
              }
            }}
          >
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
