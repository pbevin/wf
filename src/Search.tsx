import {
    Box,
    FormControl,
    FormControlLabel,
    FormLabel,
    Radio,
    RadioGroup,
    TextField,
} from '@mui/material'

import { useState } from 'react'
import { useQuery } from 'react-query'
import { useNavigate, useSearchParams } from 'react-router-dom'

type GameType = 'countdown' | 'connect'

interface SearchForm {
    input: string
    goal: GameType
}

type PreviewResults = {
    num_total: number
    num_shown: number
    groups: PreviewGroup[]
}
type PreviewGroup = [number, RatedWord[]]
type RatedWord = [string, Rating]
type Rating = 1 | 2 | 3

async function fetchPreview(form: SearchForm): Promise<PreviewResults | null> {
    if (form.input === '') {
        return null
    }
    const url =
        '/api/preview?' +
        new URLSearchParams({ q: form.input, goal: form.goal })
    const resp = await fetch(url)
    return await resp.json()
}

export function Search(): JSX.Element {
    const [searchParams, setSearchParams] = useSearchParams()
    const [form, setForm] = useState<SearchForm>({
        input: '',
        goal: 'connect',
    })
    const preview = useQuery(
        ['preview', form],
        async (): Promise<PreviewResults | null> => fetchPreview(form)
    )

    // if (searchParams.get('q') === "") {
    //     setSearchParams({})
    // }

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const input = event.target.value.toUpperCase().replaceAll(/[^A-Z]/g, '')
        setForm({ ...form, input })
    }

    const handleGoalChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setForm({ ...form, goal: event.target.value as GameType })
    }

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        setSearchParams({ q: form.input, goal: form.goal })
    }

    return (
        <Box>
            <h1>Word Search</h1>
            <Box component="form" noValidate onSubmit={handleSubmit}>
                <FormControl>
                    <FormLabel id="game-type-label">Game type</FormLabel>
                    <RadioGroup
                        row
                        name="goal"
                        aria-labelledby="game-type-label"
                        value={form.goal}
                        onChange={handleGoalChange}
                    >
                        <FormControlLabel
                            value="countdown"
                            control={<Radio />}
                            label="Countdown"
                        />
                        <FormControlLabel
                            value="connect"
                            control={<Radio />}
                            label="Connect"
                        />
                    </RadioGroup>
                </FormControl>
                <TextField
                    variant="outlined"
                    inputProps={{
                        autoCorrect: 'off',
                        autoCapitalize: 'off',
                    }}
                    fullWidth
                    margin="normal"
                    required
                    id="q"
                    label="Letters"
                    name="q"
                    value={form.input}
                    onChange={handleChange}
                    autoFocus
                />
                <Preview data={preview.data} />
            </Box>
            <Results />
        </Box>
    )
}

type PreviewProps = {
    data: PreviewResults | null | undefined
}

const Preview = ({ data }: PreviewProps): JSX.Element | null => {
    if (!data || data.num_total === 0) {
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

    return (
        <>
            <Box>
                Preview: {data.num_shown} of {data.num_total} results
            </Box>
            {elements}
        </>
    )
}

async function fetchFullResults(form: SearchForm): Promise<PreviewResults> {
    const url =
        '/api/results?' +
        new URLSearchParams({ q: form.input, goal: form.goal })
    const resp = await fetch(url)
    return await resp.json()
}


// Displays the full results for a search.
const Results = () => {
    const [searchParams] = useSearchParams()
    const form = {
        input: searchParams.get('q') || '',
        goal: searchParams.get('goal') as GameType,
    }
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

const colorizeWord = (word: string, rating: Rating) => {
    const styles = [
        { color: '#666' },
        {},
        { color: '#4caf50', fontWeight: 'bold' },
    ]
    const style = styles[rating - 1]
    return <span style={style}>{word}</span>
}
