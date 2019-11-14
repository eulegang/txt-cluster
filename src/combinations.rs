pub struct Combinations<'a, T> {
    slice: &'a [T],
    tortise: usize,
    hare: usize,
}

pub fn combinations<'a, T>(slice: &'a [T]) -> Combinations<'a, T> {
    let tortise = 0;
    let hare = 1;
    Combinations {
        slice,
        tortise,
        hare,
    }
}

impl<'a, T> Iterator for Combinations<'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<(&'a T, &'a T)> {
        if self.tortise < self.slice.len() {
            if self.hare < self.slice.len() {
                let first = &self.slice[self.tortise];
                let second = &self.slice[self.hare];
                self.hare += 1;
                Some((first, second))
            } else {
                self.tortise += 1;
                self.hare = self.tortise + 1;
                if self.tortise < self.slice.len() && self.hare < self.slice.len() {
                    let first = &self.slice[self.tortise];
                    let second = &self.slice[self.hare];
                    self.hare += 1;
                    Some((first, second))
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

#[test]
fn test_combinations() {
    let vec = vec![1, 2, 3, 4, 5];
    let combs = combinations(&vec).collect::<Vec<(&i32, &i32)>>();

    assert_eq!(
        combs,
        vec![
            (&1, &2),
            (&1, &3),
            (&1, &4),
            (&1, &5),
            (&2, &3),
            (&2, &4),
            (&2, &5),
            (&3, &4),
            (&3, &5),
            (&4, &5),
        ]
    )
}
