import { useReducer } from 'react'
import TextField from '@mui/material/TextField'
import Box from '@mui/material/Box'
import { Button, ButtonGroup, Stack, Typography } from '@mui/material'
import { useQuery, useQueryClient } from 'react-query'
import {
    canAddConsonant,
    canAddVowel,
    canShuffle,
    canSubmit,
    initPool,
    reducePool,
} from './pool'
import { useSearchParams } from 'react-router-dom'

export function Countdown(): JSX.Element {
    const [searchParams, setSearchParams] = useSearchParams()
    const queryClient = useQueryClient()

    const q = searchParams.get('q')
    if (q) {
        return (
            <CountdownResults search={{ q }} back={() => setSearchParams({})} />
        )
    } else {
        const handleSubmit = async (value: string) => {
            await queryClient.prefetchQuery(['countdown', { q: value }], () =>
                fetchCountdown({ q: value })
            )
            setSearchParams({ q: value })
        }
        return <Form onSubmit={handleSubmit} />
    }
}

interface FormProps {
    onSubmit: (newValue: string) => void
}

function Form({ onSubmit }: FormProps) {
    const [pool, dispatch] = useReducer(reducePool, initPool())

    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        dispatch({ type: 'set', letters: event.target.value })
    }

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        onSubmit(pool.word)
    }

    return (
        <Box component="form" noValidate onSubmit={handleSubmit}>
            <Typography variant="h4" component="h1" gutterBottom>
                Countdown Search
            </Typography>
            <Box
                sx={{
                    display: 'flex',
                    flexDirection: 'column',
                    '& > *': {
                        m: 1,
                    },
                }}
            >
                <ButtonGroup variant="outlined" aria-label="button group">
                    <Button
                        onClick={() => dispatch({ type: 'v' })}
                        disabled={!canAddVowel(pool)}
                    >
                        Vowel
                    </Button>
                    <Button
                        onClick={() => dispatch({ type: 'c' })}
                        disabled={!canAddConsonant(pool)}
                    >
                        Consonant
                    </Button>
                </ButtonGroup>
                <ButtonGroup variant="outlined" aria-label="button group">
                    <Button
                        onClick={() => dispatch({ type: 'pick', v: 3, c: 6 })}
                    >
                        3V6C
                    </Button>
                    <Button
                        onClick={() => dispatch({ type: 'pick', v: 4, c: 5 })}
                    >
                        4V5C
                    </Button>
                    <Button
                        onClick={() => dispatch({ type: 'pick', v: 5, c: 4 })}
                    >
                        5V4C
                    </Button>
                </ButtonGroup>
            </Box>

            <TextField
                variant="outlined"
                fullWidth
                margin="normal"
                required
                id="q"
                label="Letters"
                name="q"
                value={pool.word}
                onChange={handleChange}
                autoFocus
            />

            <Stack direction="row" spacing={1}>
                <Button
                    type="submit"
                    variant="contained"
                    color="primary"
                    disabled={!canSubmit(pool)}
                >
                    Solve
                </Button>
                <Button
                    variant="outlined"
                    onClick={() => dispatch({ type: 'shuffle' })}
                    disabled={!canShuffle(pool)}
                >
                    Shuffle
                </Button>
            </Stack>

            <Box mt={2}>
                <Board word={pool.word} letters="" />
            </Box>
        </Box>
    )
}

function CountdownResults({
    search,
    back,
}: {
    search: CountdownQuery
    back: () => void
}) {
    const { data, isError } = useQuery(['countdown', search], () =>
        fetchCountdown(search)
    )

    if (isError) {
        return (
            <Typography variant="h4" component="h1" gutterBottom>
                Error
            </Typography>
        )
    }

    if (!data) {
        return (
            <Typography variant="h4" component="h1" gutterBottom>
                Loading...
            </Typography>
        )
    }

    return (
        <Box>
            <Stack spacing={4} sx={{ p: 2 }}>
                <h1>Letters</h1>
                <Board key="_q" word={search.q} letters="" />
                <Button variant="contained" onClick={() => back()}>
                    Done
                </Button>

                <h1>Results</h1>
                {data.words.map((word) => (
                    <Board key={word} word={word} letters={search.q} />
                ))}
            </Stack>
        </Box>
    )
}

type BoardProps = {
    word: string
    letters: string
}

function Board({ word, letters }: BoardProps) {
    word = word.toLocaleUpperCase()
    letters = letters.toLocaleUpperCase()

    let remaining = letters
    for (const letter of word.split('')) {
        remaining = remaining.replace(letter, '')
    }
    return (
        <a
            href={'https://www.thefreedictionary.com/' + word}
            target="_blank"
            rel="noreferrer"
        >
            <Stack direction="column">
                <Letters letters={word} length={9} align="left" />
                <Letters letters={remaining} dim length={9} align="right" />
            </Stack>
        </a>
    )
}

interface LettersProps {
    letters: string
    length: number
    dim?: boolean
    align: 'left' | 'right'
}

function Letters({ letters, length, dim, align }: LettersProps) {
    letters = pad(letters, length, ' ', align)
    return (
        <Stack direction="row">
            {letters.split('').map((letter, i) => (
                <Tile key={i} dim={dim} letter={letter} />
            ))}
        </Stack>
    )
}

function Tile({ letter, dim }: { letter: string; dim?: boolean }) {
    if (dim) {
        return <Box sx={{ ...TILE_SX, color: '#999' }}>{letter}</Box>
    } else {
        return <Box sx={TILE_SX}>{letter}</Box>
    }
}

const TILE_SX = {
    fontSize: '30px',
    width: '1.2em',
    height: '1.2em',
    lineHeight: '1.2em',
    fontWeight: 'bold',
    margin: '1px',
    padding: '0',
    color: 'white',
    backgroundColor: '#1b1f9e',
    textAlign: 'center',
    float: 'left',
}

function pad(
    str: string,
    length: number,
    pad: string,
    align: 'left' | 'right'
) {
    if (align === 'left') {
        while (str.length < length) {
            str += pad
        }
    } else {
        while (str.length < length) {
            str = pad + str
        }
    }
    return str
}

export async function fetchCountdown(
    search: CountdownQuery
): Promise<CountdownResult> {
    const response = await fetch(`/api/countdown?${toUrlSearchParams(search)}`)
    return response.json()
}

function toUrlSearchParams(search: CountdownQuery) {
    const params = new URLSearchParams()
    Object.entries(search).forEach(([key, value]) => {
        if (value) {
            params.set(key, value.toString())
        }
    })
    return params
}

export interface CountdownQuery {
    q: string
}

interface CountdownResult {
    q: string
    words: string[]
}
