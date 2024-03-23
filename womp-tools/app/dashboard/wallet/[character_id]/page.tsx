import JournalList from "@/components/dashboard/wallet/journal/JournalList";

export default async function CharacterJournal({ params }: { params: { character_id: number } }) {

    return (
        <div className="flex flex-col grow">
            <JournalList character_id={params.character_id} />
        </div>
    )
}