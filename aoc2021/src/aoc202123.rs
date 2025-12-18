use aoc::runner::{Runner, output};

const ROOMS: usize = 4;
const HALLWAY: usize = 11;
const ROOM_LOCATIONS: [usize; ROOMS] = [3, 5, 7, 9];

#[derive(Default)]
pub struct AocDay {
    pub(crate) input: String,
    rooms: [[Amphipod; 2]; ROOMS],
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2021, 23)
    }

    fn parse(&mut self) {
        for line in puzlib::read_lines(&self.input) {
            let mut idx = 0;
            for (row, ch) in line.chars().enumerate() {
                if let Some(amph) = match ch {
                    'A' => Some(Amphipod::Amber),
                    'B' => Some(Amphipod::Bronze),
                    'C' => Some(Amphipod::Copper),
                    'D' => Some(Amphipod::Desert),
                    _ => None,
                } {
                    self.rooms[idx][row - 2] = amph;
                    idx += 1;
                };
            }
        }
    }

    fn part1(&mut self) -> String {
        let energy = exit_energy(&self.rooms) + entry_energy(2) + solve(&self.rooms);
        output(energy)
    }

    fn part2(&mut self) -> String {
        output("Unsolved")
    }
}

fn exit_energy<const N: usize>(rooms: &[[Amphipod; N]]) -> usize {
    // Total energy needed to get to the hallway.
    rooms
        .iter()
        .flat_map(|room| {
            room.iter()
                .enumerate()
                .map(|(depth, amph)| (depth + 1) * amph.energy())
        })
        .sum()
}

fn entry_energy(depth: usize) -> usize {
    // Total energy to get from the hallway into the rooms
    (depth * (depth + 1) / 2)
        * (Amphipod::Amber.energy()
            + Amphipod::Bronze.energy()
            + Amphipod::Copper.energy()
            + Amphipod::Desert.energy())
}

fn solve<const N: usize>(rooms: &[[Amphipod; N]; ROOMS]) -> usize {
    let mut stack = vec![(Burrow::new(rooms), 0)];
    let mut min_energy = usize::MAX;

    while let Some((burrow, cost)) = stack.pop() {
        for t in (0..4).rev() {
            if let Some((amph, target, moves)) = burrow.moves(t) {
                for (mask, price) in moves {
                    let mut burrow = burrow;
                    let cost = cost + price;
                    if cost < min_energy {
                        burrow.commit(amph, target, mask);
                        if burrow.hallway.is_empty() {
                            min_energy = cost;
                        } else {
                            stack.push((burrow, cost));
                        }
                    }
                }
            }
        }
    }
    min_energy
}

#[derive(Debug, Default, Copy, Clone)]
struct Hallway {
    amber: u16,
    bronze: u16,
    copper: u16,
    desert: u16,
}

impl Hallway {
    fn flatten(&self) -> u16 {
        self.amber | self.bronze | self.copper | self.desert
    }

    fn is_empty(&self) -> bool {
        self.flatten() == 0
    }
}

#[derive(Debug, Clone, Copy)]
struct Burrow<const N: usize> {
    rooms: [Room<N>; ROOMS],
    hallway: Hallway,
}

impl<const N: usize> Burrow<N> {
    fn new(rooms: &[[Amphipod; N]; ROOMS]) -> Self {
        let mut r: Vec<Room<N>> = vec![];
        for room in rooms {
            r.push(Room {
                occupants: room
                    .iter()
                    .map(|&a| RoomState::Occupied(a))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                position: 0,
            });
        }
        Self {
            rooms: r.try_into().unwrap(),
            hallway: Hallway::default(),
        }
    }
    fn has_path(&self, start: usize, dist: usize) -> bool {
        const HALLWAY_SLOTS: usize = HALLWAY - ROOMS;
        const EXTENSION: usize = HALLWAY - ROOM_LOCATIONS[ROOMS - 1];
        const MAX_STEPS: usize = HALLWAY_SLOTS - EXTENSION;
        if dist == 0 {
            true
        } else {
            let mask = ((2 << (dist - 1)) - 1) << (MAX_STEPS - dist - start);
            self.hallway.flatten() & mask == 0
        }
    }

