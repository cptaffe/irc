
use std::iter::Peekable;

struct Scanner<T: Iterator> {
	input: Peekable<T>
}

impl<T: Iterator> Scanner<T> {
	fn new(p: Peekable<T>)->Scanner<T> {
		Scanner {
			input: p
		}
	}
}

trait State<T: Iterator> {
	fn next(&self, sc: &Scanner<T>)->Option<Box<State<T>>>;
}

struct StateMachine<T: Iterator> {
	state: Box<State<T>>,
	sc: Scanner<T>
}

impl<T: Iterator> StateMachine<T> {
	fn new(s: Box<State<T>>, sc: Scanner<T>)->StateMachine<T> {
		StateMachine {
			state: s,
			sc: sc
		}
	}

	fn run(mut self) {
		loop {
			self.state = match self.state.next(&self.sc) {
				Some(s) => s,
				None => return
			}
		}
	}
}

struct MsgState;

impl MsgState {
	fn new()->MsgState { MsgState }
}

impl<T: Iterator<Item=char>> State<T> for MsgState {
	fn next(&self, sc: &Scanner<T>)->Option<Box<State<T>>> {
		println!("msgstate!");
		Some(Box::new(TestState::new()))
	}
}

struct TestState;

impl TestState {
	fn new()->TestState { TestState }
}

impl<T: Iterator<Item=char>> State<T> for TestState {
	fn next(&self, sc: &Scanner<T>)->Option<Box<State<T>>> {
		println!("teststate!");
		None
	}
}

fn main() {
	StateMachine::new(Box::new(MsgState::new()), Scanner::new("hello".chars().peekable())).run();
}
