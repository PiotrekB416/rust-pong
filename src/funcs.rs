use web_sys::js_sys::Function;

#[derive(Clone)]
pub struct Funcs {
    intervals: Vec<i32>,
    events: Vec<Function>,
}

impl Funcs {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.intervals.len() + self.events.len()
    }

    pub fn push_interval(&mut self, id: i32) {
        self.intervals.push(id);
    }

    pub fn get_intervals(&self) -> Vec<i32> {
        self.intervals.clone()
    }

    pub fn remove_all(&mut self) {
        self.intervals.clear();
        self.events.clear();
    }

    pub fn push_event(&mut self, event: Function) {
        self.events.push(event);
    }

    pub fn get_events(&self) -> Vec<Function> {
        self.events.clone()
    }
}
