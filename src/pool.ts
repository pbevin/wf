export type FormAction =
    | { type: 'v' }
    | { type: 'c' }
    | { type: 'pick'; v: number; c: number }
    | { type: 'set'; letters: string }
    | { type: 'shuffle' }

export function reducePool(pool: Pool, action: FormAction): Pool {
    switch (action.type) {
        case 'v':
            return takeVowel(pool)

        case 'c':
            return takeConsonant(pool)

        case 'pick':
            return pick(action.v, action.c)

        case 'set':
            return setWord(action.letters)

        case 'shuffle':
            return shuffleWord(pool)
    }
}

export interface Pool {
    vowels: string
    consonants: string
    word: string
}

function takeVowel(pool: Pool): Pool {
    if (countVowels(pool.word) >= 5) {
        return pool
    }
    const v = pool.vowels[0]
    return {
        ...pool,
        vowels: pool.vowels.slice(1),
        word: pool.word + v,
    }
}

function takeConsonant(pool: Pool): Pool {
    if (countConsonants(pool.word) >= 6) {
        return pool
    }
    const c = pool.consonants[0]
    return {
        ...pool,
        consonants: pool.consonants.slice(1),
        word: pool.word + c,
    }
}

function pick(numVowels: number, numConsonants: number): Pool {
    let pool = initPool()
    for (let i = 0; i < numVowels; i++) {
        pool = takeVowel(pool)
    }
    for (let i = 0; i < numConsonants; i++) {
        pool = takeConsonant(pool)
    }
    pool.word = shuffle(pool.word.split('')).join('')
    return pool
}

function setWord(word: string): Pool {
    let { vowels, consonants } = initPool()
    let taken = ''
    word.toUpperCase()
        .split('')
        .forEach((letter) => {
            if (taken.length === 9) {
                return
            }
            if (vowels.includes(letter)) {
                vowels = vowels.replace(letter, '')
                taken += letter
            } else if (consonants.includes(letter)) {
                consonants = consonants.replace(letter, '')
                taken += letter
            }
        })

    word = taken
    return {
        vowels,
        consonants,
        word,
    }
}

export function shuffleWord(pool: Pool): Pool {
    const word = shuffle(pool.word.split('')).join('')
    return {
        ...pool,
        word,
    }
}

export function initPool(): Pool {
    // Frequencies come from http://www.thecountdownpage.com/letters.htm
    const vowels =
        'A'.repeat(15) +
        'E'.repeat(21) +
        'I'.repeat(13) +
        'O'.repeat(13) +
        'U'.repeat(5)

    const consonants =
        'JKQVWXYZ' +
        'BFH'.repeat(2) +
        'GC'.repeat(3) +
        'MP'.repeat(4) +
        'L'.repeat(5) +
        'D'.repeat(6) +
        'N'.repeat(8) +
        'RST'.repeat(9)

    return {
        vowels: shuffle(vowels.split('')).join(''),
        consonants: shuffle(consonants.split('')).join(''),
        word: '',
    }
}

function shuffle<T>(a: T[]): T[] {
    const n = a.length
    for (let i = n - 1; i > 0; i--) {
        const j = Math.floor(Math.random() * (i + 1))
        const tmp = a[i]
        a[i] = a[j]
        a[j] = tmp
    }
    return a
}

// Must have 4 consonants and 3 vowels
export function canSubmit(pool: Pool) {
    return (
        pool.word.length === 9 &&
        countVowels(pool.word) >= 3 &&
        countConsonants(pool.word) >= 4
    )
}

export function canShuffle(pool: Pool) {
    return pool.word.length === 9
}

export function canAddVowel(pool: Pool) {
    return countVowels(pool.word) < 5
}

export function canAddConsonant(pool: Pool) {
    return countConsonants(pool.word) < 6
}

function countVowels(word: string) {
    return word.replace(/[^aeiou]/gi, '').length
}

function countConsonants(word: string) {
    return word.replace(/[^bcdfghjklmnpqrstvwxyz]/gi, '').length
}
