class LinkedList<T> {
    let data: T
    var next: LinkedList? = nil

    init(data: T) {
        self.data = data
    }

    func append(data: T) {
        if let next = next {
            next.append(data: data)
        } else {
            next = LinkedList(data: data)
        }
    }

    func printList() {
        var currentNode = self
        while let nextNode = currentNode.next {
            print(currentNode.data)
            currentNode = nextNode
        }
        print(currentNode.data)
    }
}

func yieldPairs<T>(
    list: LinkedList<T>,
    yieldFn: (T, T) -> Void
) {
    var a = list
    var b = list.next
    while let bVal = b {
        yieldFn(a.data, bVal.data)
        a = a.next!
        b = bVal.next
    }
}

func appendSorted<T: Comparable>(
    list: LinkedList<T>?,
    data: T
) -> LinkedList<T> {
    if let oldHead = list {
        let newNode = LinkedList(data: data)

        if oldHead.data >= data {
            newNode.next = oldHead
            return newNode
        } else {
            var current = oldHead
            while let next = current.next {
                if next.data >= data {
                    break
                }
                current = next
            }

            newNode.next = current.next
            current.next = newNode

            return oldHead
        }
    } else {
        return LinkedList(data: data)
    }
}

func main() {
    var list = LinkedList(data: 1)
    list = appendSorted(list: list, data: 5)
    list = appendSorted(list: list, data: 2)
    list = appendSorted(list: list, data: 3)
    list = appendSorted(list: list, data: 4)
    list.printList()
    yieldPairs(list: list) { a, b in
        print("\(a), \(b)")
    }
}

main()
