import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom'
import { SearchForm } from './SearchForm'
import { InputForm, PreviewResults } from './SearchTypes'
import { keyboard } from '@testing-library/user-event/dist/keyboard'

test('loads and displays a title', async () => {
    const preview: PreviewResults = {
        num_total: 0,
        num_shown: 0,
        groups: [],
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
})

test('sends onChange when text is typed', async () => {
    const preview: PreviewResults = {
        num_total: 0,
        num_shown: 0,
        groups: [],
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
