impl super::Parser<'_> {
    pub fn parse_expr(&mut self) {
        let atom = self.start();
        if self.at(T![@int]) {
            self.bump();
            atom.complete(self, T![@literal]);
        }
    }
}
