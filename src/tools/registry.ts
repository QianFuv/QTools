import type { ToolDefinition } from "../types/tool";
import homeTool from "./home";
import greeterTool from "./greeter";
import canvasTool from "./canvas";

const tools: ToolDefinition[] = [homeTool, greeterTool, canvasTool];

export default tools;
