"use server"

import authorizedFetch from "../auth/fetch";

export type JournalEntry = {
    id: number,
    character_id: number,
    date: string,
    description: string,
    ref_type: string,
    amount?: number,
    tax?: number,
}

export type JournalResponse = {
    successful: boolean,
    page_count: number,
    entries: JournalEntry[],
}

export async function getJournal(character_id: number, page: number) {
    const url = `${process.env.ENDPOINT_WALL}/v1/wallet/${character_id}/journal/${page}`;

    try {
        const response = await authorizedFetch(url, {
            method: 'GET',
            next: {
                tags: ['wallet']
            }
        }) as JournalResponse;

        return response;
    } catch (error) {
        console.error(error);
        return { successful: false, page_count: 0, entries: [] };
    }
}
