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
