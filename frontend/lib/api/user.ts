import { customFetch } from "./core";
import { UpdateUserRequest, UserResponse } from "@/types/user";

const ENDPOINT = "/users";

export async function getMeDetails(): Promise<UserResponse> {
  const response = await customFetch<"GET", undefined, UserResponse>(
    `${ENDPOINT}/me`,
    {
      method: "GET",
      next: { tags: ["user-profile"] },
    }
  );

  // avatar_urlを変換
  if (response.avatar_url) {
    response.avatar_url = response.avatar_url.replace(
      "minio:9000",
      process.env.NEXT_PUBLIC_MINIO_PUBLIC_URL?.replace("http://", "") ||
        "localhost:9000"
    );
  }

  return response;
}

export async function updateUserProfile(
  userData: UpdateUserRequest
): Promise<void> {
  await customFetch<"PUT", UpdateUserRequest, void>(`${ENDPOINT}/me`, {
    method: "PUT",
    body: userData,
  });
}
