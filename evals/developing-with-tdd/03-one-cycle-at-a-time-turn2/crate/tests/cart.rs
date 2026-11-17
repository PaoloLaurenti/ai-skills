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

#[test]
#[ignore]
fn applies_a_promo_code_discount() {}

#[test]
#[ignore]
fn rounds_the_discount_down_to_the_whole_cent() {}

#[test]
#[ignore]
fn a_promo_code_on_an_empty_cart_totals_zero() {}

#[test]
#[ignore]
fn rejects_an_expired_promo_code() {}

#[test]
#[ignore]
fn rejects_an_unknown_promo_code() {}

#[test]
#[ignore]
fn accepts_a_promo_code_on_its_expiry_date() {}
