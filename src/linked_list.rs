/// A node with simple payload.
/// The pointer is the XOR of the prev and next ptr.
#[derive(Debug, Clone)]
pub struct Node<T> {
    ptr: usize,
    payload: T,
}

impl<T> Node<T> {
    pub fn new(payload: T) -> Self {
        Node { ptr: 0, payload }
    }
}

/// Simple memory management in a vector.
/// 0 is a null pointer
/// All other pointers are indexes into the vector + 1
#[derive(Debug, Clone)]
pub struct Memory<T> {
    slots: Vec<Option<Node<T>>>,
    free_slots: Vec<usize>,
}

impl<T> Memory<T> {
    pub fn new() -> Self {
        Memory {
            slots: Vec::new(),
            free_slots: Vec::new(),
        }
    }

    /// Allocates a new node and returns its pointer
    pub fn alloc(&mut self, payload: T) -> usize {
        let node = Node::new(payload);
        if let Some(slot_index) = self.free_slots.pop() {
            self.slots[slot_index] = Some(node);
            slot_index + 1
        } else {
            self.slots.push(Some(node));
            self.slots.len()
        }
    }

    /// Returns the node's payload and frees the node
    pub fn remove(&mut self, ptr: usize) -> Option<T> {
        self.free_slots.push(ptr - 1);
        self.slots
            .get_mut(ptr - 1)
            .and_then(Option::take)
            .map(|node| node.payload)
    }

    pub fn get_mut(&mut self, ptr: usize) -> Option<&mut Node<T>> {
        if ptr == 0 {
            return None;
        }

        self.slots.get_mut(ptr - 1).and_then(Option::as_mut)
    }

    pub fn len(&self) -> usize {
        self.slots.len() - self.free_slots.len()
    }
}

#[derive(Debug, Clone)]
pub struct LinkedList<T> {
    head: usize,
    tail: usize,
    memory: Memory<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: 0,
            tail: 0,
            memory: Memory::new(),
        }
    }

    pub fn push_front(&mut self, payload: T) {
        let node_ptr = self.memory.alloc(payload);

        // head points to new node
        let old_head_ptr = self.head;
        self.head = node_ptr;

        // new node points to old head
        let node = self.memory.get_mut(node_ptr).unwrap();
        node.ptr ^= old_head_ptr;

        // old head points back to new node
        if let Some(old_head) = self.memory.get_mut(old_head_ptr) {
            old_head.ptr ^= node_ptr; // no need to reset, was XORed with 0
        }

        if self.len() == 1 {
            // also need to set the tail ptr
            self.tail = node_ptr;
        }
    }

    pub fn push_back(&mut self, payload: T) {
        let node_ptr = self.memory.alloc(payload);

        // tail points to new node
        let old_tail_ptr = self.tail;
        self.tail = node_ptr;

        // new node points to old tail
        let node = self.memory.get_mut(node_ptr).unwrap();
        node.ptr ^= old_tail_ptr;

        // old tail points forward to new node
        if let Some(old_tail) = self.memory.get_mut(old_tail_ptr) {
            old_tail.ptr ^= node_ptr; // no need to reset, was XORed with 0
        }

        if self.len() == 1 {
            // also need to set the head ptr
            self.head = node_ptr;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let head_node_ptr = self.head;
        let head_node = self.memory.get_mut(head_node_ptr)?;

        let next_node_ptr = head_node.ptr;
        if let Some(next_node) = self.memory.get_mut(next_node_ptr) {
            // remove pointer from popped node from next_node
            next_node.ptr ^= head_node_ptr;
        }

        // set new head
        self.head = next_node_ptr;

        if self.len() == 1 {
            // also need to reset the tail ptr
            self.tail = 0;
        }

        self.memory.remove(head_node_ptr)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let tail_node_ptr = self.tail;
        let tail_node = self.memory.get_mut(tail_node_ptr)?;

        let prev_node_ptr = tail_node.ptr;
        if let Some(prev_node) = self.memory.get_mut(prev_node_ptr) {
            // remove pointer from popped node from next_node
            prev_node.ptr ^= tail_node_ptr;
        }

        // set new tail
        self.tail = prev_node_ptr;

        if self.len() == 1 {
            // also need to reset the head ptr
            self.head = 0;
        }

        self.memory.remove(tail_node_ptr)
    }

    pub fn len(&self) -> usize {
        self.memory.len()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter { linked_list: self }
    }
}

pub struct LinkedListIter<T> {
    linked_list: LinkedList<T>,
}

impl<T> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.linked_list.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_pop_back() {
        let mut list = LinkedList::new();

        for i in 0..10 {
            list.push_back(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_back(), Some(9 - i));
        }
    }

    #[test]
    fn insert_and_pop_front() {
        let mut list = LinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_front(), Some(9 - i));
        }
    }

    #[test]
    fn insert_front_and_pop_back() {
        let mut list = LinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_back(), Some(i));
        }
    }

    #[test]
    fn insert_and_reinsert() {
        let mut list = LinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_back(), Some(i));
        }

        for i in 0..10 {
            list.push_back(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_front(), Some(i));
        }
    }

    #[test]
    fn insert_and_reinsert_alt() {
        let mut list = LinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_front(), Some(9 - i));
        }

        for i in 0..10 {
            list.push_back(i);
        }

        for i in 0..10 {
            assert_eq!(list.pop_back(), Some(9 - i));
        }
    }

    #[test]
    fn gap() {
        let mut list = LinkedList::new();

        for i in 0..3 {
            list.push_front(i);
        }

        list.pop_back();
        list.pop_front();

        assert_eq!(list.clone().into_iter().collect::<Vec<_>>(), vec![1]);

        for i in 3..6 {
            list.push_back(i);
        }

        list.pop_back();
        list.pop_front();

        assert_eq!(list.clone().into_iter().collect::<Vec<_>>(), vec![3, 4]);

        for i in 6..9 {
            list.push_front(i);
        }

        list.pop_front();
        list.pop_back();

        assert_eq!(list.into_iter().collect::<Vec<_>>(), vec![7, 6, 3]);
    }
}
