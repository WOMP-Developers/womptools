'use server'

import getCharacterMain from "@/actions/character/main";
import revalidateCharacters from "@/actions/character/revalidate";
import LoadingIndicator from "@/components/LoadingIndicator";
import Image from "next/image";

export default async function CharacterCard() {
    const response = await getCharacterMain();

    const revalidate = () => {
        'use server'

        revalidateCharacters();
    }

    return (
        <div className="flex flex-col items-center justify-center gap-2 hidden md:block">
            {(response.successful && response.character) ?
                <>
                    <div className="flex w-full h-48 relative">
                        <Image
                            src={`https://images.evetech.net/characters/${response.character.character_id}/portrait`}
                            alt="Character Portrait"
                            className="shadow-lg object-cover rounded-tr-md rounded-tl-md"
                            sizes="100vw"
                            fill
                            priority />
                        <div className="flex flex-row h-[42px] absolute bottom-2 left-2 gap-1">
                            { response.character.alliance_id && <Image src={`https://images.evetech.net/alliances/${response.character.alliance_id}/logo`}
                                alt="Alliance Logo"
                                width={42}
                                height={42}
                            /> }
                            <Image src={`https://images.evetech.net/corporations/${response.character.corporation_id}/logo`}
                                alt="Corporation Logo"
                                width={42}
                                height={42}
                                sizes="100vw"
                            />
                        </div>
                    </div>
                </> : (
                    <div className="flex h-48 items-center justify-center">
                        <LoadingIndicator type={'grid'} retry={{ timeout_seconds: 1, retry_action: revalidate}} />
                    </div>
                )
            }
        </div>
    )
}
