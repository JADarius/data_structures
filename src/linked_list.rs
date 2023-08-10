#[derive(Debug, PartialEq)]
struct LinkedList {
    data: u32,
    next: Option<Box<LinkedList>>,
}

#[allow(dead_code)]
impl LinkedList {
    fn new(data: u32) -> LinkedList {
        LinkedList { data, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let list: LinkedList = LinkedList::new(23);
        assert_eq!(list.data, 23);
        assert_eq!(list.next, None);
    }
}
