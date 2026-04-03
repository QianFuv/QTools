import type { Component, JSX } from "solid-js";

export interface ToolDefinition {
  id: string;
  name: string;
  icon: () => JSX.Element;
  component: Component;
}
