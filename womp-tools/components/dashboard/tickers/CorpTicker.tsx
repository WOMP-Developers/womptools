import { GetCorporationFn } from "@/actions/esi/get_corporation";
import Link from "next/link";

export default async function GetCorporation({corporation_id, get_corporation}: { corporation_id: number, get_corporation: GetCorporationFn}) {
    const corporation = await get_corporation(corporation_id);
    const corporation_link = corporation ? `https://evemaps.dotlan.net/corp/${corporation.name.replace(' ', '_')}` : undefined;
    
    return (
        corporation && <Link href={corporation_link!} target={'_blank'} className="hover:underline underline-offset-4 decoration-1 hover:text-blue-300">{`[${corporation.ticker}]`}</Link>
    )
}