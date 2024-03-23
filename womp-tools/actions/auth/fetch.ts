'use server'

import getSession from "./session";
import { decode } from "jsonwebtoken";

export default async function authorizedFetch(path: string, init: RequestInit | undefined) {
    const session = await getSession();

    if (!session) {
        return { message: "unauthorized", statusCode: 401 }
    }

    const fetchRequest = makeFetch(path, init);

    // TODO: validate access token and refresh token
    // TODO: preemtively refresh when token is about to expire
    // TODO: refresh if request failed due to expired token

    // if (!session.access_token || isTokenExpired(session.access_token)) {
    //     const response = await refresh(session.refresh_token);

    //     if (response) {
    //         return await fetchRequest(response.access_token);
    //     }

    //     return { message: "unauthorized", statusCode: 401 }
    // }

    return await fetchRequest(session.access_token);
}

function isTokenExpired(token: string): boolean {
    const jwt = decode(token) as { exp: number };
    return jwt.exp < Date.now() / 1000;
}

function makeFetch<T>(path: string, init: RequestInit | undefined): (access_token?: string) => Promise<T> {
    return async function (access_token?: string) {

        const headers = {
            Authorization: `Bearer ${access_token}`,
            ...init?.headers,
        };

        return fetch(path, {
            ...init,
            headers,
            
        }).then((res) => res.json());
    };
}