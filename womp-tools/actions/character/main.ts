"use server"

import authorizedFetch from "../auth/fetch";
import { Character } from "./dto";

const URL = `${process.env.ENDPOINT_CHAR}/v1/characters/main`;

export type CharacterResponse = {
    successful: boolean,
    character?: Character,
}

export default async function getCharacterMain() {
    try {
        const response = await authorizedFetch(URL, {
            method: 'GET',
            next: {
                tags: ['characters']
            }
        }) as CharacterResponse;

        return { successful: true, character: response?.character };
    } catch (error) {
        return { successful: false, character: undefined };
    }
}