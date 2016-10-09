trait ProgramMemoryHistory<T: Sized> {

    /// Creates a new event and therefore steps up in the time axis
    fn create_event(&mut self);

    /// Reads from the storage at given time
    ///
    /// `past_time` - past time from now, 1 for previous time
    /// `identifier` - unique value that can be used to identify the value
    fn read(&self, past_time: usize, identifier: usize) -> T;

    /// Writes to the storage at given time
    ///
    /// `past_time` - past time from now, 1 for previous time
    /// `identifier` - unique value that can be used to identify the value
    /// `value` - value that will be written to the memory
    fn write(&mut self, past_time: usize, identifier: usize, value: &T);

}

struct InMemoryProgramMemoryHistory<T> {
    memory: Vec<Vec<T>>
}

/*
impl ProgramMemoryHistory for InMemoryProgramMemoryHistory {

    fn create_event(&mut self) {
        self.memory.push(Vec::new());
    }

    fn read(&self, past_time: usize, identifier: usize) -> T {
        
    }
}
*/