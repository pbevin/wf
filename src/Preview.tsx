import { Box } from '@mui/material'
import { InputForm, PreviewResults } from './SearchTypes'
import { colorizeWord } from './Results'

export const Preview = ({ data, form }: PreviewProps): JSX.Element | null => {
    if (data.num_total === 0) {
        return null
    }

    const elements = data.groups.map(([len, words]) => (
        <Box key={len}>
            {len}:
            {words.map(([word, rating]) => (
                <span key={word}>{colorizeWord(word, rating)} </span>
            ))}
        </Box>
    ))

    let searchTerm = form.input
    if (searchTerm.length >= 5) {
        searchTerm += ` (${searchTerm.length} letters)`
    }

    return (
        <>
            <Box>
                Preview: {data.num_shown} of {data.num_total} results for{' '}
                {searchTerm}
            </Box>
            {elements}
        </>
    )
}
type PreviewProps = {
    data: PreviewResults
    form: InputForm
}
