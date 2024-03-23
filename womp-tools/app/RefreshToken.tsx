'use client'

import getSession, { Session } from "@/actions/auth/session";
import { decode } from "jsonwebtoken";
import { useEffect, useState } from "react";

type MaybeToken = string | undefined;
type MaybeSession = Session | undefined;

const refreshAccessToken = async (refresh: (token: string) => Promise<MaybeSession>) => {
    const session = await getSession();

    if (!session?.refresh_token) {
        return undefined;
    }

    const refreshed_session = await refresh(session.refresh_token);

    return refreshed_session?.access_token;
};

const REFRESH_TIMEFRAME = 60 * 2; // 2 minutes

function secondsUntilExpire(token: string): number {
    const jwt = decode(token) as { exp: number };
    return jwt.exp - (Date.now() / 1000);
}

export default function RefreshToken({ refresh }: { refresh: (token: string) => Promise<MaybeSession>}) {
    const [tokenExpireTime, setTokenExpireTime] = useState(0);

    useEffect(() => {
        const tokenRefreshCallback = (token: MaybeToken) => {
            if (!token) {
                console.error('coult not refresh access token');
                return;
            }

            const expire_in_secs = secondsUntilExpire(token);
            
            setTokenExpireTime(expire_in_secs);
        };

        if (tokenExpireTime === 0) {
            refreshAccessToken(refresh).then(tokenRefreshCallback)
        } else {
            const refresh_in_secs = tokenExpireTime - REFRESH_TIMEFRAME;

            const timeout = setTimeout(() => {
                refreshAccessToken(refresh).then(tokenRefreshCallback);
            }, refresh_in_secs * 1000)

            return () => clearTimeout(timeout);
        }
    }, [tokenExpireTime])

    return null;
}