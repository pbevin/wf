export const game_types = ['countdown', 'connect', 'anagram'] as const
export type GameType = typeof game_types[number]

export type InputForm = {
    input: string
    goal: GameType
    limit?: number
}

export type SearchResults = Counts & TypedResult

export type Counts = {
    num_total: number
    num_shown: number
}

export type TypedResult =
    | { type: 'words_by_length'; groups: LengthGroup[] }
    | { type: 'anagrams'; anagrams: AnagramResult[] }
    | { type: 'empty' }

/**
 * A group of words with the same length
 */
export type LengthGroup = { len: number; words: RatedWord[] }
export type RatedWord = { word: string; rating: Rating }
// 1 = unpopular word (but still in lexicon), 2 = popular word, 3 = very popular word
export type Rating = 1 | 2 | 3

export type AnagramResult = { words: RatedWord[]; remainder: string }

export function goalFromString(input: string | null): GameType | undefined {
    return game_types.find((g) => g === input)
}

export type AnagramResults = {
    num_total: number
    num_shown: number
    results: AnagramResult[]
}

export function inputFormFromSearchParams(
    searchParams: URLSearchParams
): InputForm | undefined {
    const input = searchParams.get('q')
    const goal = goalFromString(searchParams.get('goal'))
    if (!input || !goal) {
        return undefined
    }
    return { input, goal }
}
