'use server'

import { cookies } from 'next/headers';
import { RedirectType, redirect } from 'next/navigation';
import { v4 as uuid } from 'uuid';
import queryString from 'query-string';

const ESI_LOGIN_ENDPOINT = "https://login.eveonline.com/v2/oauth/authorize/";
const ESI_SCOPE = "publicData esi-wallet.read_character_wallet.v1 esi-search.search_structures.v1 esi-universe.read_structures.v1 esi-assets.read_assets.v1 esi-markets.structure_markets.v1 esi-corporations.read_structures.v1 esi-industry.read_character_jobs.v1 esi-markets.read_character_orders.v1 esi-characters.read_blueprints.v1 esi-contracts.read_character_contracts.v1 esi-wallet.read_corporation_wallets.v1 esi-assets.read_corporation_assets.v1 esi-corporations.read_blueprints.v1 esi-contracts.read_corporation_contracts.v1 esi-corporations.read_starbases.v1 esi-industry.read_corporation_jobs.v1 esi-markets.read_corporation_orders.v1 esi-corporations.read_facilities.v1";

export default async function loginEve() {
    const redirectUri = encodeURI(process.env.ESI_REDIRECT_URI as string);
    const clientId = process.env.ESI_CLIENT_ID as string;

    const state = uuid();

    const parameters = {
        response_type: 'code',
        redirect_uri: redirectUri,
        client_id: clientId,
        scope: ESI_SCOPE,
        state,
    };

    const query = queryString.stringify(parameters);
    const uri = `${ESI_LOGIN_ENDPOINT}?${query}`;

    const now = Date.now();

    cookies().set('oauth_state', state, {
        httpOnly: true,
        sameSite: 'strict',
        expires: now + 60 * 15 * 1000, // now + 15 minutes
    });

    redirect(uri, RedirectType.replace);
}