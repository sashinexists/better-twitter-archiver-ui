#[derive(Debug, Clone)]
pub struct SelectList<T> {
    pub previous: Vec<T>,
    pub selected: T,
    pub next: Vec<T>,
}

impl<T: Clone> SelectList<T> {
    pub fn previous(&mut self) {
        if let Some(mut new) = self.previous.pop() {
            std::mem::swap(&mut self.selected, &mut new);
            self.next.insert(0, new);
        }
    }

    pub fn forward(&mut self) {
        if !self.next.is_empty() {
            let mut new = self.next.remove(0);
            std::mem::swap(&mut self.selected, &mut new);
            self.previous.push(new);
        }
    }

    pub fn add(&mut self, item: T) {
        self.next.clear();
        self.previous.push(self.selected.clone());
        self.selected = item;
    }

    pub fn new(item: T) -> SelectList<T> {
        SelectList {
            previous: Vec::<T>::new(),
            selected: item,
            next: Vec::<T>::new(),
        }
    }
}
