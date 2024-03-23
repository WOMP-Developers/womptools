'use server'

import authorizedFetch from './fetch';
import { RedirectType, redirect } from 'next/navigation';
import { clearSessionCookies } from './cookies';

const LOGOUT_URL = `${process.env.ENDPOINT_AUTH}/v1/users/logout`;

type LogoutResponse = {
    successful: boolean
}

export default async function logout() {
    const response = await authorizedFetch(LOGOUT_URL, {
        method: 'POST'
    }) as LogoutResponse;

    if (!response.successful) {
        console.error("logout unsuccessful");
    }

    await clearSessionCookies();

    redirect('/', RedirectType.replace);
}