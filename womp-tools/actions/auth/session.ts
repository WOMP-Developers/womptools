"use server"

import { cookies } from 'next/headers';
import jwt from 'jsonwebtoken';

export type UserSession = {
    userId: number,
}

export default async function getUserSession(): Promise<UserSession | undefined> {
    const accessToken = cookies().get('access_token');

    if (!accessToken?.value) {
        return undefined;
    }

    const decodedToken = jwt.decode(accessToken.value) as {
        sub: string,
    };

    const userId = decodedToken.sub;

    if (!userId) {
        return undefined;
    }

    return {
        userId: parseInt(userId),
    }
}