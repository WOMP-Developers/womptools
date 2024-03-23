import SideNav from "@/components/dashboard/navigation/SideNav";
import RefreshToken from "../RefreshToken";
import refresh from "@/actions/auth/refresh";

export default function Layout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <RefreshToken refresh={refresh} />
            <div className="flex h-screen flex-col md:flex-row overflow-hidden">
                <div className="flex-none w-full md:w-60 py-2 px-2 md:py-4 md:pl-4 md:pr-2">
                    <SideNav />
                </div>
                <div className="flex-grow md:py-4 pr-2 md:pr-4 pl-2">
                    <div className="flex flex-col bg-black/40 rounded-md p-4 pb-2 h-full gap-2">
                        {children}
                    </div>
                </div>
            </div>
        </>
    )
}