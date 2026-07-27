# Enums Example

This example demonstrates how to declare enums and use pattern matching with `match` and `case` in TechScript.

## Code (`enums.txs`)
```txs
enum TrafficLight
    Red
    Yellow
    Green
end

do get_action(light)
    match light
    case TrafficLight.Red
        send "Stop"
    case TrafficLight.Yellow
        send "Caution"
    case TrafficLight.Green
        send "Go"
    default
        send "Unknown state"
    end
end

say $"Red Light: {get_action(TrafficLight.Red)}"
say $"Green Light: {get_action(TrafficLight.Green)}"
```

## Running the Example
```bash
tech run enums.txs
```

## Expected Output
```
Red Light: Stop
Green Light: Go
```
