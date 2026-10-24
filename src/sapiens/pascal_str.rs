use std::ops::Index;

pub(crate) struct PascalStr<'a> {
    data: &'a [u8],
}

impl<'a> PascalStr<'a> {
    pub(crate) const fn new(data: &'a [u8]) -> Self {
        let len = data[0] as usize;
        assert!(len < data.len(), "Invalid Pascal String");
        Self { data }
    }
}

impl PascalStr<'_> {
    pub(crate) const fn len(&self) -> usize {
        self.data[0] as usize
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.data[1..1 + self.len()]
    }
}

impl Index<usize> for PascalStr<'_> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.as_bytes()[index]
    }
}
