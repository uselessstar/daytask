mod id;

pub use id::TaskId;

pub struct Task {
    id: TaskId,
}

impl Task {
    pub fn new() -> Self {
        Self { id: TaskId::new() }
    }

    #[must_use]
    pub fn id(&self) -> TaskId {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn task_id_is_unique() {
        let task1 = Task::new();
        let task2 = Task::new();
        assert_ne!(task1.id(), task2.id());
    }
}
