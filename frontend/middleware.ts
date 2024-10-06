import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

const protectedRoutes = ["/dashboard"]; // 保護対象のルートをここに追加

export async function middleware(request: NextRequest) {
  const { pathname } = request.nextUrl;

  // APIルートや公開ルートは許可
  if (
    pathname.startsWith("/api") ||
    pathname.startsWith("/public") ||
    pathname.startsWith("/auth")
  ) {
    return NextResponse.next();
  }

  // 保護対象のルートにアクセスする場合
  if (protectedRoutes.some((route) => pathname.startsWith(route))) {
    const accessToken = request.cookies.get("access_token");

    if (!accessToken || !accessToken.value) {
      return NextResponse.redirect(new URL("/auth", request.url));
    }
  }

  return NextResponse.next();
}

// ミドルウェアを適用するルートを指定
export const config = {
  matcher: ["/dashboard/:path*"],
};
