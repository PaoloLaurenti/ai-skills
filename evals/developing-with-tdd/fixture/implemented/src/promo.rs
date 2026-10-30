use crate::cart::Cart;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromoCode {
    pub code: String,
    pub percent: u8,
    pub expires_on: Date,
}

pub trait PromoCatalogue {
    fn find(&self, code: &str) -> Option<PromoCode>;
}

pub trait Clock {
    fn today(&self) -> Date;
}

#[derive(Debug, PartialEq, Eq)]
pub enum PromoError {
    UnknownCode,
    ExpiredCode,
}

pub struct Checkout<'a> {
    catalogue: &'a dyn PromoCatalogue,
    clock: &'a dyn Clock,
}

impl<'a> Checkout<'a> {
    pub fn new(catalogue: &'a dyn PromoCatalogue, clock: &'a dyn Clock) -> Self {
        Self { catalogue, clock }
    }

    pub fn total_with_code(&self, cart: &Cart, code: &str) -> Result<u32, PromoError> {
        let promo = self
            .catalogue
            .find(code)
            .ok_or(PromoError::UnknownCode)?;
        if self.clock.today() >= promo.expires_on {
            return Err(PromoError::ExpiredCode);
        }
        let total = cart.total();
        let discount = total * u32::from(promo.percent) / 100;
        Ok(total - discount)
    }
}
