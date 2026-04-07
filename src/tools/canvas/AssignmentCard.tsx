import { createMemo, Show } from "solid-js";
import type { CanvasAssignment } from "./types";

interface AssignmentCardProps {
  assignment: CanvasAssignment;
  showCourse: boolean;
  onToggleCompletion: (id: number, completed: boolean) => void;
}

export default function AssignmentCard(props: AssignmentCardProps) {
  const urgency = createMemo(() => {
    const a = props.assignment;
    if (a.manually_completed || a.has_submitted_submissions) return "done";
    if (!a.due_at) return "none";

    const now = Date.now();
    const due = new Date(a.due_at).getTime();
    const diff = due - now;

    if (diff < 0) return "overdue";
    if (diff < 3 * 86_400_000) return "soon";
    return "normal";
  });

  const urgencyBorder = () => {
    switch (urgency()) {
      case "overdue":
        return "border-l-red-500";
      case "soon":
        return "border-l-amber-500";
      case "done":
        return "border-l-emerald-500";
      default:
        return "border-l-qtools-300 dark:border-l-qtools-600";
    }
  };

  const formatDue = () => {
    const due = props.assignment.due_at;
    if (!due) return "No due date";
    const d = new Date(due);
    const now = new Date();
    const diff = d.getTime() - now.getTime();
    const days = Math.ceil(diff / 86_400_000);

    const absolute = d.toLocaleDateString(undefined, {
      month: "short",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
    });

    if (days < 0) return `${absolute} (${Math.abs(days)}d overdue)`;
    if (days === 0) return `${absolute} (today)`;
    if (days === 1) return `${absolute} (tomorrow)`;
    if (days <= 7) return `${absolute} (in ${days}d)`;
    return absolute;
  };

  const isDone = () =>
    props.assignment.manually_completed || props.assignment.has_submitted_submissions;

  return (
    <div
      class={`flex items-center gap-3 rounded-lg border border-l-4 bg-white px-4 py-3 transition-colors dark:bg-qtools-900 ${urgencyBorder()} ${
        isDone() ? "opacity-60" : ""
      } border-qtools-200 dark:border-qtools-700`}
    >
      <button
        onClick={() => {
          if (!props.assignment.has_submitted_submissions)
            props.onToggleCompletion(props.assignment.id, !props.assignment.manually_completed);
        }}
        disabled={props.assignment.has_submitted_submissions}
        class={`flex size-5 shrink-0 items-center justify-center rounded border transition-colors ${
          isDone()
            ? "border-emerald-500 bg-emerald-500 text-white"
            : "border-qtools-300 hover:border-qtools-500 dark:border-qtools-600 dark:hover:border-qtools-400"
        } ${props.assignment.has_submitted_submissions ? "cursor-default" : ""}`}
        title={
          props.assignment.has_submitted_submissions
            ? "Submitted on Canvas"
            : isDone()
              ? "Mark as incomplete"
              : "Mark as complete"
        }
      >
        <Show when={isDone()}>
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-3">
            <path
              fill-rule="evenodd"
              d="M12.416 3.376a.75.75 0 0 1 .208 1.04l-5 7.5a.75.75 0 0 1-1.154.114l-3-3a.75.75 0 0 1 1.06-1.06l2.353 2.353 4.493-6.74a.75.75 0 0 1 1.04-.207Z"
              clip-rule="evenodd"
            />
          </svg>
        </Show>
      </button>

      <div class="min-w-0 flex-1">
        <div class="flex items-center gap-2">
          <span
            class={`text-sm font-medium ${
              isDone()
                ? "text-qtools-500 line-through dark:text-qtools-400"
                : "text-qtools-900 dark:text-qtools-50"
            }`}
          >
            {props.assignment.name}
          </span>
          <Show when={props.assignment.is_quiz}>
            <span class="rounded bg-qtools-200 px-1.5 py-0.5 text-[10px] font-bold uppercase text-qtools-700 dark:bg-qtools-700 dark:text-qtools-200">
              Quiz
            </span>
          </Show>
        </div>
        <div class="flex items-center gap-3 text-xs text-qtools-500 dark:text-qtools-400">
          <span
            class={
              urgency() === "overdue"
                ? "font-medium text-red-600 dark:text-red-400"
                : urgency() === "soon"
                  ? "font-medium text-amber-600 dark:text-amber-400"
                  : ""
            }
          >
            {formatDue()}
          </span>
          <Show when={props.assignment.points_possible !== null}>
            <span>{props.assignment.points_possible} pts</span>
          </Show>
          <Show when={props.showCourse}>
            <span class="rounded bg-qtools-100 px-1.5 py-0.5 dark:bg-qtools-800">
              {props.assignment.course_name}
            </span>
          </Show>
          <Show when={props.assignment.has_submitted_submissions && !props.assignment.manually_completed}>
            <span class="text-emerald-600 dark:text-emerald-400">Submitted</span>
          </Show>
        </div>
      </div>

      <a
        href={props.assignment.html_url}
        target="_blank"
        rel="noopener noreferrer"
        class="shrink-0 rounded-lg p-1.5 text-qtools-400 transition-colors hover:bg-qtools-100 hover:text-qtools-600 dark:text-qtools-500 dark:hover:bg-qtools-800 dark:hover:text-qtools-300"
        title="Open in Canvas"
      >
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-4">
          <path d="M8.987 1.929a.75.75 0 0 0-1.473 0l-.157.627a3.25 3.25 0 0 1-2.301 2.3l-.627.158a.75.75 0 0 0 0 1.472l.627.157a3.25 3.25 0 0 1 2.301 2.301l.157.627a.75.75 0 0 0 1.473 0l.157-.627a3.25 3.25 0 0 1 2.301-2.301l.627-.157a.75.75 0 0 0 0-1.472l-.627-.158a3.25 3.25 0 0 1-2.301-2.3l-.157-.627Z" />
          <path d="M4.238 10.63a.5.5 0 0 0-.982 0l-.086.346a2.167 2.167 0 0 1-1.534 1.534l-.346.086a.5.5 0 0 0 0 .982l.346.086a2.167 2.167 0 0 1 1.534 1.534l.086.346a.5.5 0 0 0 .982 0l.086-.346a2.167 2.167 0 0 1 1.534-1.534l.346-.086a.5.5 0 0 0 0-.982l-.346-.086a2.167 2.167 0 0 1-1.534-1.534l-.086-.346Z" />
        </svg>
      </a>
    </div>
  );
}
