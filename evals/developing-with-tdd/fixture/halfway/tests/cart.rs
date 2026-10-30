use shop::cart::{Cart, Item};

#[test]
fn totals_the_price_of_a_single_item() {
    let mut cart = Cart::new();
    cart.add(Item::new("book", 1200));

    let total = cart.total();

    assert_eq!(total, 1200);
}

#[test]
#[ignore]
fn totals_the_prices_of_all_items() {}

#[test]
#[ignore]
fn an_empty_cart_totals_zero() {}
