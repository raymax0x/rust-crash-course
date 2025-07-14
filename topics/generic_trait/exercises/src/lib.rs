pub trait Iterator<T> {
    fn next(&mut self) -> Option<&T>;
}

pub struct TupleIter<T> {
    pub tuple: (T, T, T),
    pub next: usize,
}

pub struct VecIter<T> {
    pub vec: Vec<T>,
    pub next: usize,
}

impl<T> Iterator<T> for TupleIter<T> {
    fn next(&mut self) -> Option<&T> {
        match self.next {
            0 => {
                self.next += 1;
                Some(&self.tuple.0)
            }
            1 => {
                self.next += 1;
                Some(&self.tuple.1)
            }
            2 => {
                self.next += 1;
                Some(&self.tuple.2)
            }
            _ => None,
        }
    }
}
impl<T> Iterator<T> for VecIter<T> {
    fn next(&mut self) -> Option<&T> {
        if let Some(val) = self.vec.get(self.next) {
            self.next += 1;
            Some(val)
        } else {
            None
        }
    }
}
