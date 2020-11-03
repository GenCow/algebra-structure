pub mod semigroup;
pub mod monoid;
pub mod group;

use crate::structures::monoid::Monoid;
use crate::structures::group::Group;

pub struct Ring<E,A,M> where A:Group<E=E>, M:Monoid<E=E> {
    pub addition:A,
    pub multiplication:M
}
