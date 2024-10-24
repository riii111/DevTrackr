import { customFetch } from "@/lib/api/core";

const ENDPOINT = "/auth";

/**
 * ユーザーログアウト関数
 */
export async function logout(): Promise<void> {
  await customFetch(`${ENDPOINT}/logout/`, {
    method: "POST",
    credentials: "include",
  });
}
