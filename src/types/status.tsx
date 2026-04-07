export type StatusSeverity = "info" | "success" | "warning" | "error";

export interface StatusMessage {
  text: string;
  severity: StatusSeverity;
}
