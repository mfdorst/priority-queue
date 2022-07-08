pub struct PriorityQueue<T, F: Fn(&T, &T) -> bool> {
    heap: Vec<T>,
    cmp: Box<F>,
}

impl<T: PartialOrd> PriorityQueue<T, fn(&T, &T) -> bool> {
    pub fn new(data: Vec<T>) -> Self {
        Self::with_ordering(data, |a, b| a < b)
    }
}

impl<T, F: Fn(&T, &T) -> bool> PriorityQueue<T, F> {
    pub fn with_ordering(data: Vec<T>, ordering: F) -> Self {
        let mut queue = Self {
            heap: data,
            cmp: Box::new(ordering),
        };
        queue.heapify();
        queue
    }

    pub fn take_front(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let min = self.heap.pop().unwrap();
        self.sift_down(0);
        Some(min)
    }

    pub fn insert(&mut self, element: T) {
        self.heap.push(element);
        let mut i = self.heap.len() - 1;
        // This will overflow if i = 0, but we don't care because we will exit
        let mut parent = (i.wrapping_sub(1)) / 2;
        while i != 0 && self.cmp(&self.heap[i], &self.heap[parent]) {
            self.heap.swap(i, parent);
            i = parent;
            parent = (i - 1) / 2;
        }
    }

    fn cmp(&self, a: &T, b: &T) -> bool {
        (self.cmp)(a, b)
    }

    fn heapify(&mut self) {
        for i in (0..self.heap.len()).rev() {
            self.sift_down(i);
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let mut left = i * 2 + 1;
        let mut right = i * 2 + 2;
        while left < self.heap.len() && self.cmp(&self.heap[left], &self.heap[i])
            || right < self.heap.len() && self.cmp(&self.heap[right], &self.heap[i])
        {
            let smallest = if right < self.heap.len() {
                if self.cmp(&self.heap[left], &self.heap[right]) {
                    left
                } else {
                    right
                }
            } else {
                left
            };
            self.heap.swap(i, smallest);
            i = smallest;
            left = i * 2 + 1;
            right = i * 2 + 2;
        }
    }
}

#[cfg(test)]
mod test {
    use crate::PriorityQueue;
    #[test]
    fn new() {
        let mut queue = PriorityQueue::new(vec![3, 2, 6, 5, 1, 4]);
        assert_eq!(queue.take_front(), Some(1));
        assert_eq!(queue.take_front(), Some(2));
        assert_eq!(queue.take_front(), Some(3));
        assert_eq!(queue.take_front(), Some(4));
        assert_eq!(queue.take_front(), Some(5));
        assert_eq!(queue.take_front(), Some(6));
        assert_eq!(queue.take_front(), None);
    }

    #[test]
    fn insert() {
        let mut queue = PriorityQueue::new(vec![1, 5, 9]);
        queue.insert(8);
        assert_eq!(queue.take_front(), Some(1));
        assert_eq!(queue.take_front(), Some(5));
        assert_eq!(queue.take_front(), Some(8));
        assert_eq!(queue.take_front(), Some(9));
        assert_eq!(queue.take_front(), None);
    }

    #[test]
    fn custom_comparator_ascending() {
        let mut queue = PriorityQueue::with_ordering(vec![3, 2, 6, 5, 1, 4], |a, b| a < b);
        assert_eq!(queue.take_front(), Some(1));
        assert_eq!(queue.take_front(), Some(2));
        assert_eq!(queue.take_front(), Some(3));
        assert_eq!(queue.take_front(), Some(4));
        assert_eq!(queue.take_front(), Some(5));
        assert_eq!(queue.take_front(), Some(6));
        assert_eq!(queue.take_front(), None);
    }

    #[test]
    fn custom_comparator_descending() {
        let mut queue = PriorityQueue::with_ordering(vec![3, 2, 6, 5, 1, 4], |a, b| a > b);
        assert_eq!(queue.take_front(), Some(6));
        assert_eq!(queue.take_front(), Some(5));
        assert_eq!(queue.take_front(), Some(4));
        assert_eq!(queue.take_front(), Some(3));
        assert_eq!(queue.take_front(), Some(2));
        assert_eq!(queue.take_front(), Some(1));
        assert_eq!(queue.take_front(), None);
    }

    #[test]
    fn non_partial_ord() {
        #[derive(Debug)]
        enum NonPartialOrd {
            One,
            Two,
            Three,
        }
        use NonPartialOrd::*;

        // Order by: a < b
        let mut queue = PriorityQueue::with_ordering(vec![Two, One, Three], |a, b| match (a, b) {
            (One, Two) | (One, Three) | (Two, Three) => true,
            (_, _) => false,
        });

        match queue.take_front() {
            Some(One) => { /* good! */ }
            x @ _ => panic!("{x:?} != Some(One)"),
        }
        match queue.take_front() {
            Some(Two) => { /* good! */ }
            x @ _ => panic!("{x:?} != Some(Two)"),
        }
        match queue.take_front() {
            Some(Three) => { /* good! */ }
            x @ _ => panic!("{x:?} != Some(Three)"),
        }
        match queue.take_front() {
            None => { /* good! */ }
            x @ _ => panic!("{x:?} != None"),
        }
    }
}
