"use server";

import { customFetch } from "@/lib/api/core";
import {
  CreateWorkLogRequest,
  UpdateWorkLogRequest,
  WorkLogResponse,
} from "@/types/worklog";

const ENDPOINT = "/work-logs";

/**
 * 稼働記録を新規作成する関数
 */
export async function createWorkLogAction(
  workLogData: CreateWorkLogRequest
): Promise<{ success: boolean; error?: string; data: WorkLogResponse }> {
  try {
    const response = await customFetch<CreateWorkLogRequest, WorkLogResponse>(
      ENDPOINT,
      {
        method: "POST",
        body: workLogData,
      }
    );
    return {
      success: true,
      data: response.data,
    };
  } catch (error) {
    return {
      success: false,
      error:
        error instanceof Error ? error.message : "予期せぬエラーが発生しました",
      data: {} as WorkLogResponse,
    };
  }
}

/**
 * 稼働記録を更新する関数
 */
export async function updateWorkLogAction(
  workLogId: string,
  workLogData: UpdateWorkLogRequest
): Promise<{ success: boolean; error?: string }> {
  try {
    await customFetch<UpdateWorkLogRequest, WorkLogResponse>(
      `${ENDPOINT}/${workLogId}`,
      {
        method: "PUT",
        body: workLogData,
      }
    );
    return {
      success: true,
    };
  } catch (error) {
    return {
      success: false,
      error:
        error instanceof Error ? error.message : "予期せぬエラーが発生しました",
    };
  }
}
