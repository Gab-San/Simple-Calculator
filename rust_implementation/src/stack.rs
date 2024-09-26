/// Data structure of type LIFO (_Last In First Out_) growable in size.
/// 
/// # Examples
/// ```
/// use calculator::stack::Stack;
/// 
/// let mut stack = Stack::new();
/// stack.push(3);
/// stack.push(5);
/// stack.push(4);
/// stack.push(2);
/// 
/// assert_eq!(stack.pop(), Some(2));
/// assert_eq!(stack.pop(), Some(4));
/// assert_eq!(stack.pop(), Some(5));
/// assert_eq!(stack.pop(), Some(3));
/// ```

pub struct Stack<T> {
    vector : Vec<T>,
}

impl<T> Stack<T> {
    /// Creates a new empty stack.
    /// 
    /// # Example
    /// ```
    /// use calculator::stack::Stack;
    /// 
    /// let stack : Stack<i32> = Stack::new();
    /// ```
    pub fn new() -> Self {
        let vector : Vec<T> = Vec::new();
        Self{
            vector,
        }
    }

    /// Creates an empty stack of at least `usize` capacity.
    /// 
    /// Memory for usize elements is already allocated,
    /// saving some computational time.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use calculator::stack::Stack;
    /// 
    /// // This stack will contain at least 4 string before reallocation
    /// let stack : Stack<i32> = Stack::with_capacity(4);
    /// 
    /// // This are all done without reallocation
    /// for i in 0..4 {
    ///     stack.push(i);
    /// }
    /// // This may require reallocation
    /// stack.push(5);
    /// ```
    pub fn with_capacity(init_cap : usize) -> Self {
        let vector : Vec<T> = Vec::with_capacity(init_cap);
        Self {
            vector,
        }
    }

    /// Inserts an element at the top of the stack.
    /// 
    /// # Panics
    /// 
    /// Panics if the new capacity exceeds `isize::MAX_BYTES` bytes.
    /// 
    /// # Examples
    /// ```
    /// use calculator::stack::Stack;
    /// 
    /// let mut stack = Stack::new();
    /// stack.push(3);
    /// assert_eq!(stack.peek(), Some(3).as_ref());
    /// ```
    /// 
    /// # Time complexity
    /// 
    /// Takes O(1) time.
    pub fn push(&mut self, value : T) {
        self.vector.push(value);
    }

    /// Removes the last element inserted on the stack
    /// and returns it 
    /// 
    /// # Examples 
    /// ```
    /// use calculator::stack::Stack;
    /// 
    /// let mut stack = Stack::new()
    /// stack.push(3);
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), None);
    /// ```
    /// 
    /// # Time complexity
    /// 
    /// Takes O(1) time.
    pub fn pop(&mut self) -> Option<T> {
        self.vector.pop()
    }

    /// Returns the reference to the element at 
    /// the top of the stack
    /// 
    /// # Examples
    /// ```
    /// use calculator::stack::Stack;
    /// 
    /// let mut stack = Stack::new();
    /// stack.push("Hello");
    /// stack.push("Everybody");
    /// 
    /// assert_eq!(stack.peek(), Some("Everybody").as_ref());
    /// stack.pop();
    /// assert_eq!(stack.peek(), Some("Hello").as_ref());
    /// ``` 
    /// 
    /// # Time complexity
    /// Takes O(1) time.
    pub fn peek(&self) -> Option<&T> {
        self.vector.last()
    }
}


// TODO: implement tests when able to allocate memory and define a proper stack

// #[cfg(test)]
// mod tests {
//     use super::*;
// }