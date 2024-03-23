'use client'

import { useEffect } from "react"
import { BeatLoader, GridLoader, CircleLoader, PulseLoader } from "react-spinners"

type TimeoutRetry = {
    timeout_seconds: number,
    retry_action: () => void,
}

export default function LoadingIndicator({ type, retry }: { type: 'beat' | 'circle' | 'grid' | 'pulse', retry?: TimeoutRetry }) {
    useEffect(() => {
        if (retry) {
            const t = setTimeout(() => retry.retry_action(), retry.timeout_seconds * 1000);

            return () => clearTimeout(t);
        }
    }, [])

    return (
        <>
            {type === 'beat' && <BeatLoader color="#2980b9" />}
            {type === 'circle' && <CircleLoader color="#2980b9" />}
            {type === 'grid' && <GridLoader color="#2980b9" />}
            {type === 'pulse' && <PulseLoader size={10} color="#2980b9" />}
        </>
    )
}