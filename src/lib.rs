pub trait MovingAverageWeight {
    // Higher weight discounts older observations faster.
    fn weight() -> f32;
}

pub struct MovingAverage<W: MovingAverageWeight> {
    weight: std::marker::PhantomData<W>,
    avg: f32,
}

impl<W: MovingAverageWeight> MovingAverage<W> {
    pub fn new(init: f32) -> MovingAverage<W> {
        MovingAverage {
            weight: std::marker::PhantomData,
            avg: init,
        }
    }

    pub fn set(&mut self, val: f32) { self.avg = val; }
    pub fn get(&self) -> f32 { self.avg }

    pub fn add(&mut self, val: f32) -> f32 {
        self.avg = self.smooth(val);
        self.avg
    }

    fn smooth(&self, val: f32) -> f32 {
        W::weight() * val + (1.0 - W::weight()) * self.avg
    }
}
