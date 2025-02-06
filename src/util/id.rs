pub(crate) struct IdProvider {
    pub(crate) next_id: u32,
}

impl IdProvider {
    pub(crate) fn next_id(&mut self) -> u32 {
        let next_id = self.next_id;
        self.next_id = self
            .next_id
            .checked_add(1)
            .expect("explicit overflow check");
        next_id
    }
}
