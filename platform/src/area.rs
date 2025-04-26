
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area(pub usize);

impl Area {
    pub fn empty() -> Self {
        Self(0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
