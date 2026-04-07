import { createMemo, createSignal, For, Show } from "solid-js";
import type { CanvasAssignment, CanvasData, CanvasSettings } from "./types";
import AssignmentCard from "./AssignmentCard";

interface CourseViewProps {
  data: CanvasData | null;
  loading: boolean;
  settings: CanvasSettings | null;
  hideCompleted: boolean;
  onToggleCompletion: (id: number, completed: boolean) => void;
}

interface CourseGroup {
  courseId: number;
  courseName: string;
  courseCode: string;
  assignments: CanvasAssignment[];
}

export default function CourseView(props: CourseViewProps) {
  const [collapsed, setCollapsed] = createSignal<Set<number>>(new Set());

  const toggleCollapse = (courseId: number) => {
    setCollapsed((prev) => {
      const next = new Set(prev);
      if (next.has(courseId)) {
        next.delete(courseId);
      } else {
        next.add(courseId);
      }
      return next;
    });
  };

  const courseGroups = createMemo((): CourseGroup[] => {
    if (!props.data) return [];

    const ignored = new Set(props.settings?.ignored_course_ids ?? []);
    const filtered = props.data.assignments.filter((a) => {
      if (ignored.has(a.course_id)) return false;
      if (props.hideCompleted && (a.manually_completed || a.has_submitted_submissions))
        return false;
      return true;
    });

    const map = new Map<number, CourseGroup>();
    for (const a of filtered) {
      let group = map.get(a.course_id);
      if (!group) {
        const course = props.data.courses.find((c) => c.id === a.course_id);
        group = {
          courseId: a.course_id,
          courseName: a.course_name,
          courseCode: course?.course_code ?? "",
          assignments: [],
        };
        map.set(a.course_id, group);
      }
      group.assignments.push(a);
    }

    const groups = [...map.values()];
    for (const g of groups) {
      g.assignments.sort((a, b) => {
        if (!a.due_at && !b.due_at) return 0;
        if (!a.due_at) return 1;
        if (!b.due_at) return -1;
        return new Date(a.due_at).getTime() - new Date(b.due_at).getTime();
      });
    }

    return groups.sort((a, b) => a.courseName.localeCompare(b.courseName));
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
            when={courseGroups().length > 0}
            fallback={
              <div class="flex items-center justify-center py-12 text-qtools-500 dark:text-qtools-400">
                No assignments found.
              </div>
            }
          >
            <div class="flex flex-col gap-4">
              <For each={courseGroups()}>
                {(group) => {
                  const isCollapsed = () => collapsed().has(group.courseId);
                  return (
                    <div class="overflow-hidden rounded-xl border border-qtools-200 dark:border-qtools-700">
                      <button
                        onClick={() => toggleCollapse(group.courseId)}
                        class="flex w-full items-center justify-between bg-qtools-100 px-4 py-3 text-left transition-colors hover:bg-qtools-200 dark:bg-qtools-800 dark:hover:bg-qtools-700"
                      >
                        <div>
                          <h3 class="text-sm font-bold text-qtools-900 dark:text-qtools-50">
                            {group.courseName}
                          </h3>
                          <span class="text-xs text-qtools-500 dark:text-qtools-400">
                            {group.courseCode} &middot; {group.assignments.length} assignment
                            {group.assignments.length !== 1 ? "s" : ""}
                          </span>
                        </div>
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          viewBox="0 0 20 20"
                          fill="currentColor"
                          class={`size-5 text-qtools-500 transition-transform dark:text-qtools-400 ${
                            isCollapsed() ? "-rotate-90" : ""
                          }`}
                        >
                          <path
                            fill-rule="evenodd"
                            d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
                            clip-rule="evenodd"
                          />
                        </svg>
                      </button>

                      <Show when={!isCollapsed()}>
                        <div class="flex flex-col gap-2 p-3">
                          <For each={group.assignments}>
                            {(assignment) => (
                              <AssignmentCard
                                assignment={assignment}
                                showCourse={false}
                                onToggleCompletion={props.onToggleCompletion}
                              />
                            )}
                          </For>
                        </div>
                      </Show>
                    </div>
                  );
                }}
              </For>
            </div>
          </Show>
        </Show>
      </Show>
    </div>
  );
}
