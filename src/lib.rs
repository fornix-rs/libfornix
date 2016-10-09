extern crate rand;

mod memory;
mod util;

pub use memory::*;
pub use util::*;

/// A form of an intelligent program that is able to execute something
pub trait IntelligentProgram {
    /// mutably executes the program
    /// might modify itself or adjust internal memory
    fn mut_execute(&mut self, input: &ProgramInputs) -> ProgramOutputs;

    /// executed the program, without changing it's state
    fn execute(&self, input: &ProgramInputs) -> ProgramOutputs;
}

/// Manages multiple program inputs
pub struct ProgramInputs {
    list: Vec<Box<ProgramInput>>,
}

/// A single program input that represent multiple internal inputs
pub trait ProgramInput {
    fn len(&self) -> usize;
    fn read(&self, i: usize) -> f64;
}

impl ProgramInputs {
    /// Creates a new empty input list
    pub fn new() -> Self {
        ProgramInputs {
            list: Vec::new(),
        }
    }

    /// Adds a new input to the list
    ///
    /// * `input` - An object that is able to act as an input
    pub fn add(mut self, input: Box<ProgramInput>) -> Self {
        self.list.push(input);
        self
    }

    /// Returns the length of the vector
    pub fn len(&self) -> usize {
        self.list.len()
    }

    /// Returns the input at given location
    pub fn get(&self, i: usize) -> &Box<ProgramInput> {
        &self.list[i]
    }
}

impl IntoIterator for ProgramInputs {
    type Item = Box<ProgramInput>;
    type IntoIter = ::std::vec::IntoIter<Box<ProgramInput>>;

    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

/// A simple form of an input that simply holds one value
pub struct SimpleInput {
    value: f64,
}

impl SimpleInput {
    pub fn new(value: f64) -> Self {
        SimpleInput {
            value: value,
        }
    }
}

impl ProgramInput for SimpleInput {
    fn len(&self) -> usize {
        1
    }

    fn read(&self, i: usize) -> f64 {
        self.value
    }
}

pub struct ProgramOutputs;

