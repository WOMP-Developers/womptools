import { RedirectType, redirect } from "next/navigation";
import getUserSession from "../../actions/auth/session";

export default async function Dashboard() {

    const session = await getUserSession();

    if (!session) {
        redirect('/login', RedirectType.replace);
    }

    return (
        <p>User Dashboard</p>
    )
}