import { RedirectType, redirect } from "next/navigation";

type SearchParams = { [key: string]: string | string[] | undefined };

export default function EsiCallback({ searchParams }: { searchParams?: SearchParams}) {
    const code = searchParams?.code;
    const state = searchParams?.state;

    if (!code || Array.isArray(code)) {
        console.error("invalid authorization code");
        redirect("/login", RedirectType.replace);
    }

    if (!state || Array.isArray(state)) {
        console.error("invalid state parameter");
        redirect("/login", RedirectType.replace);
    }

    const uri = `/login/esi-login?code=${code}&state=${state}`;

    redirect(uri, RedirectType.replace);
}
