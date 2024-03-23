import login from '@/actions/auth/login';
import ErrorBox from '@/components/ErrorBox';
import getSession from '@/actions/auth/session';
import character from '@/actions/auth/character';
import AuthorizeAction from './AuthorizeAction';

type SearchParams = { [key: string]: string | string[] | undefined };

export default async function EsiCallback({ searchParams }: { searchParams?: SearchParams }) {
    const code = searchParams?.code;
    const state = searchParams?.state;

    const invalidCodeOrState = (!code || Array.isArray(code)) || (!state || Array.isArray(state));

    if (invalidCodeOrState) {
        return (
            <div className="flex min-h-screen flex-col items-center space-y-10 py-36 px-24">
                <ErrorBox message='Invalid authorization code, please try again.' />
            </div>
        )
    }

    return <AuthorizeAction code={code} state={state} login={login} character={character} getSession={getSession} />
}
