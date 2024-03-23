'use server'

import { Corporation } from "./dto";

export type GetCorporationFn = (corporation_id: number) => Promise<Corporation | undefined>;

export default async function getCorporation(corporation_id: number) {
    const URL = `https://esi.evetech.net/latest/corporations/${corporation_id}/?datasource=tranquility`;

    const cache_key = `corporation-${corporation_id}`;

    try {
        const response = await fetch(URL, {
            method: 'GET',
            next: {
                tags: ['corporations', cache_key]
            }
        });

        const corporation = await response.json() as Corporation;

        return corporation.error ? undefined : corporation;
    } catch(error) {
        throw new Error("Could not fetch corporation. Please try again later.")
    }
}
