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

#[macro_export]
macro_rules! state_machine {
    (
        $state_machine_name:ident,
        $state_type:ident {
            $($state_variant:ident $(($($state_variant_data:ty),*))? ),* $(,)?
        },
        $event_type:ident {
            $($event_variant:ident $(($($event_variant_data:ty),*))? ),* $(,)?
        },
        $context_type:ident {
            $($field_name:ident: $field_type:ty = $default_value:expr),* $(,)?
        }
    ) => {
        /// States enum
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum $state_type {
            $(
                $state_variant $(($($state_variant_data),*))?
            ),*
        }

        /// Events enum
        #[derive(Clone, Copy, PartialEq, Debug)]
        pub enum $event_type {
            $(
                $event_variant $(($($event_variant_data),*))?
            ),*
        }

        /// Context struct
        #[derive(Debug)]
        pub struct $context_type {
            $(
                $field_name: $field_type,
            )*
        }

        /// Implement Default trait for the Context
        impl Default for $context_type {
            fn default() -> Self {
                Self {
                    $(
                        $field_name: $default_value,
                    )*
                }
            }
        }

        /// State machine struct
        pub struct $state_machine_name {
            current_state: $state_type,
            context: $context_type,
        }

        impl $state_machine_name {
            /// Create a new state machine
            pub fn new(initial_state: $state_type, context: $context_type) -> Self {
                Self {
                    current_state: initial_state,
                    context,
                }
            }

            /// Handle an event on a state and transition if necessary
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
