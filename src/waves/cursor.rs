use crate::waves::Increment;

#[derive(Copy, Clone, Default, Debug)]
pub(crate) struct WavesCursor {
    wave_idx: usize,
    item_idx: usize,
    item_inner_idx: usize,
}

impl WavesCursor {
    pub(crate) fn wave_idx(&self) -> usize {
        self.wave_idx
    }
    pub(crate) fn item_idx(&self) -> usize {
        self.item_idx
    }
    pub(crate) fn item_inner_idx(&self) -> usize {
        self.item_inner_idx
    }

    pub(crate) fn increment_wave(&mut self) {
        self.wave_idx += 1;
        self.item_idx = 0;
        self.item_inner_idx = 0;
    }

    pub(crate) fn increment_item(&mut self) {
        self.item_idx += 1;
        self.item_inner_idx = 0;
    }

    pub(crate) fn increment_item_inner(&mut self) {
        self.item_inner_idx += 1;
    }
}
