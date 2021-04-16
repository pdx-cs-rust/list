struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T>(Option<Box<Node<T>>>);

// https://rust-unofficial.github.io/too-many-lists/second-iter.html
struct Iter<'a, T> {
    cur: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.map(|node| {
            self.cur = node.next.as_deref();
            &node.data
        })
    }
}

impl<'a, T> Iter<'a, T> {
    fn new(cur: Option<&'a Node<T>>) -> Self {
        Iter { cur }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List(None)
    }

    pub fn push(&mut self, data: T) {
        let next = self.0.take();
        *self = List(Some(Box::new(Node { data, next })));
    }

    pub fn pop(&mut self) -> Option<T> {
        let head = self.0.take()?;
        let data = head.data;
        *self = List(head.next);
        Some(data)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter::new(self.0.as_deref())
    }
}
