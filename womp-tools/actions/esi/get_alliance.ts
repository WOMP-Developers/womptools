'use server'

import { Alliance } from "./dto";

export type GetAllianceFn = (alliance_id: number) => Promise<Alliance | undefined>;

export default async function getAlliance(alliance_id: number) {
    const URL = `https://esi.evetech.net/latest/alliances/${alliance_id}/?datasource=tranquility`;

    const cache_key = `alliance-${alliance_id}`;

    try {
        const response = await fetch(URL, {
            method: 'GET',
            next: {
                tags: ['alliances', cache_key]
            }
        });

        const alliance = await response.json() as Alliance;

        return alliance.error ? undefined : alliance;
    } catch(error) {
        throw new Error("Could not fetch alliance. Please try again later.")
    }
}
