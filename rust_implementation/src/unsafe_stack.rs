use std::alloc::{alloc, dealloc, handle_alloc_error, realloc, Layout};


const REALLOC_AMOUNT : usize = 32;

#[derive(Debug)]
pub struct UnsafeStack<T> {
    layout : Layout,
    layout_ptr : *mut u8,
    s_top : *mut T,
    size : usize,
    capacity : usize,
}

impl<T> UnsafeStack<T> {

    pub fn new() -> Self {
        unsafe {
            let layout = Layout::array::<T>(0).unwrap();
            let layout_ptr = alloc(layout);
            if layout_ptr.is_null() {
                handle_alloc_error(layout);
            }

            let s_top = layout_ptr as *mut T;

            Self {
                layout,
                layout_ptr,
                s_top,
                size : 0,
                capacity : 0,
            }
        }
    }

    pub fn with_capacity(init_cap : usize) -> Self {
        unsafe{
            let layout = Layout::array::<T>(init_cap).unwrap();
            let layout_ptr = alloc(layout);
            if layout_ptr.is_null() {
                handle_alloc_error(layout);
            }

            let s_top = layout_ptr as *mut T;

            Self {
                layout,
                layout_ptr,
                s_top,
                size : 0,
                capacity : init_cap,
            }
        }
    }

    pub fn push(&mut self, value : T) {
        unsafe {
            if !(self.size < self.capacity) {
                self.capacity += REALLOC_AMOUNT;
                self.layout_ptr = realloc(self.layout_ptr, self.layout, self.capacity);
                if self.layout_ptr.is_null(){
                    handle_alloc_error(self.layout);
                }
            }

            if self.size != 0 {
                self.s_top = self.s_top.add(1);
            }

            *self.s_top = value;
            self.size += 1;
        }
    }

    pub fn pop(&mut self) -> Option<&T> {
        unsafe {
            if self.size == 0 {
                return None;
            }
            
            let pop_ptr = self.s_top;
            self.s_top = self.s_top.sub(1);

            self.size -= 1;

            Some(&*pop_ptr)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }

        unsafe {
            Some(&*self.s_top)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}


// TODO: implement tests when able to allocate memory and define a proper stack

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_stack() {
        let stack: UnsafeStack<u8> = UnsafeStack::new();
        println!("{:#?}", stack);
        unsafe {dealloc(stack.layout_ptr, stack.layout);}
    }

    #[test]
    fn create_stack_with_capacity() {
        let mut stack : UnsafeStack<u8> = UnsafeStack::with_capacity(5);
        for i in 0..5 {
            stack.push(i);
        }
        assert_eq!(*stack.peek().unwrap(), 4);
        println!("{:#?}", stack);
        unsafe{dealloc(stack.layout_ptr, stack.layout);}
    }

    #[test]
    fn push_through_capacity() {
        let mut stack : UnsafeStack<u8> = UnsafeStack::with_capacity(5);
        for i in 0..=7 {
            stack.push(i);
        }
        assert_eq!(*stack.peek().unwrap(), 7);
        println!("{:#?}", stack);
        unsafe{dealloc(stack.layout_ptr, stack.layout);}
    }

    #[test]
    fn peek_stack() {
        let mut stack : UnsafeStack<String> = UnsafeStack::with_capacity(10);
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
        let mut stack = UnsafeStack::new();
        assert!(stack.is_empty());
        stack.push(2);
        assert!(!stack.is_empty());
        stack.pop();
        assert!(stack.is_empty());
    }
}