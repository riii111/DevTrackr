"use server";

import { customFetch } from "@/lib/api/core";
import { AuthResponse, AuthTokenCreatedResponse } from "@/types/user";
import { proxyServerCookies } from "@/lib/utils/cookiesForServer";
import { redirect } from "next/navigation";
import { z } from "zod";

const ENDPOINT = "/auth";

const loginSchema = z.object({
  email: z.string().email("有効なメールアドレスを入力してください"),
  password: z.string().min(8, "パスワードは8文字以上である必要があります"),
});

export type LoginActionResult = {
  success: boolean;
  error?: string;
};

/**
 * ユーザーログイン関数
 */
export async function loginAction(
  email: string,
  password: string
): Promise<LoginActionResult> {
  try {
    // セキュリティ対策としてサーバーサイドでもバリデーション
    loginSchema.parse({ email, password });

    const { headers } = await customFetch<
      "POST",
      { email: string; password: string },
      AuthResponse
    >(`${ENDPOINT}/login/`, {
      method: "POST",
      body: { email, password },
    });

    // レスポンスヘッダーからCookieを設定
    await proxyServerCookies(headers);

    // サーバ側でリダイレクトすれば追加のラウンドトリップは不要、早く到達する
    redirect("/dashboard");

    return { success: true };
  } catch (error) {
    console.error("ログインに失敗しました", error);
    if (error instanceof z.ZodError) {
      return { success: false, error: "入力内容が正しくありません。" };
    } else if (error instanceof Error) {
      return { success: false, error: error.message };
    }
    return { success: false, error: "予期せぬエラーが発生しました。" };
  }
}

const registerSchema = z.object({
  username: z.string().min(1, "名前を入力してください"),
  email: z.string().email("有効なメールアドレスを入力してください"),
  password: z.string().min(8, "パスワードは8文字以上である必要があります"),
});

export type RegisterActionResult = {
  success: boolean;
  error?: string;
};

/**
 * ユーザー登録関数
 */
export async function registerAction(
  username: string,
  email: string,
  password: string
): Promise<RegisterActionResult> {
  try {
    // セキュリティ対策としてサーバーサイドでもバリデーション
    registerSchema.parse({ username, email, password });

    const { data, headers } = await customFetch<
      "POST",
      { username: string; email: string; password: string },
      AuthTokenCreatedResponse
    >(`${ENDPOINT}/register/`, {
      method: "POST",
      body: { username, email, password },
    });

    // レスポンスヘッダーからCookieを設定
    await proxyServerCookies(headers);

    // サーバ側でリダイレクトすれば追加のラウンドトリップは不要、早く到達する
    redirect("/dashboard");

    return { success: true };
  } catch (error) {
    console.error("アカウント登録に失敗しました", error);
    if (error instanceof z.ZodError) {
      return { success: false, error: "入力内容が正しくありません。" };
    } else if (error instanceof Error) {
      return { success: false, error: error.message };
    }
    return { success: false, error: "予期せぬエラーが発生しました。" };
  }
}
