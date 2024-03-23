
export type Corporation = {
    error?: string,

    name: string,
    ticker: string,
    member_count: number,

    alliance_id?: number

    ceo_id: number,
    creator_id: number

    date_founded?: string,

    description?: string,

    faction_id?: number,

    home_station_id?: number,

    war_eligible?: boolean,

    url?: string,
}

export type Alliance = {
    error?: string,

    name: string,
    ticker: string,

    creator_corporation_id: number,
    executor_corporation_id?: number,

    creator_id: number,
    date_founded: string,

    faction_id?: number,
}