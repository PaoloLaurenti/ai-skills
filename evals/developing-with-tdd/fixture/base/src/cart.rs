#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub price_cents: u32,
}

impl Item {
    pub fn new(name: &str, price_cents: u32) -> Self {
        Self {
            name: name.to_string(),
            price_cents,
        }
    }
}

#[derive(Debug, Default)]
pub struct Cart {
    items: Vec<Item>,
}

impl Cart {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn total(&self) -> u32 {
        self.items.iter().map(|item| item.price_cents).sum()
    }
}
