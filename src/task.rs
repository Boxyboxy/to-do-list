#[derive(Debug)]
pub struct Task {
    pub task: String,
    pub done_status: bool,
    pub id: u64,
}

impl Task {
    pub fn update_status(&mut self) {
        self.done_status = true;
    }

    pub fn update_task(&mut self, new_name: String) {
        self.task = new_name;
    }
}
