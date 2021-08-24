use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Default)]
struct Data(usize);

// Custom clone implementation in which new data have counter incremented by one
impl Clone for Data {
    fn clone(&self) -> Self {
        Self(self.0 + 1)
    }
}

// Container is `Clone` because `Data` are `Clone` and so are `Box`, `Rc` and references
#[derive(Clone, Debug)]
struct Container<'a> {
    // Owned data located on the stack.
    owned: Data,
    // Immutably shared data located on the stack that must outlive container's lifetime.
    stack_shared: &'a Data,
    // Pointer to owned data located on the heap.
    heap_owned: Box<Data>,
    // Reference counter poining to shared data located on the heap.
    heap_shared: Rc<Data>,
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        // Allocate new data on the stack
        let stack_data = Data::default();

        // Allocate new reference-counted data on the heap
        let rc_data = Rc::new(Data::default());

        // Allocate two containers on the heap (behind a `Box`)

        let box1 = Box::new(Container {
            owned: Data::default(),
            stack_shared: &stack_data,
            heap_owned: Data::default().into(),
            heap_shared: rc_data.clone(),
        });

        let box2 = Box::new(Container {
            owned: Data::default(),
            stack_shared: &stack_data,
            heap_owned: Data::default().into(),
            heap_shared: rc_data.clone(),
        });

        // Clone both containers
        let clone1 = box1.clone();
        let clone2 = box2.clone();

        // Owned data are cloned because the container is behind a `Box`
        assert_eq!(clone1.owned.0, 1);
        assert_eq!(clone2.owned.0, 1);

        // Data behind `&` are *not* cloned, only the pointer is trivially copied
        assert_eq!(stack_data.0, 0);
        assert_eq!(clone1.stack_shared.0, 0);
        assert_eq!(clone2.stack_shared.0, 0);

        // Heap allocated data (behind a `Box`) are cloned (also for the same reason as above)
        assert_eq!(clone1.heap_owned.0, 1);
        assert_eq!(clone2.heap_owned.0, 1);

        // Data behind `Rc` are *not* cloned, only the pointer is (and the counter is incremented)
        assert_eq!(rc_data.0, 0);
        assert_eq!(clone1.heap_shared.0, 0);
        assert_eq!(clone2.heap_shared.0, 0);

        // Heap data behind `Rc` are still valid after a shared reference is dropped
        drop(box1);
        assert_eq!(rc_data.0, 0);
        assert_eq!(clone2.heap_shared.0, 0);
    }
}
