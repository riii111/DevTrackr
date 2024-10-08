import { toast } from "@/lib/hooks/use-toast";
import { ApiResponse } from "@/types/api";
import { getClientSideAuthHeader } from "@/lib/utils/cookies";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

export type HttpMethod = "GET" | "POST" | "PUT" | "DELETE" | "PATCH";

// GETメソッドの場合はクエリパラメータ、それ以外はリクエストボディの型を定義
type FetchParams<T, M extends HttpMethod> = M extends "GET"
  ? T
  : Record<string, string | number | boolean>;
type FetchBody<T, M extends HttpMethod> = M extends
  | "POST"
  | "PUT"
  | "DELETE"
  | "PATCH"
  ? T | FormData
  : never;

// フェッチオプションのインターフェース定義
interface IFetchOptions<T extends Record<string, any>, M extends HttpMethod> {
  headers?: Record<string, any>;
  method: M;
  params?: FetchParams<T, M>;
  body?: FetchBody<T, M>;
  cache?: RequestCache;
}

// APIエラーを表すカスタムエラークラス
class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}

// カスタムフェッチ関数
export async function customFetch<
  M extends HttpMethod,
  RequestInput extends Record<string, any> = Record<string, any>,
  RequestResult = unknown
>(
  endpoint: string,
  {
    headers: optionHeaders,
    method,
    body,
    params,
    cache = "no-cache",
  }: IFetchOptions<RequestInput, M>
): Promise<ApiResponse<RequestResult>> {
  if (!API_BASE_URL) {
    throw new Error("API_BASE_URL is not defined");
  }

  // 末尾にスラッシュ追加する事でプロキシ・バックエンドなどのサーバー側のリダイレクトを回避し、パフォーマンスを僅かに向上させる
  let url = `${API_BASE_URL}${endpoint}${endpoint.endsWith("/") ? "" : "/"}`;

  async function getAuthHeader(): Promise<Record<string, string>> {
    if (typeof window === "undefined") {
      const { getServerSideAuthHeader } = await import(
        "@/lib/utils/cookiesForServer"
      );
      return getServerSideAuthHeader();
    } else {
      return getClientSideAuthHeader();
    }
  }

  const authHeader = await getAuthHeader();
  console.log("authHeader", authHeader);
  const headers = new Headers({
    ...optionHeaders,
    ...authHeader,
  });

  if (!headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  const fetchOptions: RequestInit = {
    method,
    headers,
    credentials: "include",
    cache,
    mode: "cors",
  };

  if (body) {
    fetchOptions.body = body instanceof FormData ? body : JSON.stringify(body);
  }

  // GETメソッドの場合、クエリパラメータをURLに追加
  if (params && method === "GET") {
    const searchParams = new URLSearchParams(params as Record<string, string>);
    url += `?${searchParams.toString()}`;
  }

  try {
    let response = await fetch(url, fetchOptions);

    // TODO: 401エラーの場合、アクセストークンを更新して再リクエスト

    if (!response.ok) {
      let errorMessage = "エラーが発生しました";
      try {
        const errorData = await response.json();
        errorMessage = errorData.message || errorMessage;
      } catch (e) {
        // JSON解析に失敗した場合は、デフォルトのエラーメッセージを使用
      }
      throw new ApiError(response.status, errorMessage);
    }

    const data: ApiResponse<RequestResult> = await response.json();
    return data;
  } catch (error) {
    if (error instanceof ApiError) {
      if (typeof window !== "undefined") {
        // クライアントサイドの場合のみtoastを使用
        toast({
          variant: "destructive",
          title: "エラー",
          description: error.message,
        });
      } else {
        // サーバーサイドの場合はコンソールにエラーを出力
        console.error("API Error:", error.message);
      }
    } else {
      if (typeof window !== "undefined") {
        // クライアントサイドの場合のみtoastを使用
        toast({
          variant: "destructive",
          title: "エラー",
          description: "予期せぬエラーが発生しました。",
        });
      } else {
        // サーバーサイドの場合はコンソールにエラーを出力
        console.error("Unexpected Error:", error);
      }
    }
    throw error;
  }
}
