use crate::structures::semigroup::SemiGroup;

pub trait Monoid : SemiGroup {
    //cheap neutral element
    fn cne(&self, r:&mut Self::E);
    //neutral element
    #[inline]
    fn ne(&self) -> Self::E {
        let mut r = self.new_element();

        self.cne(&mut r);

        return r;
    }

    #[inline]
    fn is_neutral(&self, o:&Self::E) -> bool where Self::E : PartialEq{
        let mut r = self.new_element();
        self.cne(&mut r);

        return &r == o;
    }

    #[inline]
    fn has_inverse(&self, o:&Self::E) -> bool where Self::E : PartialEq {
        self.is_neutral(o)
    }

    #[inline]
    fn try_inv(&self, r:&mut Option<Self::E>, o:&Self::E) where Self::E : PartialEq {
        if self.has_inverse(o) {
            let mut content = self.new_element();
            self.cne(&mut content);
            *r = Some(content);
        } else {
            *r = None;
        }
    }
}