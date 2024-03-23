import React from "react";

export default async function StatusBadge({ children, variant }: { children: React.ReactNode, variant: 'status' | 'warning' | 'error' }) {
    return (
    <>
        {variant === 'status' && <div className="flex gap-1 justify-center items-center border-2 rounded-sm border-solid border-green-600 px-1 md:px-2 py-1 bg-green-800 text-sm small-caps">{children}</div>}
        {variant === 'warning' && <div className="flex gap-1 justify-center items-center border-2 rounded-sm border-dashed border-yellow-600 px-1 md:px-2 py-1 bg-yellow-800 text-sm small-caps">{children}</div>}
        {variant === 'error' && <div className="flex gap-1 justify-center items-center border-2 rounded-sm border-dashed border-red-600 px-1 md:px-2 py-1 bg-red-800 text-sm small-caps">{children}</div>}
    </>
    )
}