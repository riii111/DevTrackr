import { getSession } from "next-auth/react";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

export async function refreshAccessToken() {
  const session = await getSession();
  if (!session) throw new Error("No active session");

  const response = await fetch(`${API_BASE_URL}/api/auth/refresh`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    credentials: "include", // Cookieを送信
  });

  if (!response.ok) throw new Error("Failed to refresh token");

  const data = await response.json();
  return data.access_token;
}
