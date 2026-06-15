// @generated
#[allow(non_camel_case_types,unused)]

#[derive(Clone,Copy,Debug)]
pub struct Span{pub sl:u32,pub sc:u32,pub el:u32,pub ec:u32}
impl Span{pub fn d()->Self{Self{sl:0,sc:0,el:0,ec:0}}pub fn mg(&self,o:&Span)->Self{Self{sl:self.sl,sc:self.sc,el:o.el,ec:o.ec}}}

#[derive(Clone,PartialEq,Debug)]
pub enum TK{Ident,IntLit,StrLit,Fn,Ret,Plus,Star,S,LB,RB,LP,RP,EOF}

#[derive(Clone,Debug)]
pub struct Tok{pub k:TK,pub s:Span,pub v:String}

pub struct Lex{pub c:Vec<char>,pub p:usize,pub l:u32,pub col:u32}

impl Lex{pub fn new(s:&str)->Self{Self{c:s.chars().collect(),p:0,l:1,col:1}}
pub fn tkz(&mut self)->Vec<Tok>{let mut t=Vec::new();loop{self.skip();if self.p>=self.c.len(){break}match self.c[self.p]{
'"'=>t.push(self.rs()),
c if c.is_ascii_digit()=>t.push(self.rn()),
c if c.is_alphabetic()||c=='_'=>t.push(self.ri()),
'+'=>t.push(self.rf("+",TK::Plus)),
'*'=>t.push(self.rf("*",TK::Star)),
';'=>t.push(self.rf(";",TK::S)),
'{'=>t.push(self.rf("{",TK::LB)),
'}'=>t.push(self.rf("}",TK::RB)),
'('=>t.push(self.rf("(",TK::LP)),
')'=>t.push(self.rf(")",TK::RP)),
_=>panic!("bad '{}'",self.c[self.p])}}
t.push(Tok{k:TK::EOF,s:Span::d(),v:String::new()});t}
fn skip(&mut self){loop{let c=self.c.get(self.p);match c{Some(' '|'\t'|'\r')=>{self.p+=1;self.col+=1;}Some('\n')=>{self.p+=1;self.l+=1;self.col=1;}Some('/')if self.p+1<self.c.len()&&self.c[self.p+1]=='/'=>{while self.p<self.c.len(){if self.c[self.p]=='\n'{break}self.p+=1;}}_=>{break}}}}
fn rs(&mut self)->Tok{let(sl,sc)=(self.l,self.col);self.p+=1;let mut v=String::new();while self.p<self.c.len()&&self.c[self.p]!='"'{if self.c[self.p]=='\\'{self.p+=1;self.col+=1;match self.c.get(self.p){Some('n')=>v.push('\n'),Some('t')=>v.push('\t'),Some('0')=>v.push('\0'),Some(c)=>v.push(*c),None=>{}}}else{v.push(self.c[self.p]);}self.p+=1;self.col+=1;}if self.p<self.c.len(){self.p+=1;}Tok{k:TK::StrLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn rn(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_ascii_digit()||self.c[self.p]=='.'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();Tok{k:TK::IntLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn ri(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_alphanumeric()||self.c[self.p]=='_'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();let k=match v.as_str(){"fn"=>TK::Fn,"return"=>TK::Ret,_=>TK::Ident};Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn rf(&mut self,s:&str,k:TK)->Tok{let(sl,sc)=(self.l,self.col);self.p+=s.len();self.col+=s.len()as u32;Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v:s.to_string()}}
}

// AST
#[derive(Clone,Debug)]
pub struct AIdent{pub s:Span,pub v:String}
#[derive(Clone,Debug)]
pub struct AInt{pub s:Span,pub v:i64}

#[derive(Clone,Debug)]
pub enum AN{Ident(Box<AIdent>),Int(Box<AInt>),Program(Box<AProgram>),Stmt(Box<AStmt>),FnDecl(Box<AFnDecl>),ReturnStmt(Box<AReturnStmt>),Expr(Box<AExpr>),BinaryExpr(Box<ABinaryExpr>),}

#[derive(Clone,Debug)]
pub struct AProgram{pub s:Span,pub stmt:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AFnDecl{pub s:Span,pub ident:Box<AN>,pub stmt:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AReturnStmt{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AExpr{pub s:Span,}

#[derive(Clone,Debug)]
pub struct ABinaryExpr{pub s:Span,pub expr:Box<AN>,pub operator:Box<AN>,pub expr_1:Box<AN>,}

// HIR
#[derive(Clone,Debug)]
pub struct HIdent{pub s:Span}
#[derive(Clone,Debug)]
pub struct HInt{pub s:Span,pub v:i64}

#[derive(Clone,Debug)]
pub enum HN{Ident(Box<HIdent>),Int(Box<HInt>),HirFnDecl(Box<HHirFnDecl>),HirReturn(Box<HHirReturn>),HirInt(Box<HHirInt>),HirAdd(Box<HHirAdd>),}

#[derive(Clone,Debug)]
pub struct HHirFnDecl{pub s:Span,pub fn_decl:Box<HN>,}

#[derive(Clone,Debug)]
pub struct HHirReturn{pub s:Span,pub return_stmt:Box<HN>,}

#[derive(Clone,Debug)]
pub struct HHirInt{pub s:Span,pub int_literal:Box<HN>,}

#[derive(Clone,Debug)]
pub struct HHirAdd{pub s:Span,pub binary_expr:Box<HN>,}

// Parser
pub struct P{pub t:Vec<Tok>,pub p:usize}
impl P{pub fn new(t:Vec<Tok>)->Self{Self{t,p:0}}
fn op_prec(&self,k:&TK)->u32{match k{
TK::Plus => 5,
TK::Star => 6,
_=>0}}
pub fn pprogram(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let mut stmts:Vec<AN>=Vec::new();
while let Ok(v)=self.pstmt(){stmts.push(v)}
Ok(AN::Program(Box::new(AProgram {s:Span::d(),stmt:stmts,})))
}
pub fn pstmt(&mut self)->Result<AN,String>{
if let Ok(v)=self.pfn_decl(){return Ok(v)}
if let Ok(v)=self.preturn_stmt(){return Ok(v)}
Err(format!("no alt for Stmt at {}",self.tok().s.sl))
}
pub fn pfn_decl(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Fn)?;
let a0=self.pi()?;
self.e(TK::LP)?;
self.e(TK::RP)?;
self.e(TK::LB)?;
let mut stmts:Vec<AN>=Vec::new();
while let Ok(v)=self.pstmt(){stmts.push(v)}
self.e(TK::RB)?;
Ok(AN::FnDecl(Box::new(AFnDecl {s:Span::d(),ident:Box::new(a0),stmt:stmts,})))
}
pub fn preturn_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Ret)?;
let a0=self.pexpr()?;
self.e(TK::S)?;
Ok(AN::ReturnStmt(Box::new(AReturnStmt {s:Span::d(),expr:Box::new(a0),})))
}
pub fn pexpr(&mut self)->Result<AN,String>{
if matches!(self.tok().k,TK::Ident){return self.pi()}
if matches!(self.tok().k,TK::IntLit){return self.pn()}
if let Ok(v)=self.pbinary_expr(){return Ok(v)}
Err(format!("no alt for Expr at {}",self.tok().s.sl))
}
pub fn pbinary_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
let a1=self.pi()?;
let a2=self.pexpr()?;
Ok(AN::BinaryExpr(Box::new(ABinaryExpr {s:Span::d(),expr:Box::new(a0),operator:Box::new(a1),expr_1:Box::new(a2),})))
}
pub fn tok(&self)->&Tok{&self.t[self.p]}
pub fn adv(&mut self){self.p+=1;}
pub fn e(&mut self,k:TK)->Result<(),String>{if self.tok().k==k{self.adv();Ok(())}else{Err(format!("expected {:?} at {}",k,self.tok().s.sl))}}
pub fn pi(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::Ident{return Err("id".into());}self.adv();Ok(AN::Ident(Box::new(AIdent{s:t.s,v:t.v})))}
pub fn pn(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::IntLit{return Err("int".into());}self.adv();let n:i64=t.v.parse().map_err(|_|"bad")?;Ok(AN::Int(Box::new(AInt{s:t.s,v:n})))}
}

// ── AST Visitor ──

pub trait AstVisit<T>{
fn visit_program(&mut self,n:&AProgram)->T;
fn visit_stmt(&mut self,n:&AStmt)->T;
fn visit_fn_decl(&mut self,n:&AFnDecl)->T;
fn visit_return_stmt(&mut self,n:&AReturnStmt)->T;
fn visit_expr(&mut self,n:&AExpr)->T;
fn visit_binary_expr(&mut self,n:&ABinaryExpr)->T;
}

pub struct AstWalk;
impl<T:Default> AstVisit<T> for AstWalk{
fn visit_program(&mut self,_n:&AProgram)->T{T::default()}
fn visit_stmt(&mut self,_n:&AStmt)->T{T::default()}
fn visit_fn_decl(&mut self,_n:&AFnDecl)->T{T::default()}
fn visit_return_stmt(&mut self,_n:&AReturnStmt)->T{T::default()}
fn visit_expr(&mut self,_n:&AExpr)->T{T::default()}
fn visit_binary_expr(&mut self,_n:&ABinaryExpr)->T{T::default()}
}

// ── Lowering (Tree Transducer) ──

pub fn lower_node(ast:&AN)->Result<HN,String>{
match ast{
AN::Ident(a)=>Ok(HN::Ident(Box::new(HIdent{s:a.s}))),
AN::Int(a)=>Ok(HN::Int(Box::new(HInt{s:a.s,v:a.v}))),
AN::ReturnStmt(a)=>{
return Ok(HN::HirReturn(Box::new(HHirReturn{s:a.s,return_stmt:Box::new(lower_node(&a.expr)?)})));
}
AN::Expr(_)=>Err("skip".into()),
AN::BinaryExpr(a)=>{
return Ok(HN::HirAdd(Box::new(HHirAdd{s:a.s,binary_expr:Box::new(lower_node(&a.expr)?)})));
}
AN::Stmt(_)=>Err("skip".into()),
AN::Program(_)=>Err("skip".into()),
AN::FnDecl(a)=>{
return Ok(HN::HirFnDecl(Box::new(HHirFnDecl{s:a.s,fn_decl:Box::new(lower_node(&a.ident)?)})));
}
_=>Err("unknown node".into())
}
}


