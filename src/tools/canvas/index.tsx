import type { ToolDefinition } from "../../types/tool";
import CanvasTool from "./CanvasTool";
import SettingsPanel from "./SettingsPanel";

const canvasTool: ToolDefinition = {
  id: "canvas",
  name: "Canvas",
  icon: () => (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 20 20"
      fill="currentColor"
      class="size-5"
    >
      <path d="M7.171 4.146l1.947 2.466a3.514 3.514 0 0 1 1.764 0l1.947-2.466a6.52 6.52 0 0 0-5.658 0Zm8.683 3.025l-2.466 1.947c.15.578.15 1.186 0 1.764l2.466 1.947a6.52 6.52 0 0 0 0-5.658Zm-3.025 8.683l-1.947-2.466c-.578.15-1.186.15-1.764 0l-1.947 2.466a6.52 6.52 0 0 0 5.658 0ZM4.146 12.83l2.466-1.947a3.514 3.514 0 0 1 0-1.764L4.146 7.171a6.52 6.52 0 0 0 0 5.658ZM10 8a2 2 0 1 0 0 4 2 2 0 0 0 0-4Z" />
      <path
        fill-rule="evenodd"
        d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm0-1.5a6.5 6.5 0 1 0 0-13 6.5 6.5 0 0 0 0 13Z"
        clip-rule="evenodd"
      />
    </svg>
  ),
  component: CanvasTool,
  settingsComponent: SettingsPanel,
};

export default canvasTool;
