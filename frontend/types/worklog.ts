export interface WorkLog {
  id?: string;
  project_id: string;
  start_time: string;
  end_time?: string;
  memo?: string;
  created_at: string;
  updated_at?: string;
}

export interface CreateWorkLogRequest {
  project_id: string;
  start_time: string;
  end_time?: string;
  memo?: string;
  break_time?: number;
  actual_work_minutes?: number;
}

export interface UpdateWorkLogRequest {
  project_id: string;
  start_time: string;
  end_time?: string;
  memo?: string;
  break_time?: number;
  actual_work_minutes?: number;
}

export type WorkLogResponse = WorkLog & {
  id: string;
};
