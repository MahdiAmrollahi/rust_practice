// Topic: Typestates
//
// Summary:
//   An airline wants to reduce the amount of lost luggage by
//   ensuring luggage is properly tracked.
//
// Requirements:
// * Implement a luggage tracking system using the typestate pattern
// * Each piece of luggage has a tracking id
// * Luggage goes through multiple states at the airport:
//   * Check-in        (passenger gives luggage to airport)
//   * OnLoading       (luggage is loaded onto correct plane)
//   * Offloading      (luggage is taken off plane at destination)
//   * AwaitingPickup  (luggage is at destination waiting for passenger pickup)
//   * EndCustody      (luggage was picked up by passenger)
// Notes:
// * Optionally use generics for each state

#[derive(Debug)]
struct CheckIn;
#[derive(Debug)]
struct OnLoading;
#[derive(Debug)]
struct Offloading;
#[derive(Debug)]
struct AwaitingPickup;
#[derive(Debug)]
struct EndCustody;

#[derive(Debug)]
struct Luggage<State> {
    id: i64,
    state: State,
}

impl Luggage<CheckIn> {
    fn new(id: i64) -> Self {
        Self { id, state: CheckIn }
    }

    fn load(self) -> Luggage<OnLoading> {
        Luggage {
            id: self.id,
            state: OnLoading,
        }
    }
}

impl Luggage<OnLoading> {
    fn offload(self) -> Luggage<Offloading> {
        Luggage {
            id: self.id,
            state: Offloading,
        }
    }
}

impl Luggage<Offloading> {
    fn carousel(self) -> Luggage<AwaitingPickup> {
        Luggage {
            id: self.id,
            state: AwaitingPickup,
        }
    }
}

impl Luggage<AwaitingPickup> {
    fn pickup(self) -> Luggage<EndCustody> {
        Luggage {
            id: self.id,
            state: EndCustody,
        }
    }
}

fn main() {
    let luggage = Luggage::new(123);
    let loaded = luggage.load();
    let offloaded = loaded.offload();
    let awaiting = offloaded.carousel();
    let end = awaiting.pickup();
    println!("Luggage processing complete: {:?}", end);
}
