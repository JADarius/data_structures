pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

#[allow(dead_code)]
impl<T> LinkedList<T> {
    fn new() -> LinkedList<T> {
        LinkedList { head: None }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn push(&mut self, data: T) {
        let new_head = Box::new(Node{ data, next: self.head.take()});
        self.head = Some(new_head);
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|n| {self.head = n.next; n.data})
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|n| {&n.data})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list: LinkedList<u32> = LinkedList::new();
        assert!(list.is_empty());
    }
    
    #[test]
    fn test_list() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        assert_eq!(list.pop().unwrap(), 4);
        assert_eq!(list.pop().unwrap(), 3);
        assert_eq!(list.pop().unwrap(), 2);
        assert_eq!(list.pop().unwrap(), 1);
    }
}
