use crate::expr_ast::Expr;

pub enum Stmt {
    PrintStmt(PrintStmt),
    ExprStmt(ExprStmt),
    VarDeclStmt(VarDeclStmt),
    BlockStmt(BlockStmt),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    FuncDeclStmt(FuncDeclStmt),
    ReturnStmt(ReturnStmt),
}

impl From<PrintStmt> for Stmt {
    fn from(value: PrintStmt) -> Self {
        Self::PrintStmt(value)
    }
}

impl From<ExprStmt> for Stmt {
    fn from(value: ExprStmt) -> Self {
        Self::ExprStmt(value)
    }
}

impl From<VarDeclStmt> for Stmt {
    fn from(value: VarDeclStmt) -> Self {
        Self::VarDeclStmt(value)
    }
}

impl From<BlockStmt> for Stmt {
    fn from(value: BlockStmt) -> Self {
        Self::BlockStmt(value)
    }
}

impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::IfStmt(value)
    }
}

impl From<WhileStmt> for Stmt {
    fn from(value: WhileStmt) -> Self {
        Self::WhileStmt(value)
    }
}

impl From<FuncDeclStmt> for Stmt {
    fn from(value: FuncDeclStmt) -> Self {
        Self::FuncDeclStmt(value)
    }
}

impl From<ReturnStmt> for Stmt {
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

pub struct VarDeclStmt {
    pub name: String,
    pub initializer: Option<Expr>,
}

impl From<(String, Option<Expr>)> for VarDeclStmt {
    fn from(value: (String, Option<Expr>)) -> Self {
        Self {
            name: value.0,
            initializer: value.1,
        }
    }
}

pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

impl From<Vec<Stmt>> for BlockStmt {
    fn from(value: Vec<Stmt>) -> Self {
        Self { stmts: value }
    }
}

pub struct IfStmt {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

impl From<(Expr, Stmt, Option<Stmt>)> for IfStmt {
    fn from(value: (Expr, Stmt, Option<Stmt>)) -> Self {
        Self {
            condition: value.0,
            then_branch: Box::new(value.1),
            else_branch: value.2.map(|s| Box::new(s)),
        }
    }
}

pub struct WhileStmt {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl From<(Expr, Stmt)> for WhileStmt {
    fn from(value: (Expr, Stmt)) -> Self {
        Self {
            condition: value.0,
            body: Box::new(value.1),
        }
    }
}

pub struct FuncDeclStmt {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

impl From<(String, Vec<String>, Vec<Stmt>)> for FuncDeclStmt {
    fn from(value: (String, Vec<String>, Vec<Stmt>)) -> Self {
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
