"use server";

import { customFetch } from "@/lib/api/core";
import { UpdateUserRequest, UserResponse } from "@/types/user";
import { revalidateTag } from "next/cache";

const ENDPOINT = "/users";

/**
 * ユーザーを更新する関数
 */
export async function updateUserAction(
  userData: UpdateUserRequest
): Promise<{ success: boolean; error?: string }> {
  try {
    await customFetch<UpdateUserRequest, void>(`${ENDPOINT}/me`, {
      method: "PUT",
      body: userData,
    });
    revalidateTag("user-profile");
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
