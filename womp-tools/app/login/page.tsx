
import { RedirectType, redirect } from 'next/navigation';
import getUserSession from '../../actions/auth/session';
import LoginButton from '@/components/login/loginButton';

export default async function Login() {
    const session = await getUserSession();

    if (session) {
        redirect('/dashboard', RedirectType.replace);
    }

    return (
        <main className="flex min-h-screen flex-col items-center space-y-10 py-36 px-24">
            <h2 className={`mb-3 text-2xl font-semibold`}>
                Welcome to WOMP Tools
            </h2>

            <LoginButton />
        </main>
    )
}
