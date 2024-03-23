"use server"

import authorizedFetch from "../auth/fetch";

export type BalanceSummary = {
    date: string,
    amount?: number
    balance?: number,
}

export type BalanceSummaryResponse = {
    successful: boolean,
    balance_summary: BalanceSummary[],
}

export type GetBalanceSummaryFn = () => Promise<BalanceSummaryResponse>;

export async function getBalanceSummary() {
    const url = `${process.env.ENDPOINT_WALL}/v1/wallet/hist/balance`;

    try {
        const response = await authorizedFetch(url, {
            method: 'GET',
            next: {
                tags: ['wallet']
            }
        }) as BalanceSummaryResponse;

        return response;
    } catch (error) {
        console.error(error);
        return { successful: false, balance_summary: undefined };
    }
}
