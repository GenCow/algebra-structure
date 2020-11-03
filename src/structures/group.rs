use crate::structures::monoid::Monoid;
use std::mem::MaybeUninit;

pub trait Group : Monoid {
    //cheap inverse
    fn cinv(&self, r:&mut Self::E, o:&Self::E);
    //inverse
    #[inline]
    fn inv(&self, o:&Self::E) -> Self::E {
        let mut r = self.new_element();

        self.cinv(&mut r, o);

        return r;
    }

    #[inline]
    fn has_inverse(&self, _:&Self::E) -> bool {
        true
    }

    #[inline]
    fn try_inv(&self, r:&mut Option<Self::E>, o:&Self::E) where Self::E : PartialEq {
        let mut content = self.new_element();
        self.cinv(&mut content,o);
        *r = Some(content);
    }


    //cheap anti-operation
    //r = o1/o2
    #[inline]
    fn caop(&self, r:&mut Self::E, o1:&Self::E, o2:&Self::E) {
        let mut i: Self::E =  self.new_element();

        self.cinv(&mut i, o2);
        self.cop(r, o1, &i);
    }

    //r = o1^o2
    #[inline]
    fn cpower(&self, r:&mut Self::E, o1:&Self::E, o2:i64) {
        self.cne(r);

        if o2 > 0 {
            for _ in 0..o2 {
                self.copa(r, o1);
            }
        } else if o2 < 0 {
            unsafe {
                let mut inv: Self::E = MaybeUninit::uninit().assume_init();

                self.cinv(&mut inv, o1);

                for _ in 0..o2 {
                    self.copa(r, &inv);
                }
            }
        }
    }
}

