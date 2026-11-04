use std::ops::Deref;

pub(crate) struct ChaineC {
    data: Vec<u8>,
}

impl ChaineC {
    pub(crate) fn concat(&mut self, txt: &[u8]) {
        let len = self.data.len() - 1; // data[len] == 0
        self.data.truncate(len);
        self.data
            .extend(txt.iter().copied().take_while(|&c| c != 0));
        self.data.push(0);
    }
}

impl From<&[u8]> for ChaineC {
    fn from(value: &[u8]) -> Self {
        let len = value.iter().position(|&c| c == 0).unwrap_or(value.len());
        let mut data = Vec::with_capacity(len + 1);
        data.extend_from_slice(&value[..len]);
        data.push(0);
        Self { data }
    }
}

impl Deref for ChaineC {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
