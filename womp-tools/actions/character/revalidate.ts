'use server'

import { revalidatePath, revalidateTag } from "next/cache";

export default async function revalidateCharacters() {
    revalidateTag('characters');
    revalidatePath('/dashboard/characters');
}
