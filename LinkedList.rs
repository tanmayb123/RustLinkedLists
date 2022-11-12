use std::fmt::Display;

struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T> LinkedList<T> {
    fn new(data: T) -> LinkedList<T> {
        LinkedList {
            data: data,
            next: None,
        }
    }

    fn append(&mut self, data: T) {
        if let Some(ref mut next) = self.next {
            next.append(data);
        } else {
            self.next = Some(Box::new(LinkedList::new(data)));
        }
    }
}

impl<T> LinkedList<T> where T: Display {
    fn print(&self) {
        let mut current = self;
        while let Some(ref next) = current.next {
            println!("{}", current.data);
            current = next;
        }
        println!("{}", current.data);
    }
}

fn yield_pairs<'a, T, F>(
    list: &'a LinkedList<T>,
    yield_fn: F
) where F: Fn(&'a T, &'a T) {
    let mut a = list;
    let mut b = &list.next;
    while let Some(b_val) = b {
        yield_fn(&a.data, &b_val.data);
        a = a.next.as_ref().unwrap();
        b = &b_val.next;
    }
}

fn append_sorted<T: Ord>(
    list: Option<LinkedList<T>>,
    data: T,
) -> LinkedList<T> {
    if let Some(mut old_head) = list {
        let mut new_node = LinkedList::new(data);

        if old_head.data >= new_node.data {
            new_node.next.replace(Box::new(old_head));
            return new_node;
        } else {
            let mut current = &mut old_head;
            while let Some(ref next) = current.next {
                if next.data >= new_node.data {
                    break;
                }
                current = current.next.as_mut().unwrap();
            }

            new_node.next = current.next.take();
            current.next.replace(Box::new(new_node));

            return old_head;
        }
    } else {
        LinkedList::new(data)
    }
}


fn main() {
    let mut list = LinkedList::new(1);
    list = append_sorted(Some(list), 5);
    list = append_sorted(Some(list), 2);
    list = append_sorted(Some(list), 4);
    list = append_sorted(Some(list), 3);
    list.print();
    yield_pairs(&list, |a, b| println!("{}, {}", a, b));
}
