module.exports = {
    '*.{js,jsx,ts,tsx}': ['eslint --max-warnings=0'],
    '**/*.ts?(x)': () => 'tsc -p tsconfig.json --noEmit',
    '*.{js,jsx,ts,tsx,json,css,js}': ['prettier --write'],
}
