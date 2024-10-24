import { customFetch } from "./core";
import { UpdateUserRequest, UserResponse } from "@/types/user";

const ENDPOINT = "/users";

export async function getMeDetails(): Promise<UserResponse> {
  const { data } = await customFetch<"GET", undefined, UserResponse>(
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

export async function updateUserProfile(
  userData: UpdateUserRequest
): Promise<void> {
  await customFetch<"PUT", UpdateUserRequest, void>(`${ENDPOINT}/me`, {
    method: "PUT",
    body: userData,
  });
}
