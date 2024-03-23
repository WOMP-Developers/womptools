"use client"

import { Character } from "@/actions/character/dto";
import { WalletBalance as Wallet } from "@/actions/wallet/get_balance";
import Link from "next/link";
import { BanknotesIcon } from "@heroicons/react/24/solid";
import clsx from "clsx";
import Counter from "../utils/Counter";
import moment from "moment";


export default function CharacterWallet({ character, wallet }: { character: Character, wallet: Wallet }) {

    return (
        <Link href={`/dashboard/wallet/${character.character_id}`}>
            <div className="relative flex flex-none h-40 w-72 bg-gray-950/20 backdrop-blur-2xl rounded-md ring-1 ring-black/50 shadow-lg shadow-gray-950/60 hover:shadow-lg hover:shadow-gray-950/90 p-2">
                <div className="flex flex-col flex-grow text-2xl">

                    <div className="flex grow justify-center items-center">
                        <div className={clsx('text-green-400', { 'text-red-400': wallet.balance < 0 })}>
                            <Counter value={wallet.balance} />
                        </div>
                    </div>

                    <div className="w-full flex flex-col px-2 gap-2">
                        <div className="flex flex-col">
                            <div className="text-base text-gray-300">
                                {character.name}
                            </div>

                            <div className="text-xs text-gray-600">
                                updated { moment(wallet.updated_at).fromNow() }
                            </div>
                        </div>
                    </div>
                </div>
                <div className="absolute right-2 top-2">
                    <BanknotesIcon className="w-6 text-gray-600/40" />
                </div>
            </div>
        </Link>
    );
}