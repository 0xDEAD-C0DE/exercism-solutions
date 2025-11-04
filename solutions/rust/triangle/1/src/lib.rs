use std::ops::Add;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T: From<i32> + Copy + PartialEq + PartialOrd + Add<Output = T>> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides[0] == T::from(0) && sides[1] == T::from(0) && sides[2] == T::from(0) {
            return None;
        }
        if sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[0] + sides[2] >= sides[1]
        {
            Some(Triangle {
                a: sides[0],
                b: sides[1],
                c: sides[2],
            })
        } else {
            None
        }
        // todo!("Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid.");
    }

    pub fn is_equilateral(&self) -> bool {
        self.a == self.b && self.a == self.c && self.b == self.c && self.b == self.a
    }

    pub fn is_scalene(&self) -> bool {
        self.a != self.b && self.b != self.c && self.c != self.a
    }

    pub fn is_isosceles(&self) -> bool {
        self.a == self.b || self.b == self.c || self.c == self.a
    }
}