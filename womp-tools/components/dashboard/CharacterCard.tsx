import Image from "next/image";

export default function CharacterCard() {
    return (
        <div className="w-full pb-4 pt-2">
            <div className="flex flex-col items-center">
                <Image
                    src="https://images.evetech.net/characters/1294567802/portrait"
                    alt="Character Portrait"
                    className="h-24 w-24 rounded-full shadow-lg"
                    width={240}
                    height={240}
                    priority />
                <h4 className="mb-1 text-xl font-medium mt-3">
                    Ageliten
                </h4>
                <div className="flex">
                    <Image src="https://images.evetech.net/alliances/434243723/logo"
                        alt="Alliance Logo"
                        width={120}
                        height={120}
                        className="h-10 w-10"
                    />
                    <Image src="https://images.evetech.net/corporations/109299958/logo"
                        alt="Corporation Logo"
                        width={120}
                        height={120}
                        className="h-10 w-10"
                    />
                </div>
            </div>
        </div>
    )
}
