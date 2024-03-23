import loginEve from "@/actions/auth/login_eve";
import revalidateCharacters from "@/actions/character/revalidate";
import CharacterList from "@/components/dashboard/character/CharacterList";
import { ArrowPathIcon, UserPlusIcon } from "@heroicons/react/24/solid";
import { Suspense } from "react";

export default async function Characters() {

    return (
        <>
            <div className="flex flex-row gap-2">
                <div className="mb-1 text-xl font-medium mt-3">
                    Registered Characters
                </div>
                <div className="grow"></div>
                <form action={revalidateCharacters}>
                    <button className="rounded-md p-3 text-sm font-small bg-black/60 hover:bg-black/90">
                        <ArrowPathIcon className="w-6" />
                    </button>
                </form>
                <form action={loginEve}>
                    <button className="rounded-md p-3 text-sm font-small bg-black/60 hover:bg-black/90">
                        <UserPlusIcon className="w-6" />
                    </button>
                </form>
            </div>

            <div className="flex flex-grow flex-col gap-2 overflow-y-scroll">
                <div className="flex flex-grow flex-col">
                    <div className="bg-black/60 h-[2px] mr-4"></div>
                    <div className="flex flex-row text-sm gap-3 p-2 mr-4">
                        <div className="w-[32px]"></div>

                        <div className="w-44">
                            Character
                        </div>

                        <div className="flex flex-row justify-around grow gap-3">
                            <div className="w-44 hidden xl:block">
                                Wallet
                            </div>

                            <div className="w-44 hidden xl:block">
                                Skillpoints
                            </div>

                            <div className="w-20 hidden 2xl:block">
                                Alliance
                            </div>

                            <div className="w-28 hidden 2xl:block">
                                Corporation
                            </div>

                        </div>

                        <div className="flex justify-end gap-1 w-52">
                            Status
                        </div>
                    </div>
                    <div className="bg-black/60 h-[2px] rounding-sm mb-2 mr-4"></div>
                    <div className="flex flex-col h-full overflow-y-scroll gap-2">
                        <Suspense>
                            <CharacterList />
                        </Suspense>
                    </div>
                </div>
            </div>
        </>
    )
}