// a:ln
use super::tail_end::TailEnd;

#[derive(Default, Debug)]
pub struct Outline {
    tail_end: TailEnd,
}
impl Outline {
    pub fn get_tail_end(&self) -> &TailEnd {
        &self.tail_end
    }

    pub fn get_tail_end_mut(&mut self) -> &mut TailEnd {
        &mut self.tail_end
    }

    pub fn set_tail_end(&mut self, value:TailEnd) {
        self.tail_end = value;
    }
}
