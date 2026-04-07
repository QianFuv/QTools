export interface CanvasSettings {
  api_url: string;
  api_token: string;
  cache_ttl_minutes: number;
  ignored_course_ids: number[];
}

export interface CanvasCourse {
  id: number;
  name: string;
  course_code: string;
}

export interface CanvasAssignment {
  id: number;
  course_id: number;
  course_name: string;
  name: string;
  due_at: string | null;
  points_possible: number | null;
  html_url: string;
  submission_types: string[];
  has_submitted_submissions: boolean;
  is_quiz: boolean;
  manually_completed: boolean;
}

export interface CanvasData {
  courses: CanvasCourse[];
  assignments: CanvasAssignment[];
  fetched_at: string;
}
