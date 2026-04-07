import { createMemo, For, Show } from "solid-js";
import type { CanvasAssignment, CanvasData, CanvasSettings } from "./types";
import AssignmentCard from "./AssignmentCard";

interface TimelineViewProps {
  data: CanvasData | null;
  loading: boolean;
  settings: CanvasSettings | null;
  hideCompleted: boolean;
  onToggleCompletion: (id: number, completed: boolean) => void;
}

interface TimelineGroup {
  label: string;
  assignments: CanvasAssignment[];
}

export default function TimelineView(props: TimelineViewProps) {
  const groups = createMemo((): TimelineGroup[] => {
    if (!props.data) return [];

    const now = new Date();
    const todayStart = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const tomorrowStart = new Date(todayStart.getTime() + 86_400_000);
    const weekEnd = new Date(todayStart.getTime() + 7 * 86_400_000);

    const overdue: CanvasAssignment[] = [];
    const today: CanvasAssignment[] = [];
    const tomorrow: CanvasAssignment[] = [];
    const thisWeek: CanvasAssignment[] = [];
    const later: CanvasAssignment[] = [];
    const noDue: CanvasAssignment[] = [];

    const ignored = new Set(props.settings?.ignored_course_ids ?? []);
    const filtered = props.data.assignments.filter((a) => {
      if (ignored.has(a.course_id)) return false;
      if (props.hideCompleted && (a.manually_completed || a.has_submitted_submissions))
        return false;
      return true;
    });

    const sorted = [...filtered].sort((a, b) => {
      if (!a.due_at && !b.due_at) return 0;
      if (!a.due_at) return 1;
      if (!b.due_at) return -1;
      return new Date(a.due_at).getTime() - new Date(b.due_at).getTime();
    });

    for (const a of sorted) {
      if (!a.due_at) {
        noDue.push(a);
        continue;
      }
      const due = new Date(a.due_at);
      if (due < now && !a.has_submitted_submissions && !a.manually_completed) {
        overdue.push(a);
      } else if (due >= todayStart && due < tomorrowStart) {
        today.push(a);
      } else if (due >= tomorrowStart && due < new Date(tomorrowStart.getTime() + 86_400_000)) {
        tomorrow.push(a);
      } else if (due >= tomorrowStart && due < weekEnd) {
        thisWeek.push(a);
      } else if (due >= weekEnd) {
        later.push(a);
      } else {
        later.push(a);
      }
    }

    const result: TimelineGroup[] = [];
    if (overdue.length) result.push({ label: "Overdue", assignments: overdue });
    if (today.length) result.push({ label: "Today", assignments: today });
    if (tomorrow.length) result.push({ label: "Tomorrow", assignments: tomorrow });
    if (thisWeek.length) result.push({ label: "This Week", assignments: thisWeek });
    if (later.length) result.push({ label: "Later", assignments: later });
    if (noDue.length) result.push({ label: "No Due Date", assignments: noDue });
    return result;
  });

  return (
    <div class="p-4">
      <Show
        when={!props.loading || props.data}
        fallback={
          <div class="flex items-center justify-center py-12 text-qtools-500 dark:text-qtools-400">
            Loading Canvas data...
          </div>
        }
      >
        <Show
          when={props.data}
          fallback={
            <div class="flex flex-col items-center justify-center gap-3 py-12 text-qtools-500 dark:text-qtools-400">
              <p>No data loaded. Configure your Canvas API token in Settings and click Refresh.</p>
            </div>
          }
        >
          <Show
            when={groups().length > 0}
            fallback={
              <div class="flex items-center justify-center py-12 text-qtools-500 dark:text-qtools-400">
                No assignments found.
              </div>
            }
          >
            <div class="flex flex-col gap-6">
              <For each={groups()}>
                {(group) => (
                  <div>
                    <h2
                      class={`mb-3 text-sm font-bold uppercase tracking-wider ${
                        group.label === "Overdue"
                          ? "text-red-600 dark:text-red-400"
                          : "text-qtools-600 dark:text-qtools-300"
                      }`}
                    >
                      {group.label}
                      <span class="ml-2 font-normal text-qtools-400 dark:text-qtools-500">
                        ({group.assignments.length})
                      </span>
                    </h2>
                    <div class="flex flex-col gap-2">
                      <For each={group.assignments}>
                        {(assignment) => (
                          <AssignmentCard
                            assignment={assignment}
                            showCourse
                            onToggleCompletion={props.onToggleCompletion}
                          />
                        )}
                      </For>
                    </div>
                  </div>
                )}
              </For>
            </div>
          </Show>
        </Show>
      </Show>
    </div>
  );
}
