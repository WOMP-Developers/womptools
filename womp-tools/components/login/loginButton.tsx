"use client"

import { useRouter } from 'next/navigation';

export default function LoginButton() {
    const router = useRouter();

    return (
        <button className='bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4' onClick={() => router.push('/login/esi-authorize')}>
            Login using ESI
        </button>
    )
}

