import { headers, cookies } from "next/headers";

const CHARACTER_LIST_URL = "http://localhost:8081/v1/character/list";

export default async function getCharacterList() {
    const access_token = cookies().get('access_token');

    const res = await fetch(CHARACTER_LIST_URL, {
        headers: { Authorization: `Bearer ${access_token?.value}`}
    });

    return await res.text();
}