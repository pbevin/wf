import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'
import CssBaseline from '@mui/material/CssBaseline'
import { ThemeProvider } from '@mui/material/styles'
import React from 'react'
import { QueryClient, QueryClientProvider } from 'react-query'
import theme from './theme'
import App from './App'

const rootElement = document.getElementById('root')
if (rootElement) {
    const root = createRoot(rootElement)
    const queryClient = new QueryClient()

    root.render(
        <React.StrictMode>
            <QueryClientProvider client={queryClient}>
                <ThemeProvider theme={theme}>
                    {/* CssBaseline kickstart an elegant, consistent, and simple baseline to build upon. */}
                    <CssBaseline />
                    <BrowserRouter>
                        <App />
                    </BrowserRouter>
                </ThemeProvider>
            </QueryClientProvider>
        </React.StrictMode>
    )
}
