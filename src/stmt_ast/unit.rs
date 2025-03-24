use crate::expr_ast::Expr;

#[derive(Debug, Clone)]
pub enum Stmt {
    BlockStmt(BlockStmt),
    ExprStmt(ExprStmt),
    ForStmt(ForStmt),
    FuncDeclStmt(FuncDeclStmt),
    IfStmt(IfStmt),
    PrintStmt(PrintStmt),
    #[allow(dead_code)]
    ReturnStmt(ReturnStmt),
    VarDeclStmt(VarDeclStmt),
    WhileStmt(WhileStmt),
}

// From implementations for Stmt
impl From<BlockStmt> for Stmt {
    fn from(value: BlockStmt) -> Self {
        Self::BlockStmt(value)
    }
}

impl From<ExprStmt> for Stmt {
    fn from(value: ExprStmt) -> Self {
        Self::ExprStmt(value)
    }
}

impl From<ForStmt> for Stmt {
    fn from(value: ForStmt) -> Self {
        Self::ForStmt(value)
    }
}

impl From<FuncDeclStmt> for Stmt {
    fn from(value: FuncDeclStmt) -> Self {
        Self::FuncDeclStmt(value)
    }
}

impl From<IfStmt> for Stmt {
    fn from(value: IfStmt) -> Self {
        Self::IfStmt(value)
    }
}

impl From<PrintStmt> for Stmt {
    fn from(value: PrintStmt) -> Self {
        Self::PrintStmt(value)
    }
}

impl From<VarDeclStmt> for Stmt {
    fn from(value: VarDeclStmt) -> Self {
        Self::VarDeclStmt(value)
    }
}

impl From<WhileStmt> for Stmt {
    fn from(value: WhileStmt) -> Self {
        Self::WhileStmt(value)
    }
}

impl From<ReturnStmt> for Stmt {
    fn from(value: ReturnStmt) -> Self {
        Self::ReturnStmt(value)
    }
}

// PrintStmt and its implementation
#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub expr: Expr,
}

impl From<Expr> for PrintStmt {
    fn from(value: Expr) -> Self {
        Self { expr: value }
    }
}

// ExprStmt and its implementation
#[derive(Debug, Clone)]
pub struct ExprStmt {
    pub expr: Expr,
}

impl From<Expr> for ExprStmt {
    fn from(value: Expr) -> Self {
        Self { expr: value }
    }
}

// VarDeclStmt and its implementation
#[derive(Debug, Clone)]
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

// BlockStmt and its implementation
#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub stmts: Vec<Stmt>,
}

impl From<Vec<Stmt>> for BlockStmt {
    fn from(value: Vec<Stmt>) -> Self {
        Self { stmts: value }
    }
}

// IfStmt and its implementation
#[derive(Debug, Clone)]
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

// WhileStmt and its implementation
#[derive(Debug, Clone)]
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

#[allow(dead_code)]
/// FuncDeclStmt and its implementation
#[derive(Debug, Clone)]
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

#[allow(dead_code)]
// ReturnStmt and its implementation
#[derive(Debug, Clone)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
}

impl From<Option<Expr>> for ReturnStmt {
    fn from(value: Option<Expr>) -> Self {
        Self { value }
    }
}

// ForStmt and its implementation
#[derive(Debug, Clone)]
pub struct ForStmt {
    pub initializer: Option<Box<Stmt>>,
    pub condition: Option<Expr>,
    pub increment: Option<Expr>,
    pub body: Box<Stmt>,
}

impl From<(Option<Stmt>, Option<Expr>, Option<Expr>, Stmt)> for ForStmt {
    fn from(value: (Option<Stmt>, Option<Expr>, Option<Expr>, Stmt)) -> Self {
        Self {
            initializer: value.0.map(|s| Box::new(s)),
            condition: value.1,
            increment: value.2,
            body: Box::new(value.3),
        }
    }
}
