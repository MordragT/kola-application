use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(description: impl Into<String>) -> Task {
        Task {
            description: description.into(),
            done: false,
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { description, done } = self;
        let status = if *done { "[X]" } else { "[]" };
        write!(f, "{status} : {description}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(usize);

#[derive(Debug, Clone, Default)]
pub struct App {
    tasks: Vec<Task>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, task: Task) -> TaskId {
        let id = self.tasks.len();

        self.tasks.push(task);

        TaskId(id)
    }

    pub fn find(&self, by: usize) -> Option<TaskId> {
        if self.tasks.get(by).is_some() {
            Some(TaskId(by))
        } else {
            None
        }
    }

    pub fn complete(&mut self, id: TaskId) {
        self.tasks[id.0].done = true;
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id, task) in self.tasks.iter().enumerate() {
            writeln!(f, "{id} {task}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Task, TaskId};

    #[test]
    fn test() {
        let mut app = App::new();

        let a = Task::new("a");
        let a_id = app.add(a);
        let found = app.find(0).unwrap();

        assert_eq!(a_id, found);
    }

    #[test]
    #[should_panic]
    fn fail() {
        let mut app = App::new();

        let id = TaskId(0);

        app.complete(id);
    }
}
