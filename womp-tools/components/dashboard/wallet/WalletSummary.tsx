"use client"

import clsx from "clsx";
import Counter from "../utils/Counter";
import IncomeGraph from "./IncomeGraph";
import { useEffect, useState } from "react";
import { BalanceSummary, getBalanceSummary } from "@/actions/wallet/get_balance_summary";
import LoadingIndicator from "@/components/LoadingIndicator";
import BalanceGraph from "./BalanceGraph";
import { getUserBalance } from "@/actions/wallet/get_balance";

export default function WalletSummary() {
    const [totalBalance, setTotalBalance] = useState<number>();
    const [balanceSummary, setBalanceSummary] = useState<BalanceSummary[]>();
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        getUserBalance().then(wallets => {
            setTotalBalance(wallets.characters.reduce((sum, balance) => sum + balance.balance, 0));
        })

        getBalanceSummary().then(balance_summary => {
            setBalanceSummary(balance_summary.balance_summary)
        });
    }, []);

    useEffect(() => {
        setIsLoading(totalBalance === undefined || balanceSummary === undefined);
    }, [totalBalance, balanceSummary]);

    if (isLoading) {
        return (
            <div className="flex flex-col grow">
                <div className="flex flex-row flex-1 justify-center items-center">
                    <LoadingIndicator type="beat" />
                </div>
            </div>
        )
    }

    return (
        <div className="flex flex-col grow">
            <div className="flex flex-row flex-1 justify-center items-center">
                {totalBalance ? <div className={clsx('text-3xl text-green-400', { 'text-red-400': totalBalance < 0 })}>
                    <Counter value={totalBalance} />
                </div> : <LoadingIndicator type="beat" />}
            </div>
            <div className="flex flex-row basis-1/4 flex-1 items-center">
                <div className="flex h-full flex-grow justify-center">
                    {balanceSummary ? <IncomeGraph balance_summary={balanceSummary} /> : <LoadingIndicator type="beat" />}
                </div>
                <div className="flex h-full flex-grow justify-center">
                    {balanceSummary ? <BalanceGraph balance_summary={balanceSummary} /> : <LoadingIndicator type="beat" />}
                </div>
            </div>
        </div>
    )
}