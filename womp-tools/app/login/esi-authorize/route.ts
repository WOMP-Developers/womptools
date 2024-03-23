import { cookies } from 'next/headers';
import { RedirectType, redirect } from 'next/navigation';
import { v4 as uuid } from 'uuid';
import queryString from 'query-string';

const ESI_LOGIN_ENDPOINT = "https://login.eveonline.com/v2/oauth/authorize/";

export function GET() {
    const redirectUri = encodeURI(process.env.ESI_REDIRECT_URI as string);
    const scope = (process.env.ESI_SCOPE as string).replaceAll(',', ' ');
    const clientId = process.env.ESI_CLIENT_ID as string;

    const state = uuid();

    const parameters = {
        response_type: 'code',
        redirect_uri: redirectUri,
        client_id: clientId,
        scope,
        state,
    };

    const query = queryString.stringify(parameters);
    const uri = `${ESI_LOGIN_ENDPOINT}?${query}`;

    cookies().set('oauth_state', state, {
        httpOnly: true,
        sameSite: 'lax',
    });

    redirect(uri, RedirectType.replace);
}
