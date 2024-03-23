"use server"

import authorizedFetch from "../auth/fetch";
import { Character } from "./dto";

export type CharacterResponse = {
    successful: boolean,
    character?: Character,
}

export default async function getCharacterById(character_id: number) {
    const URL = `${process.env.ENDPOINT_CHAR}/v1/characters/${character_id}`;

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