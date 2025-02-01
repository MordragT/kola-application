use app::{App, Task};
use std::io::{self, Stdout, Write};

mod app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    let mut stdout = io::stdout();
    let stdin = io::stdin();

    let prompt = |stdout: &mut Stdout, p| -> Result<String, io::Error> {
        write!(stdout, "{p}")?;
        stdout.flush()?;
        let mut buf = String::new();
        stdin.read_line(&mut buf)?;
        Ok(buf)
    };

    loop {
        writeln!(stdout, "1. Add new Task")?;
        writeln!(stdout, "2. Mark Task As Done")?;
        writeln!(stdout, "3. Show Tasks")?;
        writeln!(stdout, "4. Exit")?;

        let choice = prompt(&mut stdout, "Enter your choice: ")?;

        match choice.trim() {
            "1" => {
                let description = prompt(&mut stdout, "Enter task description: ")?;
                let task = Task::new(description);
                app.add(task);
            }
            "2" => {
                let index = prompt(&mut stdout, "Enter the task index to mark as done: ")?;
                if let Ok(index) = usize::from_str_radix(index.trim(), 10) {
                    if let Some(id) = app.find(index) {
                        app.complete(id);
                    } else {
                        writeln!(stdout, "Task not found")?;
                        continue;
                    }
                } else {
                    writeln!(stdout, "Invalid input, enter valid number")?;
                    continue;
                }
            }
            "3" => writeln!(stdout, "{app}")?,
            "4" => break,
            _ => writeln!(stdout, "Invalid option, enter number between 1-4")?,
        }
    }

    Ok(())
}
