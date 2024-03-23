import { RedirectType, redirect } from 'next/navigation';
import getSession from '@/actions/auth/session';
import loginEve from '@/actions/auth/login_eve';

export default async function Login() {
  const session = await getSession();

  if (session) {
    redirect('/dashboard', RedirectType.replace);
  }

  return (
    <main className="flex min-h-screen flex-col items-center space-y-10 py-36 px-24">
      <h2 className={`mb-3 text-2xl font-semibold`}>
        Welcome to WOMP Tools
      </h2>

      <form action={loginEve}>
        <button className='bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4'>
          Login using ESI
        </button>
      </form>
    </main>
  )
}
