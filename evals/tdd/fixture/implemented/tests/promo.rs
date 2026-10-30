use shop::cart::{Cart, Item};
use shop::promo::{Checkout, Clock, Date, PromoCatalogue, PromoCode, PromoError};

struct CatalogueDouble {
    codes: Vec<PromoCode>,
}

impl CatalogueDouble {
    fn stub_with(code: &str, percent: u8, expires_on: Date) -> Self {
        Self {
            codes: vec![PromoCode {
                code: code.to_string(),
                percent,
                expires_on,
            }],
        }
    }

    fn stub_empty() -> Self {
        Self { codes: vec![] }
    }
}

impl PromoCatalogue for CatalogueDouble {
    fn find(&self, code: &str) -> Option<PromoCode> {
        self.codes.iter().find(|promo| promo.code == code).cloned()
    }
}

struct ClockDouble {
    today: Date,
}

impl ClockDouble {
    fn stub_fixed_at(today: Date) -> Self {
        Self { today }
    }
}

impl Clock for ClockDouble {
    fn today(&self) -> Date {
        self.today
    }
}

fn cart_worth(price_cents: u32) -> Cart {
    let mut cart = Cart::new();
    cart.add(Item::new("book", price_cents));
    cart
}

#[test]
fn applies_the_promo_code_discount() {
    let catalogue = CatalogueDouble::stub_with("SPRING10", 10, Date::new(2026, 5, 31));
    let clock = ClockDouble::stub_fixed_at(Date::new(2026, 5, 1));
    let checkout = Checkout::new(&catalogue, &clock);

    let total = checkout.total_with_code(&cart_worth(5000), "SPRING10");

    assert_eq!(total, Ok(4500));
}

#[test]
fn rounds_the_discount_down_to_the_cent() {
    let catalogue = CatalogueDouble::stub_with("SPRING10", 10, Date::new(2026, 5, 31));
    let clock = ClockDouble::stub_fixed_at(Date::new(2026, 5, 1));
    let checkout = Checkout::new(&catalogue, &clock);

    let total = checkout.total_with_code(&cart_worth(999), "SPRING10");

    assert_eq!(total, Ok(900));
}

#[test]
fn rejects_an_unknown_code() {
    let catalogue = CatalogueDouble::stub_empty();
    let clock = ClockDouble::stub_fixed_at(Date::new(2026, 5, 1));
    let checkout = Checkout::new(&catalogue, &clock);

    let total = checkout.total_with_code(&cart_worth(5000), "SPRING10");

    assert_eq!(total, Err(PromoError::UnknownCode));
}

#[test]
fn rejects_an_expired_code() {
    let catalogue = CatalogueDouble::stub_with("SPRING10", 10, Date::new(2026, 5, 31));
    let clock = ClockDouble::stub_fixed_at(Date::new(2026, 6, 1));
    let checkout = Checkout::new(&catalogue, &clock);

    let total = checkout.total_with_code(&cart_worth(5000), "SPRING10");

    assert_eq!(total, Err(PromoError::ExpiredCode));
}
