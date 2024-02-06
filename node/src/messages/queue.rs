pub struct Queue<T> {
    items: Vec<T>,
    max_items: usize,
}

impl<T> Queue<T> {
    pub fn new(max_items: usize) -> Self {
        Queue {
            items: Vec::new(),
            max_items,
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.push(item);
        if self.items.len() > self.max_items {
            self.items.remove(0);
        }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }
}
