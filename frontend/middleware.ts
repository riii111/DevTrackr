import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

const protectedRoutes = ["/dashboard"]; // 保護対象のルートをここに追加
const publicRoutes = ["/auth"];

export async function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // APIルートは全て許可
  if (pathname.startsWith("/api")) {
    return NextResponse.next();
  }

  // 公開ルートは常に許可
  if (publicRoutes.some((route) => pathname.startsWith(route))) {
    return NextResponse.next();
  }

  // 保護対象のルートにアクセスする場合
  if (protectedRoutes.some((route) => pathname.startsWith(route))) {
    let accessToken = request.cookies.get("access_token");

    // アクセストークンが存在しない、または無効（期限切れを含む）の場合
    if (
      !accessToken ||
      !accessToken.value ||
      isTokenExpired(accessToken.value)
    ) {
      try {
        // リフレッシュトークンを使用してアクセストークンを更新
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_API_BASE_URL}/auth/refresh/`,
          {
            method: "POST",
            headers: {
              Cookie: request.headers.get("cookie") || "",
            },
          }
        );

        if (response.ok) {
          return NextResponse.next();
        } else {
          // リフレッシュに失敗した場合、ログイン画面にリダイレクト
          return NextResponse.redirect(new URL("/auth", request.url));
        }
      } catch (error) {
        // エラーの場合、ログイン画面にリダイレクト
        return NextResponse.redirect(new URL("/auth", request.url));
      }
    }
  }

  return NextResponse.next();
}

// トークンが期限切れかどうかを確認する関数
function isTokenExpired(token: string): boolean {
  console.log("isTokenExpired called");
  try {
    const payload = JSON.parse(atob(token.split(".")[1]));
    const exp = payload.exp * 1000; // JWTの有効期限はUNIXタイムスタンプ（秒）なので、ミリ秒に変換
    console.log("exp:", exp);
    return Date.now() >= exp;
  } catch (error) {
    console.error("Token validation error:", error);
    return true; // エラーが発生した場合は、安全のためトークンを期限切れとみなす
  }
}

// ミドルウェアを適用するルートを指定
export const config = {
  matcher: ["/dashboard/:path*", "/api/:path*"],
};
