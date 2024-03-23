
export type Character = {
    character_id: number,
    name: string,

    corporation_id: number,
    alliance_id?: number,

    is_main: boolean,
    requires_authorization: boolean,
}
