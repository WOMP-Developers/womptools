"use client"

import { BalanceSummary } from "@/actions/wallet/get_balance_summary";
import moment from "moment";
import { Area, AreaChart, CartesianGrid, ResponsiveContainer, XAxis } from "recharts";



export default function IncomeGraph({ balance_summary }: { balance_summary: BalanceSummary[] }) {

    const gradientOffset = () => {
        const dataMax = Math.max(...balance_summary.map((i) => i.amount ?? 0));
        const dataMin = Math.min(...balance_summary.map((i) => i.amount ?? 0));

        if (dataMax <= 0) {
            return 0;
        }
        if (dataMin >= 0) {
            return 1;
        }

        return dataMax / (dataMax - dataMin);
    };

    const off = gradientOffset();

    return (
        <ResponsiveContainer width="100%" height="100%">
            <AreaChart width={500} margin={{ left: 8, right: 8, top: 8, bottom: 8 }} height={300} data={balance_summary}>
                <CartesianGrid stroke="#000" strokeLinecap={"square"} />
                <XAxis dataKey="date" tickFormatter={(value, _index) => moment(value).format("MM/DD")} />
                <defs>
                    <linearGradient id="splitColor" x1="0" y1="0" x2="0" y2="1">
                        <stop offset={off} stopColor="green" stopOpacity={1} />
                        <stop offset={off} stopColor="red" stopOpacity={1} />
                    </linearGradient>
                </defs>
                <Area type="monotone" dataKey="amount" stroke="#000" fill="url(#splitColor)" />
            </AreaChart>
        </ResponsiveContainer>
    )
}