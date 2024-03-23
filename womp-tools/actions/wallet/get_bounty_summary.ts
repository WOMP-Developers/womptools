"use server"

import authorizedFetch from "../auth/fetch";

export type BountySummary = {
    date: string,
    character_id?: number,
    sum_bounties?: number
    sum_taxes?: number,
}

export type BountySummaryResponse = {
    successful: boolean,
    bounty_summary: BountySummary[],
}

export type GetBountySummaryFn = () => Promise<BountySummaryResponse>;

export async function getBountySummary() {
    const url = `${process.env.ENDPOINT_WALL}/v1/wallet/hist/bounty`;

    try {
        const response = await authorizedFetch(url, {
            method: 'GET',
            next: {
                tags: ['wallet']
            }
        }) as BountySummaryResponse;

        return response;
    } catch (error) {
        console.error(error);
        return { successful: false, bounty_summary: undefined };
    }
}
