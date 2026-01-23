#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct State {
    pub(crate) a: usize,
    pub(crate) b: usize,
    pub(crate) c: usize,
    pub(crate) pointer: usize,
    pub(crate) program: Vec<usize>,
    pub(crate) output: Vec<usize>,
}

impl State {
    pub(crate) fn with_a(&self, a: usize) -> State {
        let mut new_state = self.clone();
        new_state.a = a;
        new_state
    }

    pub(crate) fn with_b(&self, b: usize) -> State {
        let mut new_state = self.clone();
        new_state.b = b;
        new_state
    }

    pub(crate) fn with_c(&self, c: usize) -> State {
        let mut new_state = self.clone();
        new_state.c = c;
        new_state
    }

    pub(crate) fn advance_pointer(&self, n: usize) -> State {
        let mut new_state = self.clone();
        new_state.pointer += n;
        new_state
    }

    pub(crate) fn with_pointer(&self, pointer: usize) -> State {
        let mut new_state = self.clone();
        new_state.pointer = pointer;
        new_state
    }

    pub(crate) fn append_output(&self, value: usize) -> State {
        let mut new_state = self.clone();
        new_state.output.push(value);
        new_state
    }
}
