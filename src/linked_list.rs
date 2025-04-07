use std::fmt::Display;

struct Node <T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> where T:Copy + Display {
    pub fn new()-> Self {
        LinkedList{
            head: None,
            size: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        if self.is_empty() {
            self.head = Some(new_node);
        } else {
            let mut current = &mut self.head;
            while let Some(node) = current {
                if node.next.is_none() {
                    node.next = Some(new_node);
                    break;
                }
                current = &mut node.next;
            }
        }
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map( |node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
        
    }

    pub fn display(&self) {
        if self.is_empty() {
            println!("List is empty");
            return;
        }
        let mut current = &self.head;
        while let Some(node) = current {
            print!("{} -> ", node.value);
            current = &node.next;
        }
        println!("None");
    }
}

pub fn test_linked_list() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    list.display(); // 2 -> 1 -> 3 -> None
    println!("Length: {}", list.len()); // Length: 3
    list.pop_front();
    list.display(); // 1 -> 3 -> None
    println!("Length: {}", list.len()); // Length: 2
}