"use server";
import { cookies } from "next/headers";
import { COOKIE_KEYS } from "@/lib/utils/cookies";

export async function getServerSideCookie(
  key: string
): Promise<string | undefined> {
  const cookieStore = cookies();
  return cookieStore.get(key)?.value;
}

export async function getServerSideAuthHeader(): Promise<
  Record<string, string>
> {
  const accessToken = await getServerSideCookie(COOKIE_KEYS.ACCESS_TOKEN);
  return accessToken ? { Authorization: `Bearer ${accessToken}` } : {};
}
