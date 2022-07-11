import { Box } from '@mui/material'

import { useState } from 'react'
import { useQuery } from 'react-query'
import { useSearchParams } from 'react-router-dom'
import { Results } from './Results'
import { SearchForm } from './SearchForm'
import {
    InputForm,
    inputFormFromSearchParams,
    SearchResults,
} from './SearchTypes'

async function fetchPreview(form: InputForm): Promise<SearchResults> {
    if (form.input === '') {
        return {
            num_total: 0,
            num_shown: 0,
            type: 'empty',
        }
    }
    const url = `/api/preview?${new URLSearchParams({
        q: form.input,
        goal: form.goal,
    })}`
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
