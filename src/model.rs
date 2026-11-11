use std::ops::ShrAssign;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct Point {
    pub(crate) h: i16,
    pub(crate) v: i16,
}

impl Point {
    pub(crate) const fn new(h: i16, v: i16) -> Self {
        Self { h, v }
    }

    pub(crate) const fn zoom(mut self, zoom: f32) -> Self {
        self.v = (self.v as f32 / zoom) as i16;
        self.h = (self.h as f32 / zoom) as i16;
        self
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct Rect {
    pub(crate) top: i16,
    pub(crate) left: i16,
    pub(crate) bottom: i16,
    pub(crate) right: i16,
}

impl Rect {
    pub(crate) const fn new(left: i16, top: i16, right: i16, bottom: i16) -> Self {
        Rect {
            top,
            left,
            bottom,
            right,
        }
    }

    pub(crate) const fn OffsetRect(&mut self, dh: i16, dv: i16) {
        self.left = self.left.wrapping_add(dh);
        self.right = self.right.wrapping_add(dh);
        self.top = self.top.wrapping_add(dv);
        self.bottom = self.bottom.wrapping_add(dv);
    }

    pub(crate) const fn InsetRect(&mut self, dh: i16, dv: i16) {
        self.left = self.left.wrapping_add(dh);
        self.right = self.right.wrapping_sub(dh);
        self.top = self.top.wrapping_add(dv);
        self.bottom = self.bottom.wrapping_sub(dv);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.right <= self.left || self.bottom <= self.top
    }

    pub(crate) fn intersect(&self, other: &Rect) -> Rect {
        Rect {
            left: self.left.max(other.left),
            top: self.top.max(other.top),
            right: self.right.min(other.right),
            bottom: self.bottom.min(other.bottom),
        }
    }

    pub(crate) fn union(&self, other: &Self) -> Self {
        if self.is_empty() {
            return *other;
        }
        if other.is_empty() {
            return *self;
        }
        Self {
            left: self.left.min(other.left),
            top: self.top.min(other.top),
            right: self.right.max(other.right),
            bottom: self.bottom.max(other.bottom),
        }
    }

    pub(crate) const fn pt_in_rect(&self, pt: Point) -> bool {
        pt.h >= self.left && pt.h < self.right && pt.v >= self.top && pt.v < self.bottom
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(crate) struct RGBColor {
    pub(crate) red: u16,
    pub(crate) green: u16,
    pub(crate) blue: u16,
}

impl ShrAssign<u8> for RGBColor {
    fn shr_assign(&mut self, rhs: u8) {
        self.red >>= rhs;
        self.green >>= rhs;
        self.blue >>= rhs;
    }
}
