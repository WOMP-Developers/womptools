import { GetAllianceFn } from "@/actions/esi/get_alliance";
import Link from "next/link";

export default async function AllianceTicker({ alliance_id, get_alliance }: { alliance_id: number, get_alliance: GetAllianceFn }) {
    const alliance = await get_alliance(alliance_id);
    const alliance_link = alliance ? `https://evemaps.dotlan.net/alliance/${alliance.name.replace(' ', '_')}` : undefined;

    return (
        alliance && <Link href={alliance_link!} target={'_blank'} className="hover:underline underline-offset-4 decoration-1 hover:text-blue-300">{`[${alliance.ticker}]`}</Link>
    );
}