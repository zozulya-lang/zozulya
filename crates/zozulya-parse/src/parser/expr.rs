use zozulya_ir::cst::ZozulyaKind::*;

impl super::Parser<'_> {
    pub fn parse_expr(&mut self) {
        let atom = self.start();
        if self.at(INT) {
            self.bump();
            atom.complete(self, LITERAL);
        } else {
            unreachable!("expected expression")
        }
    }
}
