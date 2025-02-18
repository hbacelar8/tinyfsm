#![no_std]

use core::fmt;

/// Trait for the state behavior
pub trait StateBehavior {
    type State: Clone + Copy + PartialEq + fmt::Debug;
    type Event: Clone + Copy + PartialEq + fmt::Debug;
    type Context: Default + fmt::Debug;

    /// Handle an event and return next state (if a transition occurs)
    fn handle(&self, event: &Self::Event, _context: &mut Self::Context) -> Option<Self::State>;

    /// State entry
    fn enter(&self, _context: &mut Self::Context) {}

    /// State exit
    fn exit(&self, _context: &mut Self::Context) {}
}

/// # RustFSM
///
/// A full static Rust finite state machine library.
///
/// ## Usage
///
/// The `rustfsm` macro takes as input the state machine's name, list of
/// states, list of events and context.
///
/// The state machine's name can be just an ident if no other member is desired
/// to the struct:
///
/// ```rust,ignore
/// use rustfsm::{rustfsm, StateBehavior};
///
/// rustfsm!(
///     FooName,
///     FooStates {
///         FooStateA,
///         FooStateB,
///     },
///     Events {
///         FooEvent1,
///         FooEvent2,
///     },
///     Context {
///         context_foo_data: u8 = 0,
///         context_boo_data: bool = true,
///     }
/// );
/// ```
///
/// The state machine's name can also be a struct with default values if data
/// other than the cotext is desired:
///
/// ```rust,ignore
/// use rustfsm::{rustfsm, StateBehavior};
///
/// rustfsm!(
///     FooName {
///         foo_data: u16 = 0,
///         boo_data: boo = false,
///     },
///     FooStates {
///         FooStateA,
///         FooStateB,
///     },
///     Events {
///         FooEvent1,
///         FooEvent2,
///     },
///     Context {
///         foo_data: u8 = 0,
///         boo_data: bool = true,
///     }
/// );
/// ```
#[macro_export]
macro_rules! rustfsm {
    // Case 1: With additional members for the state machine struct
    (
        $state_machine_name:ident {
            $($member_field:ident: $member_field_type:ty = $member_default:expr),* $(,)?
        },
        $state_type:ident {
            $first_state:ident $(($($first_state_data:ty),*))?,
            $($remaining_states:ident $(($($remaining_state_data:ty),*))? ),* $(,)?
        },
        $event_type:ident {
            $($event_variant:ident $(($($event_variant_data:ty),*))? ),* $(,)?
        },
        $context_type:ident {
            $($context_field:ident: $context_field_type:ty = $context_default:expr),* $(,)?
        }
    ) => {
        rustfsm!(@generate $state_machine_name, $state_type, $event_type, $context_type,
            states { $first_state $(($($first_state_data),*))?, $($remaining_states $(($($remaining_state_data),*))? ),* },
            events { $($event_variant $(($($event_variant_data),*))? ),* },
            context { $($context_field: $context_field_type = $context_default),* },
            members { $($member_field: $member_field_type = $member_default),* },
            initial_state = $first_state
        );
    };

    // Case 1: Without additional members for the state machine struct
    (
        $state_machine_name:ident,
        $state_type:ident {
            $first_state:ident $(($($first_state_data:ty),*))?,
            $($remaining_states:ident $(($($remaining_state_data:ty),*))? ),* $(,)?
        },
        $event_type:ident {
            $($event_variant:ident $(($($event_variant_data:ty),*))? ),* $(,)?
        },
        $context_type:ident {
            $($context_field:ident: $context_field_type:ty = $context_default:expr),* $(,)?
        }
    ) => {
        rustfsm!(@generate $state_machine_name, $state_type, $event_type, $context_type,
            states { $first_state $(($($first_state_data),*))?, $($remaining_states $(($($remaining_state_data),*))? ),* },
            events { $($event_variant $(($($event_variant_data),*))? ),* },
            context { $($context_field: $context_field_type = $context_default),* },
            members { },
            initial_state = $first_state
        );
    };

    // Internal implementation for generating the state machine
    (
        @generate $state_machine_name:ident, $state_type:ident, $event_type:ident, $context_type:ident,
        states { $($state_variant:ident $(($($state_variant_data:ty),*))? ),* },
        events { $($event_variant:ident $(($($event_variant_data:ty),*))? ),* },
        context { $($context_field:ident: $context_field_type:ty = $context_default:expr),* },
        members { $($member_field:ident: $member_field_type:ty = $member_default:expr),* },
        initial_state = $initial_state:ident
    ) => {
        /// State machine state type.
        ///
        /// - The first state in the list is the state machine's initial state.
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum $state_type {
            $(
                $state_variant $(($($state_variant_data),*))?
            ),*
        }

        /// State machine event type.
        ///
        /// List of all events handled by the state machine.
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum $event_type {
            $(
                $event_variant $(($($event_variant_data),*))?
            ),*
        }

        /// State machine context data struct.
        ///
        /// The Context struct holds all the state's machine data common and
        /// accessible to every state.
        #[derive(Debug)]
        pub struct $context_type {
            $(
                $context_field: $context_field_type,
            )*
        }

        // Implement Default trait for the Context.
        impl Default for $context_type {
            fn default() -> Self {
                Self {
                    $(
                        $context_field: $context_default,
                    )*
                }
            }
        }

        /// State machine struct.
        pub struct $state_machine_name {
            current_state: $state_type,
            context: $context_type,
            $(
                $member_field: $member_field_type,
            )*
        }

        impl $state_machine_name {
            /// Create a new state machine.
            pub fn new() -> Self {
                Self {
                    current_state: $state_type::$initial_state,
                    context: $context_type::default(),
                    $(
                        $member_field: $member_default,
                    )*
                }
            }

            /// Transition to a new state.
            pub fn transition(&mut self, new_state: $state_type) {
                self.current_state.exit(&mut self.context);
                self.current_state = new_state;
                self.current_state.enter(&mut self.context);
            }

            /// Force transition to a new state without calls to respectives
            /// `enter` and `exit` functions.
            pub fn force_state(&mut self, new_state: $state_type) {
                self.current_state = new_state;
            }

            /// Get a copy of the current state
            pub fn get_current_state(&self) -> $state_type {
                self.current_state
            }

            /// Handle event and transition if necessary.
            fn handle(&mut self, event: $event_type) {
                match self.current_state.handle(&event, &mut self.context) {
                    Some(next_state) => {
                        self.current_state.exit(&mut self.context);
                        self.current_state = next_state;
                        self.current_state.enter(&mut self.context);
                    }
                    None => (),
                }
            }
        }
    };
}
