export type GameType = 'countdown' | 'connect'

export interface InputForm {
    input: string
    goal: GameType
}

export type PreviewResults = {
    num_total: number
    num_shown: number
    groups: PreviewGroup[]
}
export type PreviewGroup = [number, RatedWord[]]
export type RatedWord = [string, Rating]
export type Rating = 1 | 2 | 3
