import { Character } from "@/actions/character/dto";
import getCharacterList from "@/actions/character/list";
import revalidateWallet from "@/actions/wallet/revalidate";
import ErrorBox from "@/components/ErrorBox";
import ContentContainer from "@/components/dashboard/utils/ContentContainer";
import CharacterWalletList from "@/components/dashboard/wallet/CharacterWalletList";
import RattingBountiesGraph from "@/components/dashboard/wallet/RattingBountiesGraph";
import WalletSummary from "@/components/dashboard/wallet/WalletSummary";
import { ArrowPathIcon } from "@heroicons/react/24/solid";

export default async function Wallet() {
    const characters = await getCharacterList();

    if (!characters.successful) {
        return (
            <div className="flex flex-col grow justify-center items-center">
                <ErrorBox message="Could not fetch wallet data" />
            </div>
        )
    }

    const character_lookup = new Map<number, Character>();

    characters.characters.forEach(character => {
        character_lookup.set(character.character_id, character);
    });

    return (
        <div className="flex flex-col gap-4 mr-4 grow">
            <div className="flex flex-row grow gap-2">
                <ContentContainer title="Summary">
                    <WalletSummary />
                </ContentContainer>
                <ContentContainer title="Ratting Bounties">
                    <RattingBountiesGraph character_map={character_lookup} />
                </ContentContainer>
            </div>
            <div className="flex flex-col gap-2">
                <div className="px-2">
                    Character Wallets
                </div>
                <div className="flex flex-wrap gap-4">
                    <CharacterWalletList character_map={character_lookup} />
                </div>
            </div>
        </div>
    );
}