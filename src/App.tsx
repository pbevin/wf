import { Box, Container, Paper } from '@mui/material'
import { Route, Routes } from 'react-router-dom'
import './App.css'
import { Countdown } from './Countdown'
import { Search } from './Search'

function App() {
    return (
        <Container maxWidth="sm">
            <Paper elevation={3}>
                <Box
                    sx={{
                        minHeight: '600px',
                        m: {
                            sm: 0,
                            md: 3,
                        },
                        p: {
                            sm: 0,
                            md: 3,
                        },
                    }}
                >
                    <Routes>
                        <Route path="/" element={<Search />} />
                        <Route path="/countdown" element={<Countdown />} />
                    </Routes>
                </Box>
            </Paper>
        </Container>
    )
}

export default App
