use std::rc::Rc;

use crate::linked_list::List;

#[derive(PartialEq, Debug)]
pub struct Queue<T>(Rc<QueuePrivate<T>>);

#[derive(PartialEq, Debug)]
pub struct QueuePrivate<T> {
    front : Rc<List<T>>,    
    back : Rc<List<T>>,
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Queue<T> {
        Queue(
            Rc::new(QueuePrivate {
                back : Rc::new(List::new()),
                front : Rc::new(List::new()),
            })
        )
    }

    fn from_lists(front : Rc<List<T>>, back : Rc<List<T>>) -> Queue<T> {
        Queue(
            Rc::new(QueuePrivate {
                back,
                front,
            })
        )
    }

    /// Remove an element from the front of the queue, returning the removed element and the new queue.
    pub fn dequeue(&self) -> (Option<Rc<T>>, Queue<T>) {
        let (head, tail) = self.0.front.deconstruct();
        
        if head.is_some() {
            (head, Queue::from_lists(Rc::clone(&self.0.back), Rc::new(tail)))
        }
        else {
            let temp = List::reverse(&self.0.back);
            let (removed, new_front) = temp.deconstruct();
            
            (removed, Queue::from_lists(Rc::new(new_front), Rc::new(List::new())))
        }
    }

    /// Enqueue an element to the back of the queue, returning the new queue.
    pub fn enqueue(&self, value : T) -> Queue<T> {
        let new_back = self.0.back.prepend(value);
        Queue::from_lists(Rc::clone(&self.0.front), Rc::new(new_back))
    }
}

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Queue<T> {
        Queue::from_lists(Rc::clone(&self.0.front), Rc::clone(&self.0.front))
    }
}
