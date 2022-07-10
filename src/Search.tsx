import { Box } from '@mui/material'

import { useState } from 'react'
import { useQuery } from 'react-query'
import { useSearchParams } from 'react-router-dom'
import { Results } from './Results'
import { SearchForm } from './SearchForm'
import { GameType, InputForm, PreviewResults } from './SearchTypes'

async function fetchPreview(form: InputForm): Promise<PreviewResults> {
    if (form.input === '') {
        return {
            num_total: 0,
            num_shown: 0,
            groups: [],
        }
    }
    const url =
        '/api/preview?' +
        new URLSearchParams({ q: form.input, goal: form.goal })
    const resp = await fetch(url)
    return await resp.json()
}

export function Search(): JSX.Element {
    const [form, setForm] = useState<InputForm>({
        input: '',
        goal: 'connect',
    })
    const preview = useQuery(['preview', form], async () => fetchPreview(form))
    const [searchParams, setSearchParams] = useSearchParams()

    const resultsForm = inputFormFromSearchParams(searchParams)

    return (
        <Box>
            <SearchForm
                preview={preview.data}
                form={form}
                onChange={setForm}
                onSubmit={() =>
                    setSearchParams({ q: form.input, goal: form.goal })
                }
            />
            {resultsForm && <Results form={resultsForm} />}
        </Box>
    )
}

const inputFormFromSearchParams = (
    searchParams: URLSearchParams
): InputForm | null => {
    const q = searchParams.get('q')
    const goal = searchParams.get('goal')
    if (q !== null && (goal === 'connect' || goal === 'countdown')) {
        return {
            input: q.toUpperCase().replace(/[^A-Z]/g, ''),
            goal: goal as GameType,
        }
    }
    return null
}
