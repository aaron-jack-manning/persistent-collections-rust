use core::fmt;
use std::rc::Rc;
use std::convert;

pub struct List<T> {
    elements : Rc<Node<T>>,
    length : usize,
}

#[derive(Debug)]
enum Node<T> {
    Nil,
    Cons(Rc<T>, Rc<Node<T>>),
}

impl<T> List<T> {
    /// Creates an empty list.
    pub fn new() -> List<T> {
        List {
            elements : Rc::new(Node::Nil),
            length : 0,
        }
    }

    /// Creates a list of a single element.
    pub fn singleton(value : T) -> List<T> {
        List {
            elements :
                Rc::new(
                    Node::Cons(
                        Rc::new(value),
                        Rc::new(Node::Nil)
                    )
                ),
            length : 1,
        }
    }

    /// Prepends to the provided list, returning the new list.
    pub fn prepend(&self, value : T) -> List<T> {
        List {
            elements :
                Rc::new(
                    Node::Cons(
                        Rc::new(value),
                        Rc::clone(&self.elements)
                )
            ),
            length : self.length + 1 
        }
    }

    /// Returns the head of the list.
    pub fn head(&self) -> Option<Rc<T>> {
        match &*self.elements {
            Node::Nil => None,
            Node::Cons(head, _) => Some(Rc::clone(head)),
        }
    }

    /// Returns the tail of the list.
    pub fn tail(&self) -> List<T> {
        match &*self.elements {
            Node::Nil => List::new(),
            Node::Cons(_, tail) => List { elements : Rc::clone(tail), length : self.length - 1 },
        }
    }

    /// Returns the length of the list.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Determines if the list is empty.
    pub fn is_empty(&self) -> bool {
        match self.length {
            0 => true,
            _ => false,
        }
    }

    fn reverse_helper(node : &Node<T>, acc : Node<T>) -> Node<T> {
        match node {
            Node::Nil => acc,
            Node::Cons(head, tail) => List::reverse_helper(tail, Node::Cons(Rc::clone(head), Rc::new(acc))),
        }
    }

    // Reverses the list.
    pub fn reverse(&self) -> List<T> {
        List {
            elements : Rc::new(List::reverse_helper(&*self.elements, Node::Nil)),
            length : self.length,
        }
    }

    /// Deconstructs the list into the head and the tail.
    pub fn deconstruct(&self) -> (Option<Rc<T>>, List<T>) {
        (self.head(), self.tail()) 
    }

    /// Returns an iterator over the list.
    pub fn iter(&self) -> ListIter<T> {
        ListIter(self.clone())
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> List<T> {
        List {
            elements : Rc::clone(&self.elements),
            length : self.length
        }
    }
}

impl<T> convert::From<Vec<T>> for List<T> {
    // Creates a List<T> from a Vec<T>.
    fn from(mut vec : Vec<T>) -> Self {
        let mut list = List::new();
        for _i in 0..vec.len() {
            list = list.prepend(vec.pop().unwrap());
        }
        list
    }
}

impl<T> PartialEq for List<T>
    where T : PartialEq {
    fn eq(&self, other : &List<T>) -> bool {
        let (head_1, tail_1) = self.deconstruct();
        let (head_2, tail_2) = other.deconstruct();
       
        match (head_1, head_2) {
            (Some(val_1), Some(val_2)) => {
                if val_1 == val_2 {
                    <List<T> as PartialEq>::eq(&tail_1, &tail_2)
                }
                else {
                    false
                }
            },
            (None, None) => true,
            _ => false,
        }
    }
}

pub struct ListIter<T>(List<T>);

impl<T> Iterator for ListIter<T> {
    type Item = Rc<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let (head, tail) = self.0.deconstruct();
        *self = ListIter(tail);
        head
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<S>(iter : S) -> Self
        where S : IntoIterator<Item = T> {

        let mut list = List::new();
        for val in iter {
            list = list.prepend(val);
        }
        list.reverse()
    }
}

impl<T> fmt::Debug for List<T>
    where T : fmt::Debug {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut formatted = String::from("[");
        
        let mut iterator = self.iter();

        for i in 0..self.len() {
            let element = iterator.next().unwrap();

            if i == self.len() - 1 {
                formatted.push_str(&format!("{:?}", element));
            }
            else {
                formatted.push_str(&format!("{:?}, ", element));
            }

        }

        formatted.push_str("]");

        write!(f, "{}", formatted)
    }
}
