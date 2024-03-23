"use client"

import getCharacterById from "@/actions/character/get_character";
import Link from "next/link";
import { usePathname } from "next/navigation"
import { useEffect, useState } from "react";

export default function WalletNavigation() {
    const pathname = usePathname();

    const [characterId, setCharacterId] = useState<number>();
    const [characterName, setCharacterName] = useState<string>();

    useEffect(() => {
        let [_empty, _dashboard, _wallet, character_id] = pathname.split('/');

        if (character_id) {
            let id = Number.parseInt(character_id);
            setCharacterId(id);

            getCharacterById(id).then(response => {
                setCharacterName(response.character?.name);
            });
        } else {
            setCharacterId(undefined);
            setCharacterName(undefined);
        }
    }, [pathname])


    return (
        <>
            { characterId !== undefined ? <Link href={"/dashboard/wallet"} className="hover:underline underline-offset-4 decoration-1">Wallet</Link> : <span>Wallet</span> }
            { characterName !== undefined && <span> / { characterName } </span> }
        </>
    )
}