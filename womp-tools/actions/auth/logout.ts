"use server"

import { cookies } from 'next/headers';

export default async function logout() {
    const cookieStore = cookies();
    cookieStore.delete('access_token');
    cookieStore.delete('refresh_token');
}