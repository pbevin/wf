import '@testing-library/jest-dom'
import { render, screen } from '@testing-library/react'
import { Preview } from './Preview'
import { InputForm, PreviewResults } from './SearchTypes'

const preview: PreviewResults = {
    num_total: 29,
    num_shown: 4,
    groups: [
        [5, [['xyzzy', 2]]],
        [
            3,
            [
                ['foo', 3],
                ['bar', 1],
                ['baz', 1],
            ],
        ],
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
