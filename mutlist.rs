// https://rust-unofficial.github.io/too-many-lists/second.html

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct List<T>(Option<Box<Node<T>>>);

struct IntoIter<T> {
    cur: Option<Box<Node<T>>>,
}

struct Iter<'a, T> {
    cur: Option<&'a Node<T>>,
}

struct IterMut<'a, T> {
    cur: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        let head = self.cur.take()?;
        let data = head.data;
        *self = IntoIter::new(head.next);
        Some(data)
    }
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

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.cur.take().map(|node| {
            self.cur = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

impl<T> IntoIter<T> {
    fn new(cur: Option<Box<Node<T>>>) -> Self {
        IntoIter { cur }
    }
}

impl<'a, T> Iter<'a, T> {
    fn new(cur: Option<&'a Node<T>>) -> Self {
        Iter { cur }
    }
}

impl<'a, T> IterMut<'a, T> {
    fn new(cur: Option<&'a mut Node<T>>) -> Self {
        IterMut { cur }
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

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        IntoIter::new(self.0)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter::new(self.0.as_deref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        IterMut::new(self.0.as_deref_mut())
    }
}
