import { Box } from '@mui/material'
import { InputForm, SearchResults } from './SearchTypes'
import { colorizeWord } from './Results'

export function Preview({ data, form }: PreviewProps): JSX.Element {
    switch (data.type) {
        case 'words_by_length':
            return <PreviewWordsByLength data={data} form={form} />
        case 'anagrams':
            return <PreviewAnagrams data={data} form={form} />
        default:
            return <Box>No results for &lsquo;{form.input}&rsquo;</Box>
    }
}

type PreviewProps = {
    data: SearchResults
    form: InputForm
}

function PreviewWordsByLength({
    data,
    form,
}: PreviewWordsByLengthProps): JSX.Element {
    const elements = data.groups.map(({ len, words }) => (
        <Box key={len}>
            {len}
            {': '}
            {words.map(({ word, rating }) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))

    return (
        <>
            <Box>
                Preview: {data.num_shown} of {data.num_total} results for{' '}
                {searchTermFromInputString(form.input)}
            </Box>
            {elements}
        </>
    )
}

type PreviewWordsByLengthProps = {
    data: SearchResults & { type: 'words_by_length' }
    form: InputForm
}

function PreviewAnagrams({ data, form }: PreviewAnagramsProps): JSX.Element {
    const results = data.values.map(([, words], i) => (
        <Box key={i}>
            {words.map(({ word, rating }) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))
    return (
        <Box>
            <Box>
                Preview: {data.num_shown} of {data.num_total} results for{' '}
                {searchTermFromInputString(form.input)}
            </Box>
            {results}
        </Box>
    )
}

type PreviewAnagramsProps = {
    data: SearchResults & { type: 'anagrams' }
    form: InputForm
}

function searchTermFromInputString(input: string) {
    if (input.length >= 5) {
        input += ` (${input.length} letters)`
    }
    return input
}
