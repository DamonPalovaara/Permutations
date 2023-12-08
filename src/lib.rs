use itertools::Itertools;

pub struct Permutation<const N: usize> {
    // Box to mitigate stack exhaustion for large values of N
    map: Box<[u32; N]>,
}

impl<const N: usize> Permutation<N> {
    pub fn new(map: [u32; N]) -> Self {
        let mut map = map.clone();
        (0..N).for_each(|index| map[index] -= 1);
        Self { map: Box::new(map) }
    }

    pub fn identity() -> Self {
        let mut map = Box::new([0; N]);
        (0..N).for_each(|index| map[index] = index as u32);
        Self { map }
    }

    pub fn rotation() -> Self {
        let mut map = Box::new([0; N]);
        (0..N)
            .map(|index| (index + 1) % N)
            .enumerate()
            .for_each(|(index, value)| map[index] = value as u32);
        Self { map }
    }

    // This is takes a 1 indexed number and returns a 1 indexed number
    // Internally it gets converted to 0 indexed
    pub fn apply(&self, n: u32) -> u32 {
        self.map[(n - 1) as usize] + 1
    }

    pub fn compose(&self, other: &Self) -> Self {
        let mut map = Box::new([0; N]);
        (0..N)
            .map(|index| other.map[index])
            .map(|index| self.map[index as usize])
            .enumerate()
            .for_each(|(index, x)| map[index] = x);

        Self { map }
    }

    pub fn first_cycle_length(&self) -> u32 {
        let mut next = self.map[0];
        let mut count = 0;
        while next != 0 {
            count += 1;
            next = self.map[next as usize];
        }
        count + 1
    }

    // Displays it in cycle notation
    pub fn display(&self) {
        let mut taken = Taken::new(N);
        let mut next = 0;
        taken.take(0);
        let mut cycle = Vec::new();
        loop {
            loop {
                cycle.push(next + 1);
                next = self.map[next as usize];
                if taken.is_taken(next) {
                    break;
                }
                taken.take(next);
            }
            print!("({})", cycle.iter().join(" "));
            cycle.clear();
            let next_cycle = taken.next();
            match next_cycle {
                Some(next_cycle) => {
                    next = next_cycle;
                    taken.take(next_cycle)
                }
                None => break,
            }
        }
        println!();
    }
}

// Helper structure for displaying cycle notation
struct Taken(Vec<bool>);

impl Taken {
    fn new(size: usize) -> Self {
        Self(vec![false; size])
    }

    fn take(&mut self, value: u32) {
        self.0[value as usize] = true
    }

    fn is_taken(&self, value: u32) -> bool {
        self.0[value as usize]
    }

    fn next(&mut self) -> Option<u32> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, is_taken)| !(*is_taken))
            .map(|(index, _)| index as u32)
            .next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_identity() {
        let rotation = Permutation::<4>::identity();
        assert_eq!(rotation.apply(1), 1);
        assert_eq!(rotation.apply(2), 2);
        assert_eq!(rotation.apply(3), 3);
        assert_eq!(rotation.apply(4), 4);
    }

    #[test]
    fn test_apply_rotation() {
        let rotation = Permutation::<4>::rotation();
        assert_eq!(rotation.apply(1), 2);
        assert_eq!(rotation.apply(2), 3);
        assert_eq!(rotation.apply(3), 4);
        assert_eq!(rotation.apply(4), 1);
    }

    #[test]
    fn test_apply_flip() {
        let flip = Permutation::<5>::new([1, 5, 4, 3, 2]);
        assert_eq!(flip.apply(1), 1);
        assert_eq!(flip.apply(2), 5);
        assert_eq!(flip.apply(3), 4);
        assert_eq!(flip.apply(4), 3);
        assert_eq!(flip.apply(5), 2);
    }

    #[test]
    fn test_double_rotation() {
        let rotation = Permutation::<4>::rotation();
        let double_rotation = rotation.compose(&rotation);
        assert_eq!(double_rotation.apply(1), 3);
        assert_eq!(double_rotation.apply(2), 4);
        assert_eq!(double_rotation.apply(3), 1);
        assert_eq!(double_rotation.apply(4), 2);
    }
}
