import getCharacterList from "@/actions/character/list"

export default async function Characters() {
    const characters = await getCharacterList();

    return (
        <p>Characters {characters}</p>
    )
}