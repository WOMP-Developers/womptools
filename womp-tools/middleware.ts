import { NextRequest, NextResponse } from "next/server";

export function middleware(request: NextRequest) {
    const refresh_token = request.cookies.get('refresh_token')?.value;

    if (!refresh_token) {
        return NextResponse.redirect(new URL('/', request.url));
    }

    return NextResponse.next();
}

export const config = {
    matcher: ['/dashboard/:path*']
}