use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use aoc::runner::{Runner, output};

const HALLWAY: usize = 11;
const ROOM_LOCATIONS: [usize; 4] = [2, 4, 6, 8];
const ROOMS: usize = ROOM_LOCATIONS.len();
const TARGETS: [Amphipod; ROOMS] = [
    Amphipod::Amber,
    Amphipod::Bronze,
    Amphipod::Copper,
    Amphipod::Desert,
];

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
        for (row, line) in puzlib::read_lines(&self.input).into_iter().enumerate() {
            let mut idx = 0;
            for ch in line.chars() {
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
        output(solve(Burrow::new(&self.rooms)))
    }

    fn part2(&mut self) -> String {
        let burrow = Burrow::new(&self.rooms).expand();
        output(solve(burrow))
    }
}

fn solve<const N: usize>(burrow: Burrow<N>) -> usize {
    let mut stack = BinaryHeap::from([Reverse((0, burrow))]);
    let mut costs = HashMap::new();

    while let Some(Reverse((cost, burrow))) = stack.pop() {
        if burrow.is_finished() {
            return cost;
        }
        for (next_cost, next_state) in burrow.moves() {
            let cost = cost + next_cost;
            let energy = costs.entry(next_state).or_insert(usize::MAX);
            if cost < *energy {
                *energy = cost;
                stack.push(Reverse((cost, next_state)));
            }
        }
    }
    0
}

fn room_is_available<const N: usize>(
    room: &[Option<Amphipod>; N],
    target: Amphipod,
) -> Option<usize> {
    let mut top_available = N - 1;
    for idx in (0..N).rev() {
        if let Some(a) = room[idx] {
            if a == target && idx > 0 {
                top_available -= 1;
            } else {
                return None;
            }
        }
    }
    Some(top_available)
}

fn peek<const N: usize>(room: &[Option<Amphipod>; N]) -> Option<(usize, Amphipod)> {
    for (idx, amph) in room.iter().enumerate() {
        if let Some(a) = amph {
            return Some((idx, *a));
        }
    }
    None
}

fn room_energy(room: &[Option<Amphipod>]) -> usize {
    room.iter().fold(0, |acc, r| {
        acc + if let Some(a) = r { a.energy() } else { 0 }
    })
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Burrow<const N: usize> {
    rooms: [[Option<Amphipod>; N]; ROOMS],
    hallway: [Option<Amphipod>; HALLWAY],
}

impl<const N: usize> PartialOrd for Burrow<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<const N: usize> Ord for Burrow<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let expected_energies = Self::complete()
            .rooms
            .iter()
            .map(|r| room_energy(r))
            .collect::<Vec<_>>();
        let self_energies = self
            .rooms
            .iter()
            .map(|r| room_energy(r))
            .zip(expected_energies.iter())
            .map(|(a, b)| a.max(*b) - a.min(*b))
            .collect::<Vec<_>>();
        let other_energies = other
            .rooms
            .iter()
            .map(|r| room_energy(r))
            .zip(expected_energies.iter())
            .map(|(a, b)| a.max(*b) - a.min(*b))
            .collect::<Vec<_>>();
        match self_energies.cmp(&other_energies) {
            std::cmp::Ordering::Equal => {
                room_energy(&self.hallway).cmp(&room_energy(&other.hallway))
            }
            ord => ord,
        }
    }
}

