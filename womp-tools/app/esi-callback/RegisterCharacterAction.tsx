'use client'

import ErrorBox from '@/components/ErrorBox';
import { useRouter } from 'next/navigation';
import { useEffect, useState } from 'react';
import AuthorizeLoading from './AuthorizeLoading';

enum LoginState {
    Ready,
    Loading,
    Failed,
    Successful
}

export default function RegisterCharacterAction({ character, code, state }: { 
    character: (code: string, state: string) => Promise<boolean>, code: string, state: string 
}) {
    const [loginState, setLoginState] = useState(LoginState.Ready);

    const router = useRouter();

    useEffect(() => {
        if (loginState === LoginState.Ready) {
            character(code, state).then((success) => {
                setLoginState(success ? LoginState.Successful : LoginState.Failed);
            });

            setLoginState(LoginState.Loading);
        }

        if (loginState === LoginState.Successful) {
            router.replace('/dashboard/characters');
        }
    }, [loginState]);

    return (
        <div className="flex min-h-screen flex-col items-center space-y-10 py-36 px-24">
            { loginState !== LoginState.Failed &&
                <AuthorizeLoading />
            }
            { loginState === LoginState.Failed &&
                <>
                    <ErrorBox message='Failed to authorize, please try again.' />
                    <button className='bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4' onClick={() => router.replace('/dashboard/characters')}>
                        Return to characters
                    </button>
                </>
            }
        </div>
    )
}