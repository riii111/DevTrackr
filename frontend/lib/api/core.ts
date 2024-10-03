import { toast } from "@/lib/hooks/use-toast";
import { ApiResponse } from "@/types/api";

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

const HTTP_STATUS = {
  BAD_REQUEST: 400,
  UNAUTHORIZED: 401,
  FORBIDDEN: 403,
  TOO_MANY_REQUESTS: 429,
  INTERNAL_SERVER_ERROR: 500,
};

class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message);
  }
}

export async function fetchApi<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<ApiResponse<T>> {
  const headers = new Headers(options.headers);

  if (!headers.has("Content-Type")) {
    headers.set("Content-Type", "application/json");
  }

  let response: Response;

  try {
    response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...options,
      headers,
      credentials: "include",
      mode: "cors",
    });

    if (response.status === HTTP_STATUS.UNAUTHORIZED) {
      // try {
      //   const newAccessToken = await refreshAccessToken();
      //   headers.set("Authorization", `Bearer ${newAccessToken}`);
      //   response = await fetch(`${API_BASE_URL}${endpoint}`, {
      //     ...options,
      //     headers,
      //     credentials: "include",
      //   });
      // } catch (refreshError) {
      // TODO: リフレッシュトークン実行する処理を追加
      window.location.href = "/auth";
      throw new ApiError(
        HTTP_STATUS.UNAUTHORIZED,
        "セッションが切れました。再度ログインしてください。"
      );
      // }
    }

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

    const data: ApiResponse<T> = await response.json();
    return data;
  } catch (error) {
    if (error instanceof ApiError) {
      toast({
        variant: "destructive",
        title: "エラー",
        description: error.message,
      });
    } else {
      toast({
        variant: "destructive",
        title: "エラー",
        description: "予期せぬエラーが発生しました。",
      });
    }
    throw error;
  }
}
