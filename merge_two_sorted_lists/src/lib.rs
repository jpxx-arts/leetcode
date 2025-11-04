#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(0)));
    let mut tail = result.as_mut();

    let mut list1_it = list1.as_ref();
    let mut list2_it = list2.as_ref();

    while list1_it.is_some() && list2_it.is_some() {
        let val;
        if list1_it.as_ref().unwrap().val <= list2_it.as_ref().unwrap().val {
            val = list1_it.as_ref().unwrap().val;
            list1_it = list1_it.unwrap().next.as_ref();
        } else {
            val = list2_it.as_ref().unwrap().val;
            list2_it = list2_it.unwrap().next.as_ref();
        }

        let new_node = Box::new(ListNode::new(val));
        tail.as_mut().unwrap().next = Some(new_node);
        tail = tail.unwrap().next.as_mut();
    }

    while list1_it.is_some() {
        let val = list1_it.as_ref().unwrap().val;
        let new_node = Box::new(ListNode::new(val));
        tail.as_mut().unwrap().next = Some(new_node);
        tail = tail.unwrap().next.as_mut();
        list1_it = list1_it.unwrap().next.as_ref();
    }

    while list2_it.is_some() {
        let val = list2_it.as_ref().unwrap().val;
        let new_node = Box::new(ListNode::new(val));
        tail.as_mut().unwrap().next = Some(new_node);
        tail = tail.unwrap().next.as_mut();
        list2_it = list2_it.unwrap().next.as_ref();
    }

    result.unwrap().next
}

pub fn better_merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    // nó dummy para facilitar construção
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    while let (Some(mut n1), Some(mut n2)) = (list1.take(), list2.take()) {
        if n1.val <= n2.val {
            list1 = n1.next.take(); // avança em list1
            list2 = Some(n2);       // devolve n2
            tail.next = Some(n1);
        } else {
            list2 = n2.next.take(); // avança em list2
            list1 = Some(n1);       // devolve n1
            tail.next = Some(n2);
        }
        tail = tail.next.as_mut().unwrap();
    }

    // liga o resto (qualquer uma que não seja None)
    tail.next = if list1.is_some() { list1 } else { list2 };

    dummy.next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
