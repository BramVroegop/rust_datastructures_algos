pub trait Hashable {
    fn hash(&self) -> usize;
    fn equals(&self, other: &Self) -> bool;
}

pub struct HashTable<Key: Hashable, Value> {
    buckets: Vec<Option<Box<(Key, Value)>>>,
}

impl<Key: Hashable, Value> HashTable<Key, Value> {
    pub fn new() -> Self {
        Self {
            buckets: Vec::<Option<Box<(Key, Value)>>>::with_capacity(256),
        }
    }

    pub fn insert(&mut self, pair: (Key, Value)) {
        let hash = pair.0.hash() % self.buckets.capacity();

        let boxed = Box::new(pair);

        for i in self.buckets.iter_mut().take(hash - 1) {
            if let None = i {
                *i = Some(boxed);
                return;
            }
        }
        self.buckets.push(Some(boxed));
    }

    pub fn get(&mut self, key: Key) -> Option<Box<(Key, Value)>> {
        let capacity = self.buckets.capacity();
        let hash = key.hash() % capacity;

        for i in self.buckets.iter_mut().take(hash - 1) {
            if let Some(ref bucket) = i {
                if bucket.0.equals(&key) {
                    return i.take();
                }
            }
        }
        None
    }

    pub fn get_ref(&self, key: Key) -> Option<&Value> {
        let hash = key.hash() % self.buckets.capacity();

        for i in self.buckets.iter().take(hash - 1) {
            if let Some(ref bucket) = i {
                if bucket.0.equals(&key) {
                    return Some(&bucket.1);
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: Key) -> Option<&mut Value> {
        let hash = key.hash() % self.buckets.capacity();

        for i in self.buckets.iter_mut().take(hash - 1) {
            if let Some(ref mut bucket) = i {
                if bucket.0.equals(&key) {
                    return Some(&mut bucket.1);
                }
            }
        }
        None
    }
}
