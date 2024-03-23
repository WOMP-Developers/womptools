"use client"

import { WalletBalance, getUserBalance } from "@/actions/wallet/get_balance"
import { useEffect, useState } from "react"
import CharacterWallet from "./CharacterWallet";
import LoadingIndicator from "@/components/LoadingIndicator";
import { Character } from "@/actions/character/dto";

type CW = {
    character: Character,
    wallet: WalletBalance,
};

export default function CharacterWalletList({ character_map }: { character_map: Map<number, Character> }) {
    const [wallets, setWallets] = useState<{ character: Character, wallet: WalletBalance }[]>();

    useEffect(() => {
        getUserBalance().then(response => {
            const character_wallets = response.characters.sort((a, b) => b.balance - a.balance).map(wallet => {
                const character = character_map.get(wallet.character_id);
                return character ? { character, wallet } : undefined;
            }).filter(cw => cw !== undefined) as CW[];

            setWallets(character_wallets);
        });

    }, [character_map]);

    return (
        wallets ? wallets.map(
            wallet => <CharacterWallet key={wallet.character.character_id} character={wallet.character} wallet={wallet.wallet} />
        ) : <div className="flex flex-col grow items-center"><LoadingIndicator type='pulse' /></div>
    );
}