'use server'

import { cookies } from 'next/headers';

export type Session = {
    access_token: string | undefined,
    refresh_token: string,
}

export default async function getSession(): Promise<Session | undefined> {
    const cookie_store = cookies();

    const access_token = cookie_store.get('access_token')?.value;
    const refresh_token = cookie_store.get('refresh_token')?.value;

    if (!refresh_token) {
        return undefined;
    }

    return {
        access_token,
        refresh_token,
    }
}