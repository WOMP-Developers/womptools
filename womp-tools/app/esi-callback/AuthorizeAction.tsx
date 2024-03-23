'use client'

import { Session } from "@/actions/auth/session";
import { useEffect, useState } from "react";
import RegisterCharacterAction from "./RegisterCharacterAction";
import LoginAction from "./LoginAction";
import AuthorizeLoading from "./AuthorizeLoading";

export default function AuthorizeAction({ code, state, getSession, login, character }: {
    code: string,
    state: string,
    getSession: () => Promise<Session | undefined>,
    login: (authorization_code: string, oauth_state: string) => Promise<boolean>,
    character: (authorization_code: string, oauth_state: string) => Promise<boolean>
}) {
    const [isReady, setIsReady] = useState<boolean>(false);
    const [session, setSession] = useState<Session>();

    useEffect(() => {
        getSession().then(session => {
            setSession(session)
            setIsReady(true);
        });
    }, []);

    if (!isReady) {
        return (
            <div className="flex min-h-screen flex-col items-center space-y-10 py-36 px-24">
                { isReady === false &&
                    <AuthorizeLoading />
                }
            </div>
        )
    }

    if (session) {
        return <RegisterCharacterAction character={character} code={code} state={state} />
    } else {
        return <LoginAction login={login} code={code} state={state} />
    }
}