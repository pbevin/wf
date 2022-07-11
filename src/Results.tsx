import { Box } from '@mui/material'
import { useQuery } from 'react-query'
import { InputForm, SearchResults, Rating as WordRating } from './SearchTypes'

// Displays the full results for a search.
export function Results({ form }: { form: InputForm }): JSX.Element | null {
    const { data } = useQuery(
        ['results', form],
        async () => await fetchFullResults(form)
    )

    if (!data) {
        return null
    }

    switch (data.type) {
        case 'words_by_length':
            return <WordsByLength data={data} form={form} />
        case 'anagrams':
            return <Anagrams data={data} form={form} />
        default:
            return null
    }
}

type WordsByLengthProps = {
    data: SearchResults & {
        type: 'words_by_length'
    }
    form: InputForm
}

export function WordsByLength({ data, form }: WordsByLengthProps): JSX.Element {
    const groups = data.groups.map(({ len, words }) => (
        <Box key={len}>
            {len}:
            {words.map(({ word, rating }) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))
    return (
        <Box>
            <h2>
                Results for
                {form.input.toUpperCase()}
            </h2>
            {groups}
        </Box>
    )
}

type AnagramsProps = {
    data: SearchResults & {
        type: 'anagrams'
    }
    form: InputForm
}

function Anagrams({ data, form }: AnagramsProps): JSX.Element {
    const results = data.values.map(([, words], i) => (
        <Box key={i}>
            {words.map(({ word, rating }) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))
    return (
        <Box>
            <h2>
                Results for
                {form.input.toUpperCase()}
            </h2>
            {results}
        </Box>
    )
}

export const colorizeWord = (word: string, rating: WordRating) => {
    const styles = [
        { color: '#666' },
        {},
        { color: '#4caf50', fontWeight: 'bold' },
    ]
    const style = styles[rating - 1]
    return <span style={style}>{word}</span>
}

async function fetchFullResults(form: InputForm): Promise<SearchResults> {
    const url = `/api/results?${new URLSearchParams({
        q: form.input,
        goal: form.goal,
    })}`
    const resp = await fetch(url)
    return await resp.json()
}
