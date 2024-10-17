import { toast } from "@/lib/hooks/use-toast";
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
  credentials?: RequestCredentials;
  params?: FetchParams<T, M>;
  body?: FetchBody<T, M>;
  cache?: RequestCache;
}

// APIエラーを表すカスタムエラークラス
export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}

// カスタムフェッチ関数
export async function customFetch<
  M extends HttpMethod,
  RequestInput extends Record<string, any> | undefined = Record<string, any>,
  RequestResult = unknown
>(
  endpoint: string,
  {
    headers: optionHeaders,
    credentials,
    method,
    body,
    params,
    cache = "no-cache",
  }: IFetchOptions<
    RequestInput extends Record<string, any>
      ? RequestInput
      : Record<string, never>,
    M
  >
) {
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
  const headers = new Headers({
    ...optionHeaders,
    ...authHeader,
  });

  if (!(body instanceof FormData) && !headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  const fetchOptions: RequestInit = {
    method,
    headers,
    cache,
    mode: "cors",
    credentials,
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
    const response = await fetch(url, fetchOptions);

    // TODO: 401エラーの場合、アクセストークンを更新して再リクエスト
    if (!response.ok) {
      const errorMessage = getErrorMessage(response.status);
      throw new ApiError(response.status, errorMessage);
    }

    // 204 No Content の場合は空のオブジェクトを返す
    if (response.status === 204) {
      return {} as RequestResult;
    }

    // レスポンスにコンテンツがある場合のみJSONとしてパースする
    const contentType = response.headers.get("content-type");
    if (contentType && contentType.includes("application/json")) {
      return response.json() as Promise<RequestResult>;
    }

    // コンテンツがない場合は空のオブジェクトを返す
    return {} as RequestResult;
  } catch (error) {
    handleFetchError(error);
    throw error;
  }
}

function handleFetchError(error: unknown) {
  if (error instanceof ApiError) {
    if (typeof window !== "undefined") {
      // クライアントサイドの場合はtoastを使用
      toast({
        variant: "error",
        title: "エラー",
        description: error.message,
      });
    } else {
      // サーバーサイドの場合はコンソールにエラーを出力
      console.error("API Error:", error.message);
    }
  } else {
    if (typeof window !== "undefined") {
      // クライアントサイドの場合はtoastを使用
      toast({
        variant: "error",
        title: "エラー",
        description: "予期せぬエラーが発生しました。",
      });
    } else {
      // サーバーサイドの場合はコンソールにエラーを出力
      console.error("Unexpected Error:", error);
    }
  }
}

// エラーメッセージを取得する関数
function getErrorMessage(status: number): string {
  switch (status) {
    case 400:
      return "リクエストに問題があります。入力内容を確認してください。";
    case 401:
      return "認証に失敗しました。再度ログインしてください。";
    case 403:
      return "アクセス権限がありません。";
    case 404:
      return "リソースが見つかりません。";
    case 413:
      return "アップロードされたファイルが大きすぎます。より小さいファイルを選択してください。";
    case 422:
      return "入力内容に誤りがあります。確認して再度お試しください。";
    case 500:
      return "サーバーエラーが発生しました。しばらく経ってから再度お試しください。";
    default:
      return "予期せぬエラーが発生しました。";
  }
}
