'use client'

import ErrorBox from "@/components/ErrorBox";
import { ArrowPathIcon } from "@heroicons/react/24/solid";
import { useEffect } from "react"

export default function Error({ error, reset }: {
    error: Error & { digest?: string },
    reset: () => void,
}) {
    useEffect(() => {
        console.error(error);
    }, [error]);

    return (
        <div className="flex flex-col h-full items-center justify-center gap-2">
            <ErrorBox message={error.message} />
            {/* <form action={reset}>
                <button className="rounded-md p-3 text-sm font-small bg-black/60 hover:bg-black/90">
                    <ArrowPathIcon className="w-6" />
                </button>
            </form> */}
        </div>
    )
}