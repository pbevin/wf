import { Box } from '@mui/material'
import { useQuery } from 'react-query'
import { InputForm, PreviewResults, Rating as WordRating } from './SearchTypes'

// Displays the full results for a search.
export const Results = ({ form }: { form: InputForm }): JSX.Element | null => {
    const { data } = useQuery(['results', form], async () => {
        return await fetchFullResults(form)
    })

    if (!data) {
        return null
    }
    const groups = data.groups.map(([len, words]) => (
        <Box key={len}>
            {len}:
            {words.map(([word, rating]) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))
    return (
        <Box>
            <h2>Results for {form.input.toUpperCase()}</h2>
            {groups}
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

async function fetchFullResults(form: InputForm): Promise<PreviewResults> {
    const url =
        '/api/results?' +
        new URLSearchParams({ q: form.input, goal: form.goal })
    const resp = await fetch(url)
    return await resp.json()
}
