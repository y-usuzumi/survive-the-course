export class ListNode<T> {
    val: T;
    next: ListNode<T> | null;

    constructor(val: T) {
        this.val = val;
        this.next = null;
    }

    prepend(val: T): ListNode<T> {
        const newNode = new ListNode(val);
        newNode.next = this;
        return newNode;
    }

    append(val: T) {
        let currNode: ListNode<T> = this;
        while (currNode.next !== null) {
            currNode = currNode.next;    
        }
        this.next = new ListNode(val);
    }
}

export class LinkedList<T> {
    head: ListNode<T> | null;
    last: ListNode<T> | null;
    size: number;

    constructor() {
        this.head = null;
        this.last = null;
        this.size = 0;
    }

    append(val: T) {
        if (this.last === null) {
            this.head = this.last = new ListNode(val);
        } else {
            const newNode = new ListNode(val);
            this.last.next = newNode;
            this.last = newNode;
        }
        this.size++;
    }

    prepend(val: T) {
        if (this.head === null) {
            this.head = this.last = new ListNode(val);
        } else {
            const newNode = new ListNode(val);
            newNode.next = this.head;
            this.head = newNode;
        }
        this.size++;
    }

    remove(f: (t: T) => boolean): T | null {
        if (this.head === null) {
            return null;
        }

        if (f(this.head.val)) {
            const oldHead = this.head;
            const newHead = oldHead.next;
            oldHead.next = null;
            this.head = newHead;
            if (this.last === oldHead) {
                this.last = null;
            }
            return oldHead.val;
        } else {
            let prev = this.head;
            let curr = this.head.next;
            while (curr !== null) {
                if (f(curr.val)) {
                    prev.next = curr.next;
                    curr.next = null;
                    if (this.last === curr) {
                        this.last = prev;
                    }
                    return curr.val;
                }
                prev = curr;
                curr = curr.next;
            }
        }
        return null;
    }

    find(f: (t: T) => boolean): T | null {
        if (this.head === null) {
            return null;
        }

        let currNode: ListNode<T> | null = this.head;
        while (currNode !== null) {
            if (f(currNode.val)) {
                return currNode.val;
            }
            currNode = currNode.next;
        }
        return null;
    }

    toArray(): T[] {
        if (this.head === null) {
            return [];
        }
        const result: T[] = [];
        let currNode: ListNode<T> | null = this.head;
        while (currNode !== null) {
            result.push(currNode.val);
            currNode = currNode.next;
        }
        return result;
    }

    static fromArray<T>(arr: T[]): LinkedList<T> {
        const result: LinkedList<T> = new LinkedList();
        for (const item of arr) {
            result.append(item);
        }
        return result;
    }
}