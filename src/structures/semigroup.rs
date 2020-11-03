use std::intrinsics::copy;

pub trait SemiGroup {
    type E;

    fn new_element(&self) -> Self::E;

    //cheap operation
    fn cop(&self, r:&mut Self::E, o1:&Self::E, o2:&Self::E);
    //operation
    #[inline]
    fn op(&self, o1:&Self::E, o2:&Self::E) -> Self::E {
        let mut r = self.new_element();

        self.cop(&mut r, o1, o2);

        return r;

    }

    //cheap operation auto-assign
    //r = r*o
    #[inline]
    fn copa(&self, r:&mut Self::E, o:&Self::E) {
        let mut o1 = self.new_element();

        unsafe {copy(r, &mut o1, 1);}

        self.cop(r, &o1, o);
    }
}