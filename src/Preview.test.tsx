import '@testing-library/jest-dom'
import { render, screen } from '@testing-library/react'
import { Preview } from './Preview'
import { InputForm, SearchResults } from './SearchTypes'

const preview: SearchResults = {
    num_total: 29,
    num_shown: 4,
    type: 'words_by_length',
    groups: [
        { len: 5, words: [{ word: 'xyzzy', rating: 3 }] },
        {
            len: 3,
            words: [
                { word: 'foo', rating: 3 },
                { word: 'bar', rating: 2 },
                { word: 'baz', rating: 1 },
            ],
        },
    ],
}

test('shows a preview', async () => {
    const form: InputForm = { input: 'QUUX', goal: 'connect' }
    render(<Preview data={preview} form={form} />)
    expect(screen.getByText('Preview: 4 of 29 results for QUUX')).not.toBeNull()
})

test('shows number of letters for longer words', () => {
    const form: InputForm = { input: 'FOOBAR', goal: 'connect' }
    render(<Preview data={preview} form={form} />)
    expect(
        screen.getByText('Preview: 4 of 29 results for FOOBAR (6 letters)')
    ).not.toBeNull()
})
