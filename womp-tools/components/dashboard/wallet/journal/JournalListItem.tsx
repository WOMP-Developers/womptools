"use client"

import { JournalEntry } from "@/actions/wallet/get_journal";
import formatIsk from "@/utils/formatIsk";
import formatJournalType from "@/utils/formatJournalType";
import clsx from "clsx";
import moment from "moment";

export default function JournalListItem({ entry }: { entry: JournalEntry }) {

    const className = clsx(
        'flex flex-row items-center justify-between rounded-sm py-1 px-2 backdrop-blur-md', {
            'bg-gray-600/20': (entry.amount === undefined || entry.amount === 0),
            'bg-red-400/30': (entry.amount && entry.amount < 0),
            'bg-green-400/30': (entry.amount && entry.amount > 0),
        });

    return (
        <div className={className}>
            <div className="w-1/4 font-['Mono']">{moment(entry.date).format('L HH:mm:ss')}</div>
            <div className="w-1/4">{formatIsk(entry.amount)}</div>
            <div>{formatIsk(entry.tax)}</div>
            <div className="grow" />
            <div>{formatJournalType(entry.ref_type)}</div>
        </div>
    );
}