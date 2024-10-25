import { customFetch } from "./core";
import { UpdateUserRequest, UserResponse } from "@/types/user";

const ENDPOINT = "/users";

/**
 * ユーザーのプロフィールを取得する関数
 */
export async function getMeDetails(): Promise<UserResponse> {
  const { data } = await customFetch<undefined, UserResponse>(
    `${ENDPOINT}/me`,
    {
      method: "GET",
      next: { tags: ["user-profile"] },
    }
  );

  // avatar_urlを変換
  if (data.avatar_url) {
    data.avatar_url = data.avatar_url.replace(
      "minio:9000",
      process.env.NEXT_PUBLIC_MINIO_PUBLIC_URL?.replace("http://", "") ||
        "localhost:9000"
    );
  }

  return data;
}

/**
 * ユーザーのプロフィールを更新する関数
 */
export async function updateUserProfile(
  userData: UpdateUserRequest
): Promise<void> {
  await customFetch<UpdateUserRequest, void>(`${ENDPOINT}/me`, {
    method: "PUT",
    body: userData,
  });
}
