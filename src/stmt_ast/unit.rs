use crate::expr_ast::Expr;

pub enum Stmt<'a> {
    PrintStmt(PrintStmt),
    ExprStmt(ExprStmt),
    VarDeclStmt(VarDeclStmt<'a>),
    BlockStmt(BlockStmt<'a>),
    IfStmt(IfStmt<'a>),
    WhileStmt(WhileStmt<'a>),
    FuncDeclStmt(FuncDeclStmt<'a>),
    ReturnStmt(ReturnStmt),
}

impl From<PrintStmt> for Stmt<'_> {
    fn from(value: PrintStmt) -> Self {
        Self::PrintStmt(value)
    }
}

impl From<ExprStmt> for Stmt<'_> {
    fn from(value: ExprStmt) -> Self {
        Self::ExprStmt(value)
    }
}

impl<'a> From<VarDeclStmt<'a>> for Stmt<'a> {
    fn from(value: VarDeclStmt<'a>) -> Self {
        Self::VarDeclStmt(value)
    }
}

impl<'a> From<BlockStmt<'a>> for Stmt<'a> {
    fn from(value: BlockStmt<'a>) -> Self {
        Self::BlockStmt(value)
    }
}

impl<'a> From<IfStmt<'a>> for Stmt<'a> {
    fn from(value: IfStmt<'a>) -> Self {
        Self::IfStmt(value)
    }
}

impl<'a> From<WhileStmt<'a>> for Stmt<'a> {
    fn from(value: WhileStmt<'a>) -> Self {
        Self::WhileStmt(value)
    }
}

impl<'a> From<FuncDeclStmt<'a>> for Stmt<'a> {
    fn from(value: FuncDeclStmt<'a>) -> Self {
        Self::FuncDeclStmt(value)
    }
}

impl From<ReturnStmt> for Stmt<'_> {
    fn from(value: ReturnStmt) -> Self {
        Self::ReturnStmt(value)
    }
}

pub struct PrintStmt {
    pub expr: Expr,
}

impl From<Expr> for PrintStmt {
    fn from(value: Expr) -> Self {
        Self { expr: value }
    }
}

pub struct ExprStmt {
    pub expr: Expr,
}

impl From<Expr> for ExprStmt {
    fn from(value: Expr) -> Self {
        Self { expr: value }
    }
}

pub struct VarDeclStmt<'a> {
    pub name: &'a str,
    pub initializer: Option<Expr>,
}

impl<'a> From<(&'a str, Option<Expr>)> for VarDeclStmt<'a> {
    fn from(value: (&'a str, Option<Expr>)) -> Self {
        Self {
            name: value.0,
            initializer: value.1,
        }
    }
}

pub struct BlockStmt<'a> {
    pub stmts: Vec<Stmt<'a>>,
}

impl<'a> From<Vec<Stmt<'a>>> for BlockStmt<'a> {
    fn from(value: Vec<Stmt<'a>>) -> Self {
        Self { stmts: value }
    }
}

pub struct IfStmt<'a> {
    pub condition: Expr,
    pub then_branch: Box<Stmt<'a>>,
    pub else_branch: Option<Box<Stmt<'a>>>,
}

impl<'a> From<(Expr, Stmt<'a>, Option<Stmt<'a>>)> for IfStmt<'a> {
    fn from(value: (Expr, Stmt<'a>, Option<Stmt<'a>>)) -> Self {
        Self {
            condition: value.0,
            then_branch: Box::new(value.1),
            else_branch: value.2.map(|s| Box::new(s)),
        }
    }
}

pub struct WhileStmt<'a> {
    pub condition: Expr,
    pub body: Box<Stmt<'a>>,
}

impl<'a> From<(Expr, Stmt<'a>)> for WhileStmt<'a> {
    fn from(value: (Expr, Stmt<'a>)) -> Self {
        Self {
            condition: value.0,
            body: Box::new(value.1),
        }
    }
}

pub struct FuncDeclStmt<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Vec<Stmt<'a>>,
}

impl<'a> From<(&'a str, Vec<&'a str>, Vec<Stmt<'a>>)> for FuncDeclStmt<'a> {
    fn from(value: (&'a str, Vec<&'a str>, Vec<Stmt<'a>>)) -> Self {
        Self {
            name: value.0,
            params: value.1,
            body: value.2,
        }
    }
}

pub struct ReturnStmt {
    pub value: Option<Expr>,
}

impl From<Option<Expr>> for ReturnStmt {
    fn from(value: Option<Expr>) -> Self {
        Self { value }
    }
}
