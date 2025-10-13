// Crabby's Challenge: Shape His
// Decisions

// Crabby is on a grand adventure, switching between three
// main roles :

// · Fighting.
// · Collecting
// · Defending.

// Your challenge is to:

// · Create an Enum for Crabby's various states (Fighting,
// Collecting, Defending).
// · For Collecting, have Crabby store the number of
// treasures he gathers.
// · Use match to represent the story outcome of each
// state.

enum CrabbyStates {
    Fighting,
    Collecting(u32), // number of treasures
    Defending,
}

impl CrabbyStates {
    fn state_represent(&self) {
        match self {
            CrabbyStates::Fighting => println!("Crabby is fighting"),
            CrabbyStates::Collecting(amount) => println!("Crabby is collecting {}", amount),
            CrabbyStates::Defending => println!("Crabby is defending"),
        }
    }
}

fn main() {
    let fighting = CrabbyStates::Fighting;
    let collecting = CrabbyStates::Collecting(15);
    let defending = CrabbyStates::Defending;

    fighting.state_represent();
    collecting.state_represent();
    defending.state_represent();
}
