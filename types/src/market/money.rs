use crate::{
    prelude::*,
};
use std::ops::{Mul};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::fmt::Formatter;

#[derive(Add, Sum, Sub, SubAssign, Div, AddAssign, MulAssign, From, Into, Copy, Clone, PartialEq, PartialOrd)]
pub struct Money(pub f64);

impl Money {
    pub fn neg(&self) -> Self {
        Money(-1. * self.0)
    }
    pub fn rneg<T: AsRef<Self>>(t: T) -> Self {
        Money(-1. * (t.as_ref()).0)
    }
}

impl Debug for Money {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&format!("${:.2}", self.0))
    }
}

impl Mul<Money> for Money {
    type Output = Money;
    fn mul(self, rhs: Money) -> Money {
        Money(self.0.mul(rhs.0))
    }
}

impl<__RhsT> ::core::ops::Mul<__RhsT> for Money
    where
        f64: ::core::ops::Mul<__RhsT, Output=f64>,
{
    type Output = Money;
    #[inline]
    fn mul(self, rhs: __RhsT) -> Money {
        Money(<f64 as ::core::ops::Mul<__RhsT>>::mul(self.0, rhs))
    }
}

impl Mul<Money> for f64 {
    type Output = Money;

    #[inline]
    fn mul(self, rhs: Money) -> Money {
        Money(rhs.0 * self)
    }
}

impl AsRef<Money> for Money {
    fn as_ref(&self) -> &Money {
        self
    }
}

impl Eq for Money {}

impl Ord for Money {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect(&*format!("Failed ordering {:?} cmp {:?}", self, other))
    }
}
