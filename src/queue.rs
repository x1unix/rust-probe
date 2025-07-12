use std::{cell::{Ref, RefCell}, rc::Rc};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }

    fn new_cell(value: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(value)))
    }
}

pub struct Queue<T> {
    length: usize,
    wc: usize,
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            length: 0,
            wc: 0,
            head: None,
            tail: None,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        let node = Node::new_cell(item);

        self.length += 1;
        self.wc += 1;
        if let (_, Some(tail)) = (self.head.as_ref(), self.tail.as_ref()) {
            tail.borrow_mut().next = Some(node.clone());
            self.tail = Some(node);
            return;
        }

        self.head = Some(node.clone());
        self.tail = Some(node.clone());
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        // Move head ownership to var, replace old ref with None.
        let head_rc = self.head.take().unwrap();
        self.length -= 1;

        // When single item left - head and tail point to a same value.
        // Rc::try_unwrap fails when nRefs is more than 1.
        // Remove tail reference to allow Rc::try_unwrap.
        if self.length == 0 {
            self.tail = None;
        }

        // get Node out of RC and then out of RefCell
        let head_node = Rc::try_unwrap(head_rc).ok().unwrap().into_inner();
        self.head = head_node.next;

        if self.tail.is_none() && self.head.is_some() {
            // restore tail if was removed.
            self.tail = self.head.clone()
        }

        // None
        Some(head_node.value)
    }

    pub fn peek(&self) -> Option<Ref<T>> {
        // Return a reference to just the value, not the entire node
        self.head.as_ref().map(|v| Ref::map(v.borrow(), |node| &node.value))
    }
}
