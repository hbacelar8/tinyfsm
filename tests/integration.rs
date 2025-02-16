use tinyfsm::*;

#[derive(Clone, Copy, PartialEq, Debug)]
enum MarioConsumables {
    Mushroom,
    Flower,
    Feather,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum MarioSize {
    Small,
    Large,
}

state_machine!(
    Mario,
    MarioStates {
        DeadMario,
        SmallMario,
        SuperMario,
        FireMario,
        CapeMario
    },
    Events { GetConsumable(MarioConsumables), Hit },
    Context {
        size: MarioSize = MarioSize::Small,
        alive: bool = true
    }
);

impl StateBehavior for MarioStates {
    type State = MarioStates;
    type Event = Events;
    type Context = Context;

    fn enter(&self, _context: &mut Self::Context) {
        use MarioSize::*;
        use MarioStates::*;
        match self {
            DeadMario => _context.alive = false,
            SmallMario => _context.size = Small,
            _ => _context.size = Large,
        }
    }

    fn handle(&self, event: &Self::Event, _context: &mut Self::Context) -> Option<Self::State> {
        use Events::*;
        use MarioConsumables::*;
        use MarioStates::*;
        match (self, event) {
            (SmallMario, GetConsumable(item)) => match item {
                Mushroom => Some(SuperMario),
                Flower => Some(FireMario),
                Feather => Some(CapeMario),
            },
            (SuperMario, GetConsumable(item)) => match item {
                Flower => Some(FireMario),
                Feather => Some(CapeMario),
                _ => None,
            },
            (FireMario, GetConsumable(item)) => match item {
                Feather => Some(CapeMario),
                _ => None,
            },
            (CapeMario, GetConsumable(item)) => match item {
                Flower => Some(FireMario),
                _ => None,
            },
            (SmallMario, Hit) => Some(DeadMario),
            (SuperMario, Hit) => Some(SmallMario),
            (FireMario, Hit) => Some(SmallMario),
            (CapeMario, Hit) => Some(SmallMario),
            (DeadMario, _) => None,
        }
    }
}

#[test]
fn integration_test() {
    let mut mario = Mario::new(
        MarioStates::SmallMario,
        Context {
            size: MarioSize::Small,
            alive: true,
        },
    );

    // Initial state
    assert_eq!(mario.current_state, MarioStates::SmallMario);
    assert_eq!(mario.context.size, MarioSize::Small);
    assert!(mario.context.alive);

    // Get a mushroom
    mario.handle(Events::GetConsumable(MarioConsumables::Mushroom));
    assert_eq!(mario.current_state, MarioStates::SuperMario);
    assert_eq!(mario.context.size, MarioSize::Large);
    assert!(mario.context.alive);

    // Get a flower
    mario.handle(Events::GetConsumable(MarioConsumables::Flower));
    assert_eq!(mario.current_state, MarioStates::FireMario);
    assert_eq!(mario.context.size, MarioSize::Large);
    assert!(mario.context.alive);

    // Get a feather
    mario.handle(Events::GetConsumable(MarioConsumables::Feather));
    assert_eq!(mario.current_state, MarioStates::CapeMario);
    assert_eq!(mario.context.size, MarioSize::Large);
    assert!(mario.context.alive);

    // Get a hit
    mario.handle(Events::Hit);
    assert_eq!(mario.current_state, MarioStates::SmallMario);
    assert_eq!(mario.context.size, MarioSize::Small);
    assert!(mario.context.alive);

    // Oh no
    mario.handle(Events::Hit);
    assert_eq!(mario.current_state, MarioStates::DeadMario);
    assert!(!mario.context.alive);
}
