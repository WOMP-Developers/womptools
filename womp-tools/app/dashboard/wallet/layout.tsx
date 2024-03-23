import revalidateWallet from "@/actions/wallet/revalidate";
import WalletNavigation from "@/components/dashboard/navigation/Wallet/WalletNavigation";
import { ArrowPathIcon } from "@heroicons/react/24/solid";

export default async function WalletLayout({ children }: { children: React.ReactNode }) {
    return (
        <>
            <div className="flex flex-row gap-2">
                <div className="mb-1 text-xl font-medium mt-3">
                    <WalletNavigation />
                </div>
                <div className="grow"></div>
                <form action={revalidateWallet}>
                    <button className="rounded-md p-3 text-sm font-small bg-black/60 hover:bg-black/90">
                        <ArrowPathIcon className="w-6" />
                    </button>
                </form>
            </div>

            <div className="flex overflow-y-scroll justify-around">
                { children }
            </div>
        </>
    )
}