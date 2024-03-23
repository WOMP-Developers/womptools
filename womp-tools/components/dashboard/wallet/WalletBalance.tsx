"use client"

import { GetBalanceFn, WalletBalance as WalletBalanceDTO } from "@/actions/wallet/get_balance"
import LoadingIndicator from "@/components/LoadingIndicator";
import clsx from "clsx";
import Link from "next/link";
import { useEffect, useState } from "react";
import Counter from "../utils/Counter";

export default function WalletBalance({ character_id, get_wallet }: { character_id: number, get_wallet: GetBalanceFn }) {
    const [wallet, setWallet] = useState<WalletBalanceDTO>();

    useEffect(() => {
        const update_wallet = () => {
            get_wallet(character_id).then((wb) => {
                if (wb.successful) {
                    setWallet(wb.balance);
                }
            });
        };

        update_wallet();

        const interval = setInterval(() => {
            update_wallet();
        }, 300 * 1000)

        return () => {
            clearInterval(interval);
        }
    }, [get_wallet, character_id]);

    return (
        <>
            {wallet ? (
                <Link className={clsx('hover:underline underline-offset-4 decoration-1 hover:text-green-300 text-green-400', { 'text-red-400': wallet.balance < 0 })} href={`/dashboard/wallet/${character_id}`}>
                    <Counter value={wallet.balance} />
                </Link>
            ) : <LoadingIndicator type="pulse" />}
        </>
    )
}
