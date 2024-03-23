"use server"

import authorizedFetch from "../auth/fetch";

export type WalletBalance = {
    character_id: number,
    balance: number
    updated_at: string,
}

export type BalanceResponse = {
    successful: boolean,
    balance?: WalletBalance,
}

export type UserBalanceResponse = {
    successful: boolean,
    characters: WalletBalance[],
}

export type GetBalanceFn = (character_id: number) => Promise<BalanceResponse>;
export type GetUserBalanceFn = () => Promise<UserBalanceResponse>;

export async function getBalance(character_id: number) {
    const url = `${process.env.ENDPOINT_WALL}/v1/wallet/${character_id}/balance`;

    try {
        const response = await authorizedFetch(url, {
            method: 'GET',
            next: {
                tags: ['wallet']
            }
        }) as BalanceResponse;

        return response;
    } catch (error) {
        console.error(error);
        return { successful: false, balance: undefined };
    }
}

export async function getUserBalance(): Promise<UserBalanceResponse> {
    const url = `${process.env.ENDPOINT_WALL}/v1/wallet/balance`;

    try {
        const response = await authorizedFetch(url, {
            method: 'GET',
            next: {
                tags: ['wallet']
            }
        }) as UserBalanceResponse;
        
        return response;
    } catch (error) {
        console.error(error);
        return { successful: false, characters: [] }
    }
}