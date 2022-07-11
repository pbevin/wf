import {
    Box,
    FormControl,
    FormControlLabel,
    FormLabel,
    Radio,
    RadioGroup,
    TextField,
} from '@mui/material'
import { GameType, InputForm, SearchResults } from './SearchTypes'
import { Preview } from './Preview'

type SearchFormProps = {
    preview: SearchResults | undefined
    form: InputForm
    onChange: (form: InputForm) => void
    onSubmit: (form: InputForm) => void
}

export function SearchForm({
    preview,
    form,
    onChange,
    onSubmit,
}: SearchFormProps) {
    const handleChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const input = event.target.value.toUpperCase().replace(/[^A-Z]/g, '')
        onChange({ ...form, input })
    }

    const handleGoalChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        onChange({ ...form, goal: event.target.value as GameType })
    }

    const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault()
        onSubmit(form)
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
                        <FormControlLabel
                            value="anagram"
                            control={<Radio />}
                            label="Anagram"
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
                {preview && <Preview data={preview} form={form} />}
            </Box>
        </Box>
    )
}
