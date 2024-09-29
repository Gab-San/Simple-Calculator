use core::slice;
use std::{alloc::{self, Layout}, mem, ptr::{self, NonNull}};


const REALLOC_AMOUNT : usize = 32;

#[derive (Debug)]
struct RawVec<T> {
    ptr: NonNull<T>,
    capacity : usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        let capacity = if mem::size_of::<T>() == 0 {isize::MAX as usize} else {0};

        RawVec {
            ptr : NonNull::dangling(),
            capacity,
        }
    }

    fn with_capacity(init_cap : usize) -> Self {

        assert!(init_cap < isize::MAX as usize, "Allocation too large");
        let capacity = if mem::size_of::<T>() == 0 {isize::MAX as usize} else {init_cap};

        // Sure to be able to allocate (introducing a bit of redundancy)
        let layout = Layout::array::<T>(init_cap).unwrap();
        
        unsafe{
            let layout_ptr = alloc::alloc(layout);
            if layout_ptr.is_null() {
                alloc::handle_alloc_error(layout);
            }
        }

        Self {
            ptr : NonNull::dangling(),
            capacity : capacity,
        }
    }

    fn grow(&mut self) {

        // If trying to grow whilst type is a ZST means the vec is overfull.
        assert!(mem::size_of::<T>() != 0, "Capacity overflow");

        let (new_cap, new_layout) = {
            let new_cap = self.capacity + REALLOC_AMOUNT;
            // Check for valid capacity
            assert!(new_cap <= isize::MAX as usize, "Allocation too large");
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        let new_ptr = if self.capacity == 0 {
            // Never allocated anything before
            unsafe {alloc::alloc(new_layout)}
        } else {
            let old_layout = Layout::array::<T>(self.capacity).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe {alloc::realloc(old_ptr, old_layout, new_layout.size())}
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.capacity = new_cap;
    } 
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        // If there was an allocation
        if self.capacity != 0 {  
            let layout = Layout::array::<T>(self.capacity).unwrap();
            unsafe {alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);}
        }
    }
}


/// Data structure of type LIFO (_Last In First Out_) growable in size.
/// 
/// # Examples
/// ```
/// use calculator::Stack;
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
#[derive(Debug)]
pub struct Stack<T> {
    // Cannot ref and deref directly into memory due to it possibly not being initialized,
    // be evaluated as valid instance of T and would call drop on the overwritten value.
    buf : RawVec<T>,
    length : usize,
}

impl<T> Stack<T> {
    fn as_ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.capacity
    }

    /// Creates a new empty stack.
    /// 
    /// # Example
    /// ```
    /// use calculator::Stack;
    /// 
    /// let stack : Stack<i32> = Stack::new();
    /// ```
    pub fn new() -> Self {
        Stack {
            buf : RawVec::new(),
            length : 0,
        }
    }


    // /// Creates an empty stack of at least `usize` capacity.
    // /// 
    // /// Memory for usize elements is already allocated,
    // /// saving some computational time.
    // /// 
    // /// # Examples
    // /// 
    // /// ```
    // /// use calculator::Stack;
    // /// 
    // /// // This stack will contain at least 4 string before reallocation
    // /// let mut stack : Stack<i32> = Stack::with_capacity(4);
    // /// 
    // /// // This are all done without reallocation
    // /// for i in 0..4 {
    // ///     stack.push(i);
    // /// }
    // /// // This may require reallocation
    // /// stack.push(5);
    // /// ```
    fn with_capacity(init_cap : usize) -> Self {
        Stack {
            buf : RawVec::with_capacity(init_cap),
            length : 0,
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
    /// use calculator::Stack;
    /// 
    /// let mut stack = Stack::new();
    /// stack.push(3);
    /// assert_eq!(stack.peek(), Some(3).as_ref());
    /// ```
    /// 
    /// # Time complexity
    /// 
    /// Takes O(1) time.
    pub fn push(&mut self, elem : T) {
        if self.length == self.cap() {
            self.buf.grow();
        }

        unsafe { ptr::write(self.as_ptr().add(self.length), elem); }
        self.length +=1;
    }


    /// Removes the last element inserted on the stack
    /// and returns it 
    /// 
    /// # Examples 
    /// ```
    /// use calculator::Stack;
    /// 
    /// let mut stack = Stack::new();
    /// stack.push(3);
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), None);
    /// ```
    /// 
    /// # Time complexity
    /// 
    /// Takes O(1) time.
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        unsafe {
            Some(ptr::read(self.as_ptr().add(self.length)))
        }
    }

    /// Returns the reference to the element at 
    /// the top of the stack
    /// 
    /// # Examples
    /// ```
    /// use calculator::Stack;
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
        unsafe {slice::from_raw_parts(self.as_ptr(), self.length).last()}
    }


    /// Returns `true` if the stack contains no elements
    /// 
    /// # Examples
    /// 
    /// ```
    /// use calculator::Stack;
    /// 
    /// let mut stack = Stack::new();
    /// assert!(stack.is_empty());
    /// stack.push(2);
    /// assert!(!stack.is_empty());
    /// stack.pop();
    /// assert!(stack.is_empty());
    /// ```
    /// 
    /// # Time complexity
    /// Takes O(1) time.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

unsafe impl<T : Send> Send for Stack<T> {}
unsafe impl<T : Sync> Sync for Stack<T> {}

impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_stack() {
        let stack: Stack<u8> = Stack::new();
        println!("{:#?}", stack);
    }

    #[test]
    #[ignore = "with_capacity doesn't work"]
    fn create_stack_with_capacity() {
        let mut stack : Stack<u8> = Stack::with_capacity(5);
        for i in 0..5 {
            stack.push(i);
        }
        assert_eq!(*stack.peek().unwrap(), 4);
        println!("{:#?}", stack);
    }

    #[test]
    fn push_values() {
        let mut stack : Stack<u8> = Stack::new();
        for i in 0..=7 {
            stack.push(i);
        }
        assert_eq!(*stack.peek().unwrap(), 7);
        println!("{:#?}", stack);
    }

    #[test]
    #[should_panic]
    fn create_stack_with_too_much_capacity() {
        let _stack : Stack<u8> = Stack::with_capacity(usize::MAX);
    }

    #[test]
    fn peek_stack() {
        let mut stack : Stack<String> = Stack::new();
        stack.push(String::from("Hello"));
        stack.push(String::from("Everybody"));
        assert_eq!(*stack.peek().unwrap(), "Everybody");
        stack.pop();
        
        assert_eq!(*stack.peek().unwrap(), "Hello");
        stack.pop();
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn check_emptyness() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());
        stack.push(2);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }
}