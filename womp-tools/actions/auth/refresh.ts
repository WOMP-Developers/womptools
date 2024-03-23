'use server'

import { clearSessionCookies, setSessionCookies } from './cookies';
import { Session } from './session';

const REFRESH_URL = `${process.env.ENDPOINT_AUTH}/v1/users/refresh`;

export default async function refresh(refresh_token: string): Promise<Session | undefined> {
    const res = await fetch(REFRESH_URL, {
        method: 'POST',
        body: JSON.stringify({
            refresh_token
        }),
        headers: {
            "content-type": "application/json",
        }
    });

    const response = await res.json();

    if (!response.successful) {
        console.log('session refresh failed');
        await clearSessionCookies();
        return undefined;
    }

    await setSessionCookies(response.access_token, response.refresh_token);

    return {
        access_token: response.access_token,
        refresh_token: response.refresh_token,
    };
}