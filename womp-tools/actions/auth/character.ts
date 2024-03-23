'use server'

import { cookies } from 'next/headers';
import authorizedFetch from './fetch';

type CharacterResponse = {
    successful: boolean,
};

const LOGIN_URL = `${process.env.ENDPOINT_AUTH}/v1/users/character`;

async function sendCharacterRequest(authorization_code: string): Promise<CharacterResponse> {
    const res = await authorizedFetch(LOGIN_URL, {
        method: "POST",
        body: JSON.stringify({
            authorization_code,
        }),
        headers: {
            'content-type': "application/json",
        }
    }) as CharacterResponse;

    if (!res.successful) {
        console.error("register character failed")
    }

    return res;
}

export default async function character(authorization_code: string, oauth_state: string): Promise<boolean> {
    const stored_state = cookies().get('oauth_state');

    if (stored_state?.value !== oauth_state) {
        console.warn("stored oauth_state doesn't match oauth_state");
        return false;
    }

    const response = await sendCharacterRequest(authorization_code);

    if (!response.successful) {
        return false;
    }

    return true;
}