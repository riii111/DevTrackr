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

    if (!accessToken || !accessToken.value) {
      try {
        // リフレッシュトークンを含むCookieを転送
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_API_BASE_URL}/auth/refresh/`,
          {
            method: "POST",
            headers: {
              Cookie: request.headers.get("cookie") || "",
            },
            credentials: "include",
          }
        );

        if (response.ok) {
          const newResponse = NextResponse.next();
          // TODO: 再度見直し. バックエンド側でセットしてるけど反映されないのでセットしている.
          newResponse.cookies.set("access_token", await response.text(), {
            httpOnly: true,
            secure: process.env.NODE_ENV === "production",
            sameSite: "strict",
          });
          return newResponse;
        } else {
          // エラーは握りつぶした上で、ログイン画面にリダイレクト
          return NextResponse.redirect(new URL("/auth", request.url));
        }
      } catch (error) {
        // エラーは握りつぶした上で、ログイン画面にリダイレクト
        return NextResponse.redirect(new URL("/auth", request.url));
      }
    }
  }

  return NextResponse.next();
}

// ミドルウェアを適用するルートを指定
export const config = {
  matcher: ["/dashboard/:path*", "/api/:path*"],
};
