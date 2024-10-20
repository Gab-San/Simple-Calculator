//! This crate provides an implementation of the stack. A stack is one of the basic data structures.
//! 
//! It can be thought sort of as a stack of boxes: each time a new element 
//! is added to the stack (push) it is added on top of it, as a new box must be put on top of the others.
//! To retrieve elements from a stack they must be taken (pop) from the top to the bottom of it. 

use core::slice;
use std::{alloc::{self, Layout}, fmt::Debug, mem, ptr::{self, NonNull}};


const REALLOC_AMOUNT : usize = 32;

#[derive(Debug)]
struct RawVec<T> {
    ptr: NonNull<T>,
    capacity : usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        let capacity = if mem::size_of::<T>() == 0 {usize::MAX} else {0};

        RawVec {
            ptr : NonNull::dangling(),
            capacity,
        }
    }

    pub fn with_capacity(init_cap : usize) -> Self {

        if mem::size_of::<T>() == 0 {
            return Self {
                ptr : NonNull::dangling(),
                capacity : usize::MAX,
            };
        }

        let capacity = init_cap;

         // Ensuring allocation validity
         assert!(init_cap <= isize::MAX as usize, "Initial allocation too large!");
        
        let layout = Layout::array::<T>(init_cap).unwrap();
        
       

        let layout_ptr = unsafe{alloc::alloc(layout)};

        let ptr = match NonNull::new(layout_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(layout),
        };

        Self {
            ptr,
            capacity,
        }
    }

    fn grow(&mut self) {

        // If trying to grow whilst type is a ZST means the vec is overfull.
        assert!(mem::size_of::<T>() != 0, "Capacity overflow");

        let (new_cap, new_layout) = {
            let new_cap = self.capacity + REALLOC_AMOUNT;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Check for valid capacity
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

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
    pub fn with_capacity(init_cap : usize) -> Self {
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
    pub fn push(&mut self, elem : T) -> &mut Self{
        if self.length == self.cap() {
            self.buf.grow();
        }

        unsafe { ptr::write(self.as_ptr().add(self.length), elem); }
        self.length +=1;

        self
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
    /// the top of the stack.
    /// 
    /// This function is not always considered as a function of a stack. But it is useful in some scenarios
    /// and avoids boiler plate popping and pushing code. 
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
    /// This function is not needed to check the emptyiness of a stack (a stack is empty if the pop returns None)
    /// though it was implemented just for completion sake.
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

impl<T : Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice : &[T] = unsafe{std::slice::from_raw_parts(self.as_ptr(), self.length)};
        f.debug_struct("RawVec")
        .field("vals", &slice.iter())
        .field("cap", &self.cap())
        .field("len", &self.length)
        .finish()
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
    fn create_empty_stack_with_capacity() {
        let stack: Stack<u32> = Stack::with_capacity(5);
        println!("{:#?}", stack);
    }

    #[test]
    fn create_stack_with_capacity() {
        let mut stack : Stack<u8> = Stack::with_capacity(5);
        for i in 0..5 {
            stack.push(i);
        }
        assert_eq!(*stack.peek().unwrap(), 4);
        println!("{:#?}", stack);
    }

    #[test]
    fn push_multi () {
        let mut s  = Stack::with_capacity(3);
        s.push(1).push(2).push(3);
        println!("{s:#?}");
    }

    #[test]
    fn push_values_past_capacity() {
        let st_cap = 5;
        let mut stack : Stack<u8> = Stack::with_capacity(st_cap);
        println!("Starting cap: {}", st_cap);
        for i in 0..5 {
            stack.push(i);
        }
        assert_eq!(*stack.peek().unwrap(), 4);
        stack.push(5);
        assert_eq!(*stack.peek().unwrap(), 5);
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