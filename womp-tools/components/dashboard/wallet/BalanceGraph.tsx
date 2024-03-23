"use client"

import { BalanceSummary } from "@/actions/wallet/get_balance_summary";
import moment from "moment";
import { Area, AreaChart, CartesianGrid, ResponsiveContainer, XAxis } from "recharts";

export default function BalanceGraph({
    balance_summary,
}: { balance_summary: BalanceSummary[] }) {
    return (
        <ResponsiveContainer>
            <AreaChart width={500} height={300} margin={{ left: 8, right: 8, top: 8, bottom: 8 }} data={balance_summary}>
                <CartesianGrid stroke="#000" strokeLinecap={"square"} />
                <XAxis dataKey={"date"} tickFormatter={(value, _index) => moment(value).format("MM/DD")} />
                <Area connectNulls type="monotone" dataKey="balance" stroke="#518ec1" fill="#3a6589" fillOpacity={0.9} />
            </AreaChart>
        </ResponsiveContainer>
    )
}