impl<const N: usize> Burrow<N> {
    fn new(rooms: &[[Amphipod; N]; ROOMS]) -> Self {
        let rooms = rooms
            .iter()
            .map(|room| {
                room.iter()
                    .map(|a| Some(*a))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            rooms,
            hallway: [None; HALLWAY],
        }
    }

    fn complete() -> Self {
        Self {
            hallway: [None; HALLWAY],
            rooms: (0..ROOMS)
                .map(|idx| [Some(TARGETS[idx]); N])
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn has_path(&self, start: usize, end: usize) -> bool {
        self.hallway[start..=end].iter().all(|loc| loc.is_none())
    }

    fn is_finished(&self) -> bool {
        self == &Self::complete()
    }

    fn moves(&self) -> Vec<(usize, Self)> {
        let mut moves = vec![];

        // Check if you can move an amphipod from the hallway to the room.
        for (hall_spot, state) in self.hallway.iter().enumerate() {
            if ROOM_LOCATIONS.contains(&hall_spot) {
                // Should never had an amphipod at the room entrance
                assert!(self.hallway[hall_spot].is_none());
                continue;
            }
            if let Some(amphipod) = state {
                let goal = ROOM_LOCATIONS[*amphipod as usize];
                let (start, end, hallway_dist) = if hall_spot < goal {
                    (hall_spot + 1, goal, goal - hall_spot)
                } else {
                    (goal, hall_spot - 1, hall_spot - goal)
                };
                let mut burrow = *self;
                let hallway = &mut burrow.hallway;
                let room = burrow.rooms.get_mut(*amphipod as usize).unwrap();
                if let Some(depth) = room_is_available(room, TARGETS[*amphipod as usize])
                    && self.has_path(start, end)
                {
                    room[depth] = hallway[hall_spot].take();
                    moves.push(((hallway_dist + depth + 1) * amphipod.energy(), burrow));
                }
            }
        }

        // Check move amphipods from the rooms
        for (room_idx, &start_location) in ROOM_LOCATIONS.iter().enumerate() {
            if let Some((depth, amphipod)) = peek(&self.rooms[room_idx]) {
                // Check if the amphipod is already in the right spot.
                if room_is_available(&self.rooms[room_idx], TARGETS[room_idx]).is_some() {
                    continue;
                }
                // Check to the left
                for target in 0..start_location {
                    self.move_to_hall_or_room(
                        &mut moves,
                        start_location,
                        target,
                        room_idx,
                        amphipod,
                        depth,
                    );
                }
                // Check to the right
                for target in start_location + 1..HALLWAY {
                    self.move_to_hall_or_room(
                        &mut moves,
                        start_location,
                        target,
                        room_idx,
                        amphipod,
                        depth,
                    );
                }
            }
        }
        moves
    }

    fn move_to_hall_or_room(
        &self,
        moves: &mut Vec<(usize, Self)>,
        start: usize,
        target: usize,
        source: usize,
        amphipod: Amphipod,
        depth: usize,
    ) {
        let goal = ROOM_LOCATIONS[amphipod as usize];
        // Skip rooms of other amphipods
        if self.hallway[target].is_some() || (ROOM_LOCATIONS.contains(&target) && target != goal) {
            return;
        }
        if !self.has_path(start.min(target), start.max(target)) {
            return;
        }
        let mut burrow = *self;
        let hallway = &mut burrow.hallway;
        let hallway_dist = start.max(target) - start.min(target);
        let source_room = &mut burrow.rooms[source];
        // Check if we can move directly to the target room.
        if target == goal
            && let Some(depth2) =
                room_is_available(&self.rooms[amphipod as usize], TARGETS[amphipod as usize])
        {
            burrow.rooms[amphipod as usize][depth2] = source_room[depth].take();
            moves.push((
                (hallway_dist + depth + depth2 + 2) * amphipod.energy(),
                burrow,
            ))
        } else if target != goal {
            hallway[target] = source_room[depth].take();
            moves.push(((hallway_dist + depth + 1) * amphipod.energy(), burrow))
        };
    }
}

impl Burrow<2> {
    fn expand(&self) -> Burrow<4> {
        let hidden = [
            [Amphipod::Desert, Amphipod::Desert],
            [Amphipod::Copper, Amphipod::Bronze],
            [Amphipod::Bronze, Amphipod::Amber],
            [Amphipod::Amber, Amphipod::Copper],
        ];
        let rooms = self
            .rooms
            .iter()
            .enumerate()
            .map(|(idx, r)| [r[0], Some(hidden[idx][0]), Some(hidden[idx][1]), r[1]])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Burrow {
            rooms,
            hallway: [None; HALLWAY],
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_is_available() {
        let expected = Some(0);
        let actual = room_is_available(&[None, Some(Amphipod::Amber)], Amphipod::Amber);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_room_is_available_empty() {
        let expected = Some(1);
        let actual = room_is_available(&[None, None], Amphipod::Amber);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_room_is_unavailable2() {
        let expected = None;
        let actual = room_is_available(
            &[None, None, Some(Amphipod::Bronze), Some(Amphipod::Bronze)],
            Amphipod::Desert,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_room_is_unavailable() {
        let expected = None;
        let actual = room_is_available(
            &[Some(Amphipod::Amber), Some(Amphipod::Copper)],
            Amphipod::Amber,
        );
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_peek() {
        let expected = Some((1, Amphipod::Desert));
        let actual = peek(&[None, Some(Amphipod::Desert), Some(Amphipod::Amber)]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_no_peek() {
        let expected = None;
        let actual = peek(&[None, None]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example() {
        use Amphipod::*;
        let rooms = [
            [Bronze, Amber],
            [Copper, Desert],
            [Bronze, Copper],
            [Desert, Amber],
        ];
        let burrow = Burrow::new(&rooms);
        let expected = 12521;
        let actual = solve(burrow);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example2() {
        use Amphipod::*;
        let burrow = Burrow::new(&[
            [Bronze, Amber],
            [Copper, Desert],
            [Bronze, Copper],
            [Desert, Amber],
        ])
        .expand();
        let expected = 44169;
        let actual = solve(burrow);
        assert_eq!(expected, actual);
    }
}
