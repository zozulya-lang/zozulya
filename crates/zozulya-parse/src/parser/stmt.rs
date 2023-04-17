impl super::Parser<'_> {
    pub fn parse_stmt(&mut self) {
        let stmt = self.start();
        if self.at(T![@ident]) {
            self.bump();
            self.expect(T![:=]);
            self.parse_expr();
            stmt.complete(self, T![@local]);
        } else {
            unreachable!("expected statement")
        }
    }
}
