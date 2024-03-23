'use sever'

import getCharacterList from "@/actions/character/list";
import LoadingIndicator from "@/components/LoadingIndicator";
import CharacterItem from "./CharacterItem";
import { Suspense } from "react";
import CharacterItemLoading from "./CharacterItemLoading";

export default async function CharacterList() {
    const characters = await getCharacterList();

    if (!characters.characters) {
        return (
            <div className="flex flex-col grow place-content-around">
                <div className="flex flex-row place-content-around">
                    <LoadingIndicator type={'beat'}/>
                </div>
            </div>
        )
    }

    return (
        <>
            {characters.characters.sort((a, b) => a.is_main ? -1 : 1).map((character) => (
                <Suspense key={character.character_id} fallback={<CharacterItemLoading />}><CharacterItem key={character.character_id} character={character}/></Suspense>
            ))}
        </>
    )
}