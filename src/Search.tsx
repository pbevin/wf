import { useState } from 'react'
import { useQuery } from 'react-query'
import { useSearchParams } from 'react-router-dom'
import { Results } from './Results'
import { SearchForm } from './SearchForm'
import {
    GameType,
    InputForm,
    inputFormFromSearchParams,
    SearchResults,
} from './SearchTypes'

export function Search(): JSX.Element {
    const [form, setForm] = useState<InputForm>({
        input: '',
        goal: 'connect',
    })
    const preview = useQuery(['preview', form], async () =>
        fetchResults({ ...form, limit: 10 })
    )
    const [searchParams, setSearchParams] = useSearchParams()

    const resultsForm = inputFormFromSearchParams(searchParams)
    const results = useQuery(['full', resultsForm], async () =>
        fetchResults(form)
    )
    if (resultsForm && results.data) {
        return <Results form={resultsForm} data={results.data} />
    } else {
        return (
            <SearchForm
                preview={preview.data}
                form={form}
                onChange={setForm}
                onSubmit={() =>
                    setSearchParams({ q: form.input, goal: form.goal })
                }
            />
        )
    }
}

type SP = {
    q: string
    goal: GameType
    limit?: string
}

async function fetchResults(form: InputForm): Promise<SearchResults> {
    if (form.input === '') {
        return {
            num_total: 0,
            num_shown: 0,
            type: 'empty',
        }
    }

    const sp: SP = {
        q: form.input,
        goal: form.goal,
    }
    if (form.limit) {
        sp.limit = form.limit.toString()
    }

    const url = `/api/search?${new URLSearchParams(sp)}`
    const resp = await fetch(url)
    return await resp.json()
}
