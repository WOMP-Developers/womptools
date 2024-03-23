'use server'

import { cookies, headers } from 'next/headers';
import { setSessionCookies } from './cookies';

type LoginResponse = {
    successful: boolean,
    access_token: string,
    refresh_token: string,
};

const LOGIN_URL = `${process.env.ENDPOINT_AUTH}/v1/users/login`;

async function sendLoginRequest(authorization_code: string, client_ip: string): Promise<LoginResponse> {
    const res = await fetch(LOGIN_URL, {
        method: "POST",
        body: JSON.stringify({
            authorization_code,
            client_ip
        }),
        headers: {
            "content-type": "application/json",
        }
    });

    if (!res.ok) {
        console.error("login failed", res.status, res.statusText)
    }

    return res.json()
}

export default async function login(authorization_code: string, oauth_state: string): Promise<boolean> {
    const cookie_store = cookies();

    const stored_state = cookie_store.get('oauth_state');

    if (stored_state?.value !== oauth_state) {
        console.warn("stored oauth_state doesn't match oauth_state");
        return false;
    }

    const client_ip = headers().get('x-forwarded-for')!;

    const response = await sendLoginRequest(authorization_code, client_ip);

    if (!response.successful) {
        return false;
    }

    await setSessionCookies(response.access_token, response.refresh_token);

    return true;
}
