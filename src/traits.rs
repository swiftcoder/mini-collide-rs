pub trait Distance<Other> {
    /// The distance between two objects
    fn distance(&self, other: Other) -> f32;
}
