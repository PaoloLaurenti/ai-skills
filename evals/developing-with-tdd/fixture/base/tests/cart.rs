use shop::cart::{Cart, Item};

#[test]
fn totals_the_prices_of_all_items() {
    let mut cart = Cart::new();
    cart.add(Item::new("book", 1200));
    cart.add(Item::new("pen", 300));

    let total = cart.total();

    assert_eq!(total, 1500);
}

#[test]
fn an_empty_cart_totals_zero() {
    let cart = Cart::new();

    let total = cart.total();

    assert_eq!(total, 0);
}
