import { NextResponse } from "next/server";
import type { NextRequest } from "next/server";

const protectedRoutes = ["/dashboard"]; // 保護対象のルートをここに追加

export async function middleware(request: NextRequest) {
  console.log("ミドルウェアが呼び出されました:", request.url);
  const { pathname } = request.nextUrl;

  // APIルートや公開ルートは許可
  if (
    pathname.startsWith("/api") ||
    pathname.startsWith("/public") ||
    pathname.startsWith("/auth")
  ) {
    console.log("公開ルートへのアクセス、許可します:", pathname);
    return NextResponse.next();
  }

  // 保護対象のルートにアクセスする場合
  if (protectedRoutes.some((route) => pathname.startsWith(route))) {
    console.log("保護対象のルートへのアクセス:", pathname);
    const accessToken = request.cookies.get("access_token");
    console.log("検出されたアクセストークン:", accessToken?.value);

    if (!accessToken || !accessToken.value) {
      console.log(
        "アクセストークンがありません。ログインページへリダイレクトします。"
      );
      return NextResponse.redirect(new URL("/auth", request.url));
    }
    console.log("アクセストークンが見つかりました。アクセスを許可します。");
  }

  return NextResponse.next();
}

// ミドルウェアを適用するルートを指定
export const config = {
  matcher: ["/dashboard/:path*"],
};
