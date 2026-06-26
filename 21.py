from typing import Optional


class ListNode:
    next: None | int

    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next


def mergeTwoLists(
    list1: Optional[ListNode], list2: Optional[ListNode]
) -> Optional[ListNode]:
    head = ListNode()
    tail = head

    while list1 and list2:
        if list1.val <= list2.val:
            tail.next = list1
            list1 = list1.next
        else:
            tail.next = list2
            list2 = list2.next
        tail = tail.next

    # get the remainder of one list if its run out
    tail.next = list1 if list1 else list2
    return head.next


def build_linked_list(arr):
    if not arr:
        return None
    head = ListNode(arr[0])
    current = head
    for val in arr[1:]:
        current.next = ListNode(val)
        current = current.next
    return head


def print_linked_list(head):
    elements = []
    current = head
    while current:
        elements.append(str(current.val))
        current = current.next
    print(" -> ".join(elements) if elements else "Empty List")


l1 = build_linked_list([1, 2, 4])
l2 = build_linked_list([1, 3, 4])

result = mergeTwoLists(l1, l2)

print("Merged Result:")
print_linked_list(result)
