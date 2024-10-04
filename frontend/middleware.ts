import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

const API_BASE_URL = process.env.NEXT_PUBLIC_API_BASE_URL;

const protectedRoutes = ['/dashboard']; // 保護対象のルートをここに追加

export async function middleware(request: NextRequest) {
  console.log('ミドルウェアが呼び出されました:', request.url);
  const { pathname } = request.nextUrl;

  // APIルートや公開ルートは許可
  if (
    pathname.startsWith('/api') ||
    pathname.startsWith('/public') ||
    pathname.startsWith('/auth')
  ) {
    console.log('公開ルートへのアクセス、許可します:', pathname);
    return NextResponse.next();
  }

  // 保護対象のルートにアクセスする場合
  if (protectedRoutes.some(route => pathname.startsWith(route))) {
    console.log('保護対象のルートへのアクセス:', pathname);
    const accessToken = request.cookies.get('access_token');
    const refreshToken = request.cookies.get('refresh_token');
    console.log('検出されたアクセストークン:', accessToken);
    console.log('検出されたリフレッシュトークン:', refreshToken);
    console.log('全てのCookies:', request.cookies.getAll());

    if (!accessToken || !accessToken.value) {
      console.log('アクセストークンがありません。ログインページへリダイレクトします。');
      // ログインページへリダイレクト
      const loginUrl = new URL('/auth', request.url);
      return NextResponse.redirect(loginUrl);
    }
    console.log('アクセストークンが見つかりました。アクセスを許可します。');
  }

  return NextResponse.next();
}

// ミドルウェアを適用するルートを指定
export const config = {
  matcher: ['/dashboard/:path*', '/'],
};
