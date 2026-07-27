# Todo List CLI Example

This example demonstrates managing a stateful collection of objects (lists containing maps) in TechScript.

## Code (`todo.txs`)
```txs
todo_list = []

do add_todo(title)
    item = {
        "title": title,
        "completed": false
    }
    todo_list.push(item)
    say $"Added: {title}"
end

do complete_todo(index)
    when index < len(todo_list)
        todo_list[index]["completed"] = true
        say $"Completed: {todo_list[index]['title']}"
    else
        say "Invalid Index"
    end
end

do print_todos()
    say "--- TODO LIST ---"
    for i in 0..(len(todo_list))
        status = "[ ]"
        when todo_list[i]["completed"]
            status = "[x]"
        end
        say $"{status} {i + 1}. {todo_list[i]['title']}"
    end
end

add_todo("Learn TechScript syntax")
add_todo("Write a simple compiler")
add_todo("Build an open source language")

complete_todo(0)
print_todos()
```

## Running the Example
```bash
tech run todo.txs
```

## Expected Output
```
Added: Learn TechScript syntax
Added: Write a simple compiler
Added: Build an open source language
Completed: Learn TechScript syntax
--- TODO LIST ---
[x] 1. Learn TechScript syntax
[ ] 2. Write a simple compiler
[ ] 3. Build an open source language
```
