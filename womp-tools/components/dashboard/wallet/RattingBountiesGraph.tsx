"use client"

import { Character } from "@/actions/character/dto";
import { getBountySummary } from "@/actions/wallet/get_bounty_summary";
import LoadingIndicator from "@/components/LoadingIndicator";
import moment from "moment";
import { useEffect, useState } from "react";
import { Bar, BarChart, CartesianGrid, ResponsiveContainer, Tooltip, XAxis, YAxis } from "recharts";

type Payload = {
    payload: Data,
};

type Data = {
    date: string,
    sum_bounties: number,
    sum_taxes: number,
    characters: {
        name: string,
        sum_bounties: number,
        sum_taxes: number,
    }[],
}

function CustomTooltip({
    payload, label, active
}: { payload?: Payload[], label?: string, active?: boolean }) {
    if (!active) {
        return null;
    }

    const data = payload![0].payload;

    return (
        <div className="flex flex-col rounded-md bg-black/90 ring-1 ring-gray-900 p-2">
            {data && (
                <>
                    <div className="flex flex-row grow gap-4 text-sm justify-between text-gray-400">
                        <div className="flex font-bold">Total (Tax)</div>
                        <div className="flex font-bold">{data.sum_bounties.toLocaleString('en-US', { maximumFractionDigits: 0 })} ({data.sum_taxes.toLocaleString('en-US', { maximumFractionDigits: 0 })})</div>
                    </div>
                    {data.characters?.map(ch => (<div className="flex flex-row grow gap-4 text-sm justify-between text-gray-400">
                        <div className="flex font-normal">{ch.name}</div>
                        <div className="flex font-semibold">{ch.sum_bounties.toLocaleString('en-US', { maximumFractionDigits: 0 })}  ({ch.sum_taxes.toLocaleString('en-US', { maximumFractionDigits: 0 })})</div>
                    </div>))}
                </>
            )}
        </div>
    );
}

export default function RattingBountiesGraph({
    character_map
}: { character_map: Map<number, Character> }) {
    const [graphData, setGraphData] = useState<Array<Data>>();

    useEffect(() => {
        getBountySummary().then(({ bounty_summary }) => {
            const data_map = new Map<string, Data>();

            const start_at = moment().utc().startOf('day').subtract(1, 'month');

            // TODO: improve this jank below

            bounty_summary?.filter(
                summary => moment(summary.date) >= start_at
            ).forEach(summary => {
                const character_name = summary.character_id ? character_map.get(summary.character_id)?.name : undefined;

                const entry = data_map.get(summary.date);

                if (!entry) {
                    const new_entry = {
                        date: summary.date,
                        sum_bounties: summary.sum_bounties ?? 0,
                        sum_taxes: summary.sum_taxes ?? 0,
                        characters: [] as { name: string, sum_bounties: number, sum_taxes: number }[],
                    };

                    if (character_name) {
                        new_entry.characters.push({
                            name: character_name,
                            sum_bounties: summary.sum_bounties ?? 0,
                            sum_taxes: summary.sum_taxes ?? 0,
                        });
                    }

                    data_map.set(new_entry.date, new_entry);
                } else if (character_name) {
                    entry.sum_bounties += summary.sum_bounties ?? 0;
                    entry.sum_taxes += summary.sum_taxes ?? 0;

                    entry.characters.push({
                        name: character_name,
                        sum_bounties: summary.sum_bounties ?? 0,
                        sum_taxes: summary.sum_taxes ?? 0,
                    });
                }
            });

            return Array.from(data_map.values());
        }).then((data: Array<Data>) => {
            setGraphData(data);
        })
    }, [character_map]);

    if (!graphData) {
        return (
            <div className="flex flex-col grow justify-center items-center">
                <LoadingIndicator type='beat' />
            </div>
        )
    }

    return (
        <ResponsiveContainer width="100%" height="100%">
            <BarChart width={500} margin={{ left: 74, right: 0, top: 8 }} height={300} data={graphData}>
                <CartesianGrid stroke="#000" strokeLinecap={"square"} />
                <XAxis dataKey={"date"} tickFormatter={(value, _index) => moment(value).format("MM/DD")} />
                <YAxis tickFormatter={(value, _index) => value.toLocaleString('en-US', { maximumFractionDigits: 0 })} />
                <Tooltip content={<CustomTooltip />} cursor={false} />
                <Bar dataKey={"sum_bounties"} stackId={"a"} fill={"#3a6589"} />
                <Bar dataKey={"sum_taxes"} stackId={"a"} fill={"#895E3A"} />
            </BarChart>
        </ResponsiveContainer>
    )
}