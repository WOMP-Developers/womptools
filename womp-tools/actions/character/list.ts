'use server'

import authorizedFetch from "../auth/fetch";
import { Character } from "./dto";

const URL = `${process.env.ENDPOINT_CHAR}/v1/characters/list`;

export type CharactersResponse = {
    successful: boolean,
    characters: Character[],
}

export default async function getCharacterList() {
    try {
        const response = await authorizedFetch(URL, {
            method: 'GET',
            next: {
                tags: ['characters']
            }
        }) as CharactersResponse;

        return response;

    } catch(error) {
        console.error(error);
        return { successful: false, characters: [] };
    }
}