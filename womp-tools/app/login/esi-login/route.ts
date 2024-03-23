import { cookies } from 'next/headers';
import { NextRequest } from 'next/server';
import { RedirectType, redirect } from 'next/navigation';

type LoginResponse = {
    successful: boolean,
    access_token: string,
    refresh_token: string,
};

// TODO: configuration variable for auth endpoint
const AUTH_ENDPOINT = "http://localhost:8080/v1/auth/login";

async function login_request(code: string): Promise<LoginResponse> {
    const res = await fetch(AUTH_ENDPOINT, {
        method: "POST",
        body: JSON.stringify({
            authorization_code: code,
        }),
        headers: {
            "content-type": "application/json",
        }
    });

    if (!res.ok) {
        console.error("login failed", res.status, res.statusText)
    }

    return res.json()
}

export async function GET(request: NextRequest) {
    const cookieStore = cookies();

    const { searchParams } = new URL(request.url);
    const authorizationCode = searchParams.get('code');
    const state = searchParams.get('state');

    if (!authorizationCode) {
        console.error("invalid authorization code");
        redirect("/login", RedirectType.replace);
    }

    const storedEsiState = cookieStore.get('oauth_state');

    if (!state || Array.isArray(state) || storedEsiState?.value !== state) {
        // TODO: For some reason this endpoint is called twice in a row, causing
        // this path to hit unintentionaly. Find out why it's called twice it should
        // only happen one time.
        redirect("/login", RedirectType.replace);
    }

    cookieStore.delete('oauth_state');

    const session = await login_request(authorizationCode);

    if (!session.successful) {
        console.error("failed login");
        redirect("/login", RedirectType.replace);
    }

    cookieStore.set('access_token', session.access_token, {
        httpOnly: true,
        sameSite: 'strict',
    });

    cookieStore.set('refresh_token', session.refresh_token, {
        httpOnly: true,
        sameSite: 'strict',
    });

    console.log("successful login");

    redirect("/login", RedirectType.replace);
}