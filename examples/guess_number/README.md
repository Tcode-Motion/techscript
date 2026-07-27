# Guess the Number Example

This example simulates a guessing game in TechScript, using loops and conditional logic.

## Code (`guess.txs`)
```txs
secret_number = 42
guesses = [10, 50, 40, 42]

for guess in guesses
    say $"Guessing: {guess}"
    when guess < secret_number
        say "Too low!"
    else when guess > secret_number
        say "Too high!"
    else
        say "Correct! You guessed it!"
        break
    end
end
```

## Running the Example
```bash
tech run guess.txs
```

## Expected Output
```
Guessing: 10
Too low!
Guessing: 50
Too high!
Guessing: 40
Too low!
Guessing: 42
Correct! You guessed it!
```
