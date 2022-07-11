import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom'
import { keyboard } from '@testing-library/user-event/dist/keyboard'
import { SearchForm } from './SearchForm'
import { game_types, InputForm, SearchResults } from './SearchTypes'

test('loads and displays a title', async () => {
    const preview: SearchResults = {
        num_total: 0,
        num_shown: 0,
        type: 'empty',
    }

    const onChange = jest.fn()
    const onSubmit = jest.fn()

    const form: InputForm = { input: '', goal: 'connect' }

    render(
        <SearchForm
            form={form}
            onChange={onChange}
            onSubmit={onSubmit}
            preview={preview}
        />
    )

    const heading = await screen.findByRole('heading')
    expect(heading).toHaveTextContent('Word Search')

    const radios = await screen.findAllByRole('radio')
    expect(radios).toHaveLength(game_types.length)
})

test('sends onChange when text is typed', async () => {
    const preview: SearchResults = {
        num_total: 0,
        num_shown: 0,
        type: 'empty',
    }

    const onChange = jest.fn()
    const onSubmit = jest.fn()

    const form: InputForm = { input: '', goal: 'connect' }

    render(
        <SearchForm
            form={form}
            onChange={onChange}
            onSubmit={onSubmit}
            preview={preview}
        />
    )

    keyboard('r')

    expect(onChange).toHaveBeenCalledWith({ input: 'R', goal: 'connect' })
})
