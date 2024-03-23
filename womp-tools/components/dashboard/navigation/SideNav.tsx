
import logout from "@/actions/auth/logout";
import NavLinks from "./NavLinks";
import { PowerIcon } from "@heroicons/react/24/outline";
import CharacterCard from "../character/CharacterCard";

export default function SideNav() {
    return (
        <div className="flex h-full flex-col bg-black/40 rounded-md md:space-y-2">
            <CharacterCard />
            
            <div className="flex grow flex-row justify-between space-x-2 md:flex-col md:space-x-0 md:space-y-2 p-2 md:overflow-y-auto">
                <NavLinks />
                <div className="h-auto w-full grow rounded-md bg-black/0"></div>
                <form action={logout}>
                    <button className="flex h-[48px] w-full grow items-center justify-center gap-2 rounded-md bg-gray-800 p-3 text-sm font-medium hover:bg-gray-700 hover:text-blue-300 md:flex-none md:justify-start md:p-2 md:px-3">
                        <PowerIcon className="w-6" />
                        <div className="hidden md:block">Sign Out</div>
                    </button>
                </form>
            </div>
        </div>
    )
}