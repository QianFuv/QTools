declare module "markdown-it-abbr" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}

declare module "markdown-it-container" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginWithParams = (md: MarkdownIt, ...params: unknown[]) => void;
  const plugin: MarkdownItPluginWithParams;
  export default plugin;
}

declare module "markdown-it-deflist" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}

declare module "markdown-it-footnote" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}

declare module "markdown-it-ins" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}

declare module "markdown-it-mark" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}

declare module "markdown-it-sub" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}

declare module "markdown-it-sup" {
  import type MarkdownIt from "markdown-it";

  type MarkdownItPluginSimple = (md: MarkdownIt) => void;
  const plugin: MarkdownItPluginSimple;
  export default plugin;
}
