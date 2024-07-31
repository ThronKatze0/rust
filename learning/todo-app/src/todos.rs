use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn add_todo(&mut self, todo: Todo) -> Result<(), &'static str> {
        if let Some(_) = self.includes(&todo) {
            return Err("Todo is already included");
        }
        self.todos.push(todo);
        Ok(())
    }

    pub fn add(&mut self, name: String, checked: bool) -> Result<(), &'static str> {
        self.add_todo(Todo { name, checked })
    }

    pub fn remove(&mut self, name: &String) -> Result<(), &'static str> {
        if let Some(index) = self.todos.iter().position(|todo: &Todo| todo.name == *name) {
            self.todos.remove(index);
            return Ok(());
        }
        Err("Todo doesn't exist")
    }

    pub fn check(&mut self, name: &String) -> Result<(), &'static str> {
        if let Some(todo) = self.todos.iter_mut().find(|todo| todo.name == *name) {
            todo.checked = !todo.checked;
            return Ok(());
        }
        Err("Can't find todo")
    }

    fn includes(&self, todo: &Todo) -> Option<&Todo> {
        self.todos.iter().find(|&current_todo| current_todo == todo)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    name: String,
    checked: bool,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
