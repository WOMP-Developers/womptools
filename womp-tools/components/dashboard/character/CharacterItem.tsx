import { Character } from "@/actions/character/dto";
import Image from "next/image";
import { LockClosedIcon, StarIcon, ExclamationTriangleIcon } from "@heroicons/react/24/solid";
import StatusBadge from "./StatusBadge";
import getCorporation from "@/actions/esi/get_corporation";
import getAlliance from "@/actions/esi/get_alliance";
import WalletBalance from "../wallet/WalletBalance";
import { Suspense } from "react";
import CorpTicker from "../tickers/CorpTicker";
import AllianceTicker from "../tickers/AllianceTicker";
import LoadingIndicator from "@/components/LoadingIndicator";
import { getBalance } from "@/actions/wallet/get_balance";

export default async function CharacterItem({ character }: { character: Character }) {

    return (
        <div className="flex h-[48px] items-center gap-3 p-2 bg-black/60 hover:bg-black/90 rounded-md mr-2">
            <div className="flex flex-row items-center gap-3">
                <Image src={`https://images.evetech.net/characters/${character.character_id}/portrait`}
                    alt="Character Portrait"
                    className="h-[32px] w-[32px] rounded-full shadow-lg"
                    width={256}
                    height={256}
                    priority
                />

                <div className="w-44">{character.name}</div>
            </div>

            <div className="flex flex-row justify-around grow gap-3">
                <div className="w-44 hidden xl:block">
                    <Suspense><WalletBalance character_id={character.character_id} get_wallet={getBalance} /></Suspense>
                </div>

                <div className="w-44 hidden xl:block">
                    SP 125,000,000
                </div>

                <div className="w-20 hidden 2xl:block">
                    {character.alliance_id && <Suspense fallback={<LoadingIndicator type="pulse" />}>
                        <AllianceTicker alliance_id={character.alliance_id} get_alliance={getAlliance} />
                    </Suspense>}
                </div>

                <div className="w-28 hidden 2xl:block">
                    <Suspense fallback={<LoadingIndicator type="pulse" />}>
                        <CorpTicker corporation_id={character.corporation_id} get_corporation={getCorporation} />
                    </Suspense>
                </div>
            </div>

            {/* <div className="grow"></div> */}

            <div className="flex flex-row justify-end gap-1 w-52">
                {character.is_main && <StatusBadge variant={'status'}><StarIcon className="w-4" /><div className="hidden md:block truncate">Main</div></StatusBadge>}
                {false && <StatusBadge variant={'warning'}><ExclamationTriangleIcon className="w-4" /><div className="hidden md:block truncate">Skill Queue Inactive</div></StatusBadge>}
                {character.requires_authorization && <StatusBadge variant={'error'}><LockClosedIcon className="w-4" /><div className="hidden md:block truncate">Invalid Token</div></StatusBadge>}
            </div>
        </div>
    )

}