    fn commit(&mut self, amph: Amphipod, target: usize, mask: u16) {
        self.rooms[target].step_out();

        match amph {
            Amphipod::Amber => {
                self.hallway.amber |= mask;
            }
            Amphipod::Bronze => {
                self.hallway.bronze |= mask;
            }
            Amphipod::Copper => {
                self.hallway.copper |= mask;
            }
            Amphipod::Desert => {
                self.hallway.desert |= mask;
            }
        }

        if self.rooms[0].is_empty() {
            self.hallway.amber = 0;
        }
        if self.rooms[1].is_empty() {
            self.hallway.bronze = 0;
        }
        if self.rooms[2].is_empty() {
            self.hallway.copper = 0;
        }
        if self.rooms[3].is_empty() {
            self.hallway.desert = 0;
        }
    }

    fn moves(&self, pos: usize) -> Option<State> {
        let amph = self.rooms[pos].peek()?;

        const COSTS: [usize; 7] = [2, 2, 4, 4, 4, 2, 2];
        let mut moves = Vec::with_capacity(7);
        let hallway = self.hallway.flatten();
        let energy = amph.energy();
        let target = amph as usize;
        let (left, right) = if pos < target {
            (pos, target)
        } else {
            (target, pos)
        };

        let dist = right - left;
        let base_cost = dist * 2;
        let path_available = self.has_path(left, dist);

        if path_available && self.rooms[target].is_empty() {
            moves.push((0, base_cost * energy));
        } else {
            if path_available || pos < target {
                moves.extend(
                    (0..left + 2)
                        .rev()
                        .map(|offset| (1 << (6 - offset), COSTS[offset]))
                        .scan(0, |acc, (mask, weight)| {
                            *acc += weight;
                            let cost = if *acc == weight { 2 } else { *acc };
                            (mask & hallway == 0).then(|| (mask, (base_cost + cost) * energy))
                        }),
                );
            }
            if path_available || pos > target {
                moves.extend(
                    (right + 2..7)
                        .map(|offset| (1 << (6 - offset), COSTS[offset]))
                        .scan(0, |acc, (mask, weight)| {
                            *acc += weight;
                            let cost = if *acc == weight { 2 } else { *acc };
                            (mask & hallway == 0).then(|| (mask, (base_cost + cost) * energy))
                        }),
                );
            }
        };
        Some((amph, pos, moves))
    }
}

type State = (Amphipod, usize, Vec<(u16, usize)>);

#[derive(Default, Debug, Copy, Clone)]
enum RoomState {
    Occupied(Amphipod),
    #[default]
    Empty,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum Amphipod {
    #[default]
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn energy(&self) -> usize {
        10_usize.pow(*self as u32)
    }
}

#[derive(Debug, Copy, Clone)]
struct Room<const N: usize> {
    occupants: [RoomState; N],
    position: usize,
}

impl<const N: usize> Room<N> {
    fn peek(&self) -> Option<Amphipod> {
        if self.position < self.occupants.len()
            && let RoomState::Occupied(amph) = self.occupants[self.position]
        {
            Some(amph)
        } else {
            None
        }
    }

    fn step_out(&mut self) {
        if self.position < N {
            self.occupants[self.position] = RoomState::Empty;
            self.position += 1;
        }
    }

    fn is_empty(&self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        use Amphipod::*;
        let mut day = AocDay {
            input: "".into(),
            rooms: [
                [Bronze, Amber],
                [Copper, Desert],
                [Bronze, Copper],
                [Desert, Amber],
            ],
        };
        let expected = 12521;
        let actual = day.part1().parse().unwrap();
        assert_eq!(expected, actual);
    }
}
