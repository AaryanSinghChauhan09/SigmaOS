extern crate alloc;
// Model-View-Intent (MVI) Reactive State Architecture for SigmaOS
// Implements a generic Store<S, A> with Reducers and Subscribers under `#![no_std]`.


use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::RefCell;

/// Trait defining state transition logic
pub trait Reducer<S, A> {
    /// Reduces previous state and action into a new state
    fn reduce(&self, state: &S, action: A) -> S;
}

/// Dynamic closure for receiving state notifications
pub type Subscriber<S> = Box<dyn Fn(&S)>;

/// Reactive State Store holding state S, processing actions A
pub struct Store<S, A> {
    state: RefCell<S>,
    reducer: Box<dyn Reducer<S, A>>,
    subscribers: RefCell<Vec<Subscriber<S>>>,
}

impl<S: Clone, A> Store<S, A> {
    /// Initialise a new Store with an initial state and a reducer
    pub fn new(initial_state: S, reducer: Box<dyn Reducer<S, A>>) -> Self {
        Self {
            state: RefCell::new(initial_state),
            reducer,
            subscribers: RefCell::new(Vec::new()),
        }
    }

    /// Retrieve a clone of the current state
    pub fn get_state(&self) -> S {
        self.state.borrow().clone()
    }

    /// Subscribe to state changes
    pub fn subscribe(&self, subscriber: Subscriber<S>) {
        self.subscribers.borrow_mut().push(subscriber);
    }

    /// Dispatch an action to trigger state transitions and notify subscribers
    pub fn dispatch(&self, action: A) {
        let next_state = self.reducer.reduce(&*self.state.borrow(), action);
        *self.state.borrow_mut() = next_state.clone();

        for subscriber in self.subscribers.borrow().iter() {
            subscriber(&next_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::rc::Rc;

    #[derive(Clone, Debug, PartialEq, Eq)]
    struct AppState {
        counter: i32,
        dark_mode: bool,
    }

    enum AppAction {
        Increment,
        Decrement,
        ToggleDarkMode,
    }

    struct AppReducer;

    impl Reducer<AppState, AppAction> for AppReducer {
        fn reduce(&self, state: &AppState, action: AppAction) -> AppState {
            match action {
                AppAction::Increment => AppState {
                    counter: state.counter + 1,
                    ..*state
                },
                AppAction::Decrement => AppState {
                    counter: state.counter - 1,
                    ..*state
                },
                AppAction::ToggleDarkMode => AppState {
                    dark_mode: !state.dark_mode,
                    ..*state
                },
            }
        }
    }

    #[test]
    fn test_store_transitions_and_subscriptions() {
        let initial_state = AppState {
            counter: 0,
            dark_mode: false,
        };
        let store = Store::new(initial_state, Box::new(AppReducer));

        let state_history = Rc::new(RefCell::new(Vec::new()));
        let state_history_clone = Rc::clone(&state_history);

        // Subscribe to changes
        store.subscribe(Box::new(move |state: &AppState| {
            state_history_clone.borrow_mut().push(state.clone());
        }));

        // Dispatch some actions
        store.dispatch(AppAction::Increment);
        store.dispatch(AppAction::ToggleDarkMode);
        store.dispatch(AppAction::Decrement);

        assert_eq!(store.get_state().counter, 0);
        assert_eq!(store.get_state().dark_mode, true);

        let history = state_history.borrow();
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].counter, 1);
        assert_eq!(history[1].dark_mode, true);
        assert_eq!(history[2].counter, 0);
    }
}
