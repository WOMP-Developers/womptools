"use client"

import { useEffect, useState } from "react"
import JournalListItem from "./JournalListItem"
import { JournalEntry, getJournal } from "@/actions/wallet/get_journal"
import { useInView } from "react-intersection-observer";
import LoadingIndicator from "@/components/LoadingIndicator";

export default function JournalList({ character_id }: { character_id: number }) {
    const [page, setPage] = useState<number>(0);
    const [pages, setPages] = useState<number>(1);
    const [isLoading, setIsLoading] = useState<boolean>(false);
    const [entries, setEntries] = useState<JournalEntry[]>([]);

    const { ref, inView } = useInView();

    useEffect(() => {
        if (inView && !isLoading && page < pages) {
            setIsLoading(true);

            const fetch_page = page + 1;

            getJournal(character_id, fetch_page).then(journal => {
                setPages(journal.page_count);
                setEntries([...entries, ...journal.entries]);
                setPage(fetch_page);

                setIsLoading(false);
            });
        }

    }, [character_id, inView, isLoading]);

    return (
        <div className="flex flex-col gap-1 mr-2">
            <div className="bg-black/60 h-[2px]"></div>
            <div className="flex flex-row text-sm p-2">
                <div className="w-1/4">
                    Date
                </div>

                <div className="w-1/4">
                    Amount
                </div>

                <div>
                    Tax
                </div>

                <div className="grow" />

                <div className="flex justify-end gap-1 w-52">
                    Type
                </div>
            </div>
            <div className="bg-black/60 h-[2px] rounding-sm mb-2"></div>

            {entries.map(entry => (<JournalListItem key={entry.id} entry={entry} />))}
            {page < pages && <div ref={ref}><LoadingIndicator type="pulse" /></div>}
        </div>
    )
}