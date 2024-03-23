'use server'

import { cookies } from "next/headers";

// const SESSION_URL = `${process.env.ENDPOINT_WEBS}/api/session`;

const ACCESS_TOKEN_COOKIE_EXPIRE = 1000 * (60 * 60 * 2);        // 2 hours
const REFRESH_TOKEN_COOKIE_EXPIRE = 1000 * (60 * 60 * 24 * 30); // 30 days

export async function clearSessionCookies() {
    // const response = await fetch(SESSION_URL, {
    //     method: 'POST',
    //     body: JSON.stringify({})
    // });

    // if (!response.ok) {
    //     console.error("couldn't clear session cookies");
    // }

    const cookie_store = cookies();
    cookie_store.delete('access_token');
    cookie_store.delete('refresh_token');
}

export async function setSessionCookies(access_token: string, refresh_token: string) {

    // const response = await fetch(SESSION_URL, {
    //     method: 'POST',
    //     body: JSON.stringify({
    //         refresh_token,
    //         access_token
    //     })
    // });

    // if (!response.ok) {
    //     console.error("couldn't set session cookies");
    // }

    const cookie_store = cookies();
    const now = Date.now();

    cookie_store.set('access_token', access_token, {
        httpOnly: true,
        secure: true,
        sameSite: 'strict',
        expires: now + ACCESS_TOKEN_COOKIE_EXPIRE,
    });

    cookie_store.set('refresh_token', refresh_token, {
        httpOnly: true,
        secure: true,
        sameSite: 'strict',
        expires: now + REFRESH_TOKEN_COOKIE_EXPIRE,
    });
}