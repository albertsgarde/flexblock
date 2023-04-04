/// Represents a array of fixed length where new elements replace the last elements.
#[derive(Clone)]
pub struct RotatingArray<T: Clone + Copy> {
    array: Vec<T>,
    cur_start: usize,
}

impl<T: Clone + Copy> RotatingArray<T> {
    /// Constructs a new RotatingArray with specified length and initial values.
    ///
    /// # Arguments
    ///
    /// * `size` - The constant size of the RotatingArray.
    /// * `initial_value` - The initial value of all elements.
    pub fn new(size: usize, initial_value: T) -> Self {
        RotatingArray {
            array: vec![initial_value; size],
            cur_start: 0,
        }
    }

    /// The constant length of the array.
    pub fn len(&self) -> usize {
        self.array.len()
    }

    /// Returns true if the length of the array is 0.
    pub fn is_empty(&self) -> bool {
        self.array.len() == 0
    }

    /// Returns true if the

    /// Gets the element with the specified index or None if no such element exists.
    ///
    /// # Arguments
    ///
    /// * `index` - The position to get the item at.
    /// If this is out of bounds, returns None.
    pub fn get(&self, index: usize) -> Option<T> {
        self.array
            .get((self.cur_start + index) % self.array.len())
            .copied()
    }

    pub fn set<F>(&mut self, index: usize, f: F) 
    where
    F: FnOnce(T) -> T,{
        let index = (self.cur_start + index) % self.array.len();
        *self.array.get_mut(index).unwrap() = f(self.array[index]);
    }

    /// Adds a new element to the begin of the array and removes one from the end.
    /// This increases the index of all elements by one.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add at the front.
    pub fn push(&mut self, value: T) {
        if self.cur_start == 0 {
            self.cur_start = self.array.len() - 1;
        } else {
            self.cur_start -= 1;
        }
        self.array[self.cur_start] = value;
    }

    /// Adds a new element to the begin of the array and removes and returns the one at the end.
    /// This increases the index of all elements by one.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to add at the front.
    pub fn push_pop(&mut self, value: T) -> T {
        if self.cur_start == 0 {
            self.cur_start = self.array.len() - 1;
        } else {
            self.cur_start -= 1;
        }
        let result = self.array[self.cur_start];
        self.array[self.cur_start] = value;
        result
    }

    /// Returns a non-consuming iterator running from the front to the back of the array.
    pub fn iter(&self) -> RotatingArrayIterator<'_, T> {
        RotatingArrayIterator::new(self)
    }
}

impl<T: Clone + Copy> IntoIterator for RotatingArray<T> {
    type Item = T;
    type IntoIter = IntoRotatingArrayIterator<T>;

    fn into_iter(self) -> IntoRotatingArrayIterator<T> {
        IntoRotatingArrayIterator::new(self)
    }
}

pub struct IntoRotatingArrayIterator<T: Clone + Copy> {
    array: RotatingArray<T>,
    cur_pos: usize,
    finished: bool,
}

impl<T: Clone + Copy> IntoRotatingArrayIterator<T> {
    fn new(array: RotatingArray<T>) -> IntoRotatingArrayIterator<T> {
        let start_pos = array.cur_start;
        IntoRotatingArrayIterator {
            array,
            cur_pos: start_pos,
            finished: false,
        }
    }
}

impl<T: Clone + Copy> Iterator for IntoRotatingArrayIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        if self.finished {
            return None;
        }
        let result = self.array.array[self.cur_pos];
        self.cur_pos += 1;
        if self.cur_pos == self.array.len() {
            self.cur_pos = 0;
        }
        if self.cur_pos == self.array.cur_start {
            self.finished = true;
        }
        Some(result)
    }
}

pub struct RotatingArrayIterator<'a, T: Clone + Copy> {
    array: &'a RotatingArray<T>,
    cur_pos: usize,
    finished: bool,
}

impl<'a, T: Clone + Copy> RotatingArrayIterator<'a, T> {
    fn new(array: &'a RotatingArray<T>) -> RotatingArrayIterator<T> {
        let start_pos = array.cur_start;
        RotatingArrayIterator {
            array,
            cur_pos: start_pos,
            finished: false,
        }
    }
}

impl<'a, T: 'a + Clone + Copy> Iterator for RotatingArrayIterator<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.finished {
            return None;
        }
        let result = &self.array.array[self.cur_pos];
        self.cur_pos += 1;
        if self.cur_pos == self.array.len() {
            self.cur_pos = 0;
        }
        if self.cur_pos == self.array.cur_start {
            self.finished = true;
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut array = RotatingArray::new(5, 0);
        for i in 1..5 {
            array.push(i)
        }
        for i in 0..5 {
            assert_eq!(array.get(i).unwrap(), 5 - i - 1);
        }
    }

    #[test]
    fn iter() {
        let mut array = RotatingArray::new(5, 0);
        for i in 1..5 {
            array.push(i);
        }
        let mut cur_num = 4;
        for &i in array.iter() {
            assert_eq!(cur_num, i);
            cur_num -= 1;
        }
    }

    #[test]
    fn into_iter() {
        let mut array = RotatingArray::new(5, 0);
        for i in 1..5 {
            array.push(i);
        }
        let mut cur_num = 4;
        for i in array.into_iter() {
            assert_eq!(cur_num, i);
            cur_num -= 1;
        }
    }
}
