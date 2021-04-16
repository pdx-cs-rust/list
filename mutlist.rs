/// Example singly-linked list implementation.

/// [Learn Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/second.html)
/// served as a valuable reference for this code.
use std::iter::FromIterator;

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

/// A singly-linked list.
pub struct List<T>(Option<Node<T>>);

/// An iterator returning successive data items from a list.
pub struct IntoIter<T> {
    cur: Option<Node<T>>,
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
        let mut head = self.cur.take()?;
        let data = head.data;
        let cur = head.next.take().map(|node| *node);
        *self = IntoIter { cur };
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

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> List<T> {
    /// A new empty list.
    pub fn new() -> Self {
        Self::default()
    }

    /// A new list containing `data`. Order is left-to-right: `data[0]` will be the head.
    pub fn from_vec(mut data: Vec<T>) -> Self {
        let mut result = List::new();
        while let Some(d) = data.pop() {
            result.push(d);
        }
        result
    }

    /// Push `data` onto the head of this list.
    pub fn push(&mut self, data: T) {
        let next = self.0.take().map(Box::new);
        (*self).0 = Some(Node { data, next });
    }

    /// Pop from the head of this list. Return `Some` element datum, or `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        let head = self.0.take()?;
        let next = head.next.map(|node| *node);
        let data = head.data;
        (*self).0 = next;
        Some(data)
    }

    /// An iterator over references to this data.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        Iter {
            cur: self.0.as_ref(),
        }
    }

    /// An iterator over mutable references to this data.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        IterMut {
            cur: self.0.as_mut(),
        }
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { cur: self.0 }
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let data: Vec<T> = iter.into_iter().collect();
        List::from_vec(data)
    }
}

#[test]
fn test_push_pop() {
    let mut list = List::from_vec(vec![1, 2, 3]);
    let mut result = Vec::new();
    while let Some(d) = list.pop() {
        result.push(d);
    }
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_iter() {
    let list = List::from_vec(vec![1, 2, 3]);
    let result: Vec<u8> = list.iter().cloned().collect();
    assert_eq!(result, &[1, 2, 3]);
}

#[test]
fn test_iter_mut() {
    let mut list = List::from_vec(vec![1, 2, 3]);
    for d in list.iter_mut() {
        *d = 4 - *d;
    }
    let result: Vec<u8> = list.iter().cloned().collect();
    assert_eq!(result, &[3, 2, 1]);
}
