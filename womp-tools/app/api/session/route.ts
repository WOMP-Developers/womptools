import { NextRequest, NextResponse } from "next/server";

const ACCESS_TOKEN_COOKIE_EXPIRE = 1000 * (60 * 60 * 2);        // 2 hours
const REFRESH_TOKEN_COOKIE_EXPIRE = 1000 * (60 * 60 * 24 * 30); // 30 days

type RequestBody = {
    refresh_token?: string,
    access_token?: string,
}

export async function POST(req: NextRequest) {
    const body = await req.json() as RequestBody;
    const res = new NextResponse();

    const now = Date.now();

    if (body.refresh_token) {
        res.cookies.set('refresh_token', body.refresh_token, {
            sameSite: 'strict',
            httpOnly: true,
            secure: true,
            expires: now + REFRESH_TOKEN_COOKIE_EXPIRE,
        });
    } else {
        res.cookies.set('refresh_token', '', {
            expires: 0
        });
    }

    if (body.access_token) {
        res.cookies.set('access_token', body.access_token, {
            sameSite: 'strict',
            httpOnly: true,
            secure: true,
            expires: now + ACCESS_TOKEN_COOKIE_EXPIRE,
        })
    } else {
        res.cookies.set('access_token', '', {
            expires: 0
        });
    }

    return res;
}
