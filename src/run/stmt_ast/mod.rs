mod print;

pub(super) use print::Print;

#[derive(Debug, Clone)]
pub(crate) enum StmtAst {
    Print(Print),
}

impl From<Print> for StmtAst {
    fn from(print: Print) -> Self {
        StmtAst::Print(print)
    }
}
