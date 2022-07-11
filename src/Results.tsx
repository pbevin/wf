import { Box } from '@mui/material'
import { InputForm, SearchResults, Rating as WordRating } from './SearchTypes'

// Displays the full results for a search.
export function Results(props: ResultsProps): JSX.Element | null {
    const { form, data, preview } = props
    switch (data.type) {
        case 'words_by_length':
            return <WordsByLength data={data} form={form} preview={preview} />
        case 'anagrams':
            return <Anagrams data={data} form={form} preview={preview} />
        default:
            return null
    }
}

type ResultsProps = {
    form: InputForm
    data: SearchResults
    preview?: boolean
}

export function WordsByLength({
    data,
    form,
    preview,
}: WordsByLengthProps): JSX.Element {
    const groups = data.groups.map(({ len, words }) => (
        <Box key={len}>
            {len}:
            {words.map(({ word, rating }) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))
    const heading = (): JSX.Element => {
        if (preview) {
            return (
                <Box sx={{ mb: 1 }}>
                    Preview: {data.num_shown} of {data.num_total} results for{' '}
                    {term}
                </Box>
            )
        } else {
            return <h2>Results for {term}</h2>
        }
    }
    const term = searchTermFromInputString(form.input)
    return (
        <Box>
            {heading()}
            {groups}
        </Box>
    )
}

type WordsByLengthProps = {
    data: SearchResults & {
        type: 'words_by_length'
    }
    form: InputForm
    preview?: boolean
}

function Anagrams({ data, form }: AnagramsProps): JSX.Element {
    return (
        <Box>
            <h2>Results for {searchTermFromInputString(form.input)}</h2>
            <p>
                Showing {data.num_shown} of {data.num_total} results
            </p>
            <Box
                sx={{
                    width: '100%',
                    bgcolor: 'background.paper',
                }}
            >
                {data.anagrams.map(({ words, remainder }, i) => (
                    <Box key={i}>
                        {words.map(({ word, rating }, idx) => (
                            <span key={word}>
                                {idx > 0 && ' + '}
                                {colorizeWord(word.toUpperCase(), rating)}
                            </span>
                        ))}
                        {remainder && ` + ${remainder}`}
                    </Box>
                ))}
            </Box>
        </Box>
    )
}

type AnagramsProps = {
    data: SearchResults & {
        type: 'anagrams'
    }
    form: InputForm
    preview?: boolean
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

function searchTermFromInputString(input: string) {
    if (input.length >= 5) {
        input += ` (${input.length} letters)`
    }
    return input
}
