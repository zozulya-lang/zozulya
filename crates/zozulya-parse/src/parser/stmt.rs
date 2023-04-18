use zozulya_ir::cst::ZozulyaKind::*;

impl super::Parser<'_> {
    pub fn parse_stmt(&mut self) {
        let stmt = self.start();
        if self.at(IDENT) {
            self.bump();
            self.expect(ASSIGN);
            self.parse_expr();
            stmt.complete(self, LOCAL);
        } else {
            unreachable!("expected statement")
        }
    }
}
