import React, { useEffect } from 'react'
import { useParams, useNavigate } from 'react-router'
import Container from '@mui/material/Container'
import Typography from '@mui/material/Typography'
import Box from '@mui/material/Box'
import { Button } from '@mui/material'
import Link from '@mui/material/Link'

export const Counter = () => {
    const { count } = useParams()
    const navigate = useNavigate()

    useEffect(() => {
        document.title = `You clicked ${count} times`
    }, [count])

    return (
        <Container maxWidth="sm">
            <Box sx={{ my: 4 }}>
                <Typography variant="h4" component="h1" gutterBottom>
                    <p>You clicked {count} times</p>
                </Typography>

                <Button
                    variant="outlined"
                    onClick={() => navigate('/counter/' + (Number(count) + 1))}
                >
                    Click me
                </Button>

                <Button variant="outlined">
                    <Link
                        variant="button"
                        underline="none"
                        href={'/counter/' + (Number(count) + 1)}
                    >
                        Click me
                    </Link>
                </Button>
            </Box>
        </Container>
    )
}
