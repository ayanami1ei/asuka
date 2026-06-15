// @generated
#[allow(non_camel_case_types,unused)]

#[derive(Clone,Copy,Debug)]
pub struct Span{pub sl:u32,pub sc:u32,pub el:u32,pub ec:u32}
impl Span{pub fn d()->Self{Self{sl:0,sc:0,el:0,ec:0}}pub fn mg(&self,o:&Span)->Self{Self{sl:self.sl,sc:self.sc,el:o.el,ec:o.ec}}}

#[derive(Clone,PartialEq,Debug)]
pub enum TK{Ident,IntLit,StrLit,KINT,KCHAR,KVOID,If,El,Wh,Fr,Ret,Br,Co,St,KSIZEOF,KTYPEDEF,KCONST,KSTATIC,Ex,KUNSIGNED,KLONG,KSHORT,En,KSWITCH,KCASE,KDEFAULT,KDO,Lt,LP,Ne,Bg,Le,O_2626,Plus,Ar,Eq,Ge,Dt,Star,Gt,And,RP,RBK,Pct,RB,S,Minus,Q,Colon,C,LBK,EqEq,O_7e,O_7c7c,Slash,LB,EOF}

#[derive(Clone,Debug)]
pub struct Tok{pub k:TK,pub s:Span,pub v:String}

pub struct Lex{pub c:Vec<char>,pub p:usize,pub l:u32,pub col:u32}

impl Lex{pub fn new(s:&str)->Self{Self{c:s.chars().collect(),p:0,l:1,col:1}}
pub fn tkz(&mut self)->Vec<Tok>{let mut t=Vec::new();loop{self.skip();if self.p>=self.c.len(){break}match self.c[self.p]{
'"'=>t.push(self.rs()),
c if c.is_ascii_digit()=>t.push(self.rn()),
c if c.is_alphabetic()||c=='_'=>t.push(self.ri()),
'!'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf("!=",TK::Ne))}
else{t.push(self.rf("!",TK::Bg))}
}
'%'=>t.push(self.rf("%",TK::Pct)),
'&'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='&'{t.push(self.rf("&&",TK::O_2626))}
else{t.push(self.rf("&",TK::And))}
}
'('=>t.push(self.rf("(",TK::LP)),
')'=>t.push(self.rf(")",TK::RP)),
'*'=>t.push(self.rf("*",TK::Star)),
'+'=>t.push(self.rf("+",TK::Plus)),
','=>t.push(self.rf(",",TK::C)),
'-'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='>'{t.push(self.rf("->",TK::Ar))}
else{t.push(self.rf("-",TK::Minus))}
}
'.'=>t.push(self.rf(".",TK::Dt)),
'/'=>t.push(self.rf("/",TK::Slash)),
':'=>t.push(self.rf(":",TK::Colon)),
';'=>t.push(self.rf(";",TK::S)),
'<'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf("<=",TK::Le))}
else{t.push(self.rf("<",TK::Lt))}
}
'='=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf("==",TK::EqEq))}
else{t.push(self.rf("=",TK::Eq))}
}
'>'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf(">=",TK::Ge))}
else{t.push(self.rf(">",TK::Gt))}
}
'?'=>t.push(self.rf("?",TK::Q)),
'['=>t.push(self.rf("[",TK::LBK)),
']'=>t.push(self.rf("]",TK::RBK)),
'{'=>t.push(self.rf("{",TK::LB)),
'|'=>t.push(self.rf("||",TK::O_7c7c)),
'}'=>t.push(self.rf("}",TK::RB)),
'~'=>t.push(self.rf("~",TK::O_7e)),
_=>panic!("bad '{}'",self.c[self.p])}}
t.push(Tok{k:TK::EOF,s:Span::d(),v:String::new()});t}
fn skip(&mut self){loop{let c=self.c.get(self.p);match c{Some(' '|'\t'|'\r')=>{self.p+=1;self.col+=1;}Some('\n')=>{self.p+=1;self.l+=1;self.col=1;}Some('/')if self.p+1<self.c.len()&&self.c[self.p+1]=='/'=>{while self.p<self.c.len(){if self.c[self.p]=='\n'{break}self.p+=1;}}_=>{break}}}}
fn rs(&mut self)->Tok{let(sl,sc)=(self.l,self.col);self.p+=1;let mut v=String::new();while self.p<self.c.len()&&self.c[self.p]!='"'{if self.c[self.p]=='\\'{self.p+=1;self.col+=1;match self.c.get(self.p){Some('n')=>v.push('\n'),Some('t')=>v.push('\t'),Some('0')=>v.push('\0'),Some(c)=>v.push(*c),None=>{}}}else{v.push(self.c[self.p]);}self.p+=1;self.col+=1;}if self.p<self.c.len(){self.p+=1;}Tok{k:TK::StrLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn rn(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_ascii_digit()||self.c[self.p]=='.'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();Tok{k:TK::IntLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn ri(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_alphanumeric()||self.c[self.p]=='_'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();let k=match v.as_str(){"int"=>TK::KINT,"char"=>TK::KCHAR,"void"=>TK::KVOID,"if"=>TK::If,"else"=>TK::El,"while"=>TK::Wh,"for"=>TK::Fr,"return"=>TK::Ret,"break"=>TK::Br,"continue"=>TK::Co,"struct"=>TK::St,"sizeof"=>TK::KSIZEOF,"typedef"=>TK::KTYPEDEF,"const"=>TK::KCONST,"static"=>TK::KSTATIC,"extern"=>TK::Ex,"unsigned"=>TK::KUNSIGNED,"long"=>TK::KLONG,"short"=>TK::KSHORT,"enum"=>TK::En,"switch"=>TK::KSWITCH,"case"=>TK::KCASE,"default"=>TK::KDEFAULT,"do"=>TK::KDO,_=>TK::Ident};Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn rf(&mut self,s:&str,k:TK)->Tok{let(sl,sc)=(self.l,self.col);self.p+=s.len();self.col+=s.len()as u32;Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v:s.to_string()}}
}

// AST
#[derive(Clone,Debug)]
pub struct AIdent{pub s:Span,pub v:String}
#[derive(Clone,Debug)]
pub struct AInt{pub s:Span,pub v:i64}
#[derive(Clone,Debug)]
pub struct AStringLiteral{pub s:Span,pub v:String}

#[derive(Clone,Debug)]
pub enum AN{Ident(Box<AIdent>),Int(Box<AInt>),StringLiteral(Box<AStringLiteral>),Program(Box<AProgram>),Stmt(Box<AStmt>),VarDecl(Box<AVarDecl>),FnDecl(Box<AFnDecl>),ParamList(Box<AParamList>),Param(Box<AParam>),Block(Box<ABlock>),IfStmt(Box<AIfStmt>),WhileStmt(Box<AWhileStmt>),ForStmt(Box<AForStmt>),ReturnStmt(Box<AReturnStmt>),BreakStmt(Box<ABreakStmt>),ContinueStmt(Box<AContinueStmt>),ExprStmt(Box<AExprStmt>),Type(Box<AType>),TypeSpec(Box<ATypeSpec>),Expr(Box<AExpr>),CallExpr(Box<ACallExpr>),ExprList(Box<AExprList>),PrimaryExpr(Box<APrimaryExpr>),SizeofExpr(Box<ASizeofExpr>),}

#[derive(Clone,Debug)]
pub struct AProgram{pub s:Span,pub stmt:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AVarDecl{pub s:Span,pub typ:Box<AN>,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFnDecl{pub s:Span,pub typ:Box<AN>,pub ident:Box<AN>,pub param_list:Option<Box<AN>>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AParamList{pub s:Span,pub param:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AParam{pub s:Span,pub typ:Box<AN>,pub ident:Option<Box<AN>>,}

#[derive(Clone,Debug)]
pub struct ABlock{pub s:Span,pub stmt:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AIfStmt{pub s:Span,pub expr:Box<AN>,pub stmt:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AWhileStmt{pub s:Span,pub expr:Box<AN>,pub stmt:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AForStmt{pub s:Span,pub expr:Option<Box<AN>>,pub expr_1:Option<Box<AN>>,pub expr_2:Option<Box<AN>>,pub stmt:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AReturnStmt{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ABreakStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AContinueStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AExprStmt{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AType{pub s:Span,pub type_spec:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ATypeSpec{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AExpr{pub s:Span,}

#[derive(Clone,Debug)]
pub struct ACallExpr{pub s:Span,pub ident:Box<AN>,pub expr_list:Option<Box<AN>>,}

#[derive(Clone,Debug)]
pub struct AExprList{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct APrimaryExpr{pub s:Span,}

#[derive(Clone,Debug)]
pub struct ASizeofExpr{pub s:Span,pub typ:Box<AN>,}

// HIR
#[derive(Clone,Debug)]
pub struct HIdent{pub s:Span}
#[derive(Clone,Debug)]
pub struct HInt{pub s:Span,pub v:i64}
#[derive(Clone,Debug)]
pub struct HStringLiteral{pub s:Span,pub v:String}

#[derive(Clone,Debug)]
pub enum HN{Ident(Box<HIdent>),Int(Box<HInt>),StringLiteral(Box<HStringLiteral>),HirInt(Box<HHirInt>),HirIdent(Box<HHirIdent>),}

#[derive(Clone,Debug)]
pub struct HHirInt{pub s:Span,pub int_literal:Box<HN>,}

#[derive(Clone,Debug)]
pub struct HHirIdent{pub s:Span,pub ident:Box<HN>,}

// Parser
pub struct P{pub t:Vec<Tok>,pub p:usize}
impl P{pub fn new(t:Vec<Tok>)->Self{Self{t,p:0}}
pub fn pprogram(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let mut stmts:Vec<AN>=Vec::new();
while let Ok(v)=self.pstmt(){stmts.push(v)}
Ok(AN::Program(Box::new(AProgram {s:Span::d(),stmt:stmts,})))
}
pub fn pstmt(&mut self)->Result<AN,String>{
if let Ok(v)=self.pvar_decl(){return Ok(v)}
if let Ok(v)=self.pfn_decl(){return Ok(v)}
if let Ok(v)=self.pif_stmt(){return Ok(v)}
if let Ok(v)=self.pwhile_stmt(){return Ok(v)}
if let Ok(v)=self.pfor_stmt(){return Ok(v)}
if let Ok(v)=self.preturn_stmt(){return Ok(v)}
if let Ok(v)=self.pbreak_stmt(){return Ok(v)}
if let Ok(v)=self.pcontinue_stmt(){return Ok(v)}
if let Ok(v)=self.pblock(){return Ok(v)}
if let Ok(v)=self.pexpr_stmt(){return Ok(v)}
Err(format!("no alt for Stmt at {}",self.tok().s.sl))
}
pub fn pvar_decl(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
let a1=self.pi()?;
self.e(TK::S)?;
Ok(AN::VarDecl(Box::new(AVarDecl {s:Span::d(),typ:Box::new(a0),ident:Box::new(a1),})))
}
pub fn pfn_decl(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
let a1=self.pi()?;
self.e(TK::LP)?;
let a2 = self.pparam_list().ok();
self.e(TK::RP)?;
let a3=self.pblock()?;
Ok(AN::FnDecl(Box::new(AFnDecl {s:Span::d(),typ:Box::new(a0),ident:Box::new(a1),param_list:a2.map(|v|Box::new(v)),block:Box::new(a3),})))
}
pub fn pparam_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pparam()?;
Ok(AN::ParamList(Box::new(AParamList {s:Span::d(),param:Box::new(a0),})))
}
pub fn pparam(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
let a1 = self.pi().ok();
Ok(AN::Param(Box::new(AParam {s:Span::d(),typ:Box::new(a0),ident:a1.map(|v|Box::new(v)),})))
}
pub fn pblock(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::LB)?;
let mut stmts:Vec<AN>=Vec::new();
while let Ok(v)=self.pstmt(){stmts.push(v)}
self.e(TK::RB)?;
Ok(AN::Block(Box::new(ABlock {s:Span::d(),stmt:stmts,})))
}
pub fn pif_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::If)?;
self.e(TK::LP)?;
let a0=self.pexpr()?;
self.e(TK::RP)?;
let a1=self.pstmt()?;
Ok(AN::IfStmt(Box::new(AIfStmt {s:Span::d(),expr:Box::new(a0),stmt:Box::new(a1),})))
}
pub fn pwhile_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Wh)?;
self.e(TK::LP)?;
let a0=self.pexpr()?;
self.e(TK::RP)?;
let a1=self.pstmt()?;
Ok(AN::WhileStmt(Box::new(AWhileStmt {s:Span::d(),expr:Box::new(a0),stmt:Box::new(a1),})))
}
pub fn pfor_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Fr)?;
self.e(TK::LP)?;
let a0 = self.pexpr().ok();
self.e(TK::S)?;
let a1 = self.pexpr().ok();
self.e(TK::S)?;
let a2 = self.pexpr().ok();
self.e(TK::RP)?;
let a3=self.pstmt()?;
Ok(AN::ForStmt(Box::new(AForStmt {s:Span::d(),expr:a0.map(|v|Box::new(v)),expr_1:a1.map(|v|Box::new(v)),expr_2:a2.map(|v|Box::new(v)),stmt:Box::new(a3),})))
}
pub fn preturn_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Ret)?;
let a0=self.pexpr()?;
self.e(TK::S)?;
Ok(AN::ReturnStmt(Box::new(AReturnStmt {s:Span::d(),expr:Box::new(a0),})))
}
pub fn pbreak_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Br)?;
self.e(TK::S)?;
Ok(AN::BreakStmt(Box::new(ABreakStmt {s:Span::d(),})))
}
pub fn pcontinue_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Co)?;
self.e(TK::S)?;
Ok(AN::ContinueStmt(Box::new(AContinueStmt {s:Span::d(),})))
}
pub fn pexpr_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
self.e(TK::S)?;
Ok(AN::ExprStmt(Box::new(AExprStmt {s:Span::d(),expr:Box::new(a0),})))
}
pub fn ptyp(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptype_spec()?;
Ok(AN::Type(Box::new(AType {s:Span::d(),type_spec:Box::new(a0),})))
}
pub fn ptype_spec(&mut self)->Result<AN,String>{
if matches!(self.tok().k,TK::KINT){
let _s=self.tok().s;
self.e(TK::KINT)?;
return Ok(AN::TypeSpec(Box::new(ATypeSpec{s:Span::d()})));
}
if matches!(self.tok().k,TK::KCHAR){
let _s=self.tok().s;
self.e(TK::KCHAR)?;
return Ok(AN::TypeSpec(Box::new(ATypeSpec{s:Span::d()})));
}
if matches!(self.tok().k,TK::KVOID){
let _s=self.tok().s;
self.e(TK::KVOID)?;
return Ok(AN::TypeSpec(Box::new(ATypeSpec{s:Span::d()})));
}
if matches!(self.tok().k,TK::St){
let _s=self.tok().s;
self.e(TK::St)?;
let a0=self.pi()?;
return Ok(AN::TypeSpec(Box::new(ATypeSpec{s:Span::d()})));
}
Err(format!("no alt for TypeSpec at {}",self.tok().s.sl))
}
pub fn pexpr(&mut self)->Result<AN,String>{
if matches!(self.tok().k,TK::Ident){return self.pi()}
if matches!(self.tok().k,TK::IntLit){return self.pn()}
if let Ok(v)=self.pcall_expr(){return Ok(v)}
Err(format!("no alt for Expr at {}",self.tok().s.sl))
}
pub fn pcall_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
self.e(TK::LP)?;
let a1 = self.pexpr_list().ok();
self.e(TK::RP)?;
Ok(AN::CallExpr(Box::new(ACallExpr {s:Span::d(),ident:Box::new(a0),expr_list:a1.map(|v|Box::new(v)),})))
}
pub fn pexpr_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
Ok(AN::ExprList(Box::new(AExprList {s:Span::d(),expr:Box::new(a0),})))
}
pub fn pprimary_expr(&mut self)->Result<AN,String>{
if matches!(self.tok().k,TK::Ident){return self.pi()}
if matches!(self.tok().k,TK::IntLit){return self.pn()}
Err(format!("no alt for PrimaryExpr at {}",self.tok().s.sl))
}
pub fn psizeof_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::KSIZEOF)?;
self.e(TK::LP)?;
let a0=self.ptyp()?;
self.e(TK::RP)?;
Ok(AN::SizeofExpr(Box::new(ASizeofExpr {s:Span::d(),typ:Box::new(a0),})))
}
pub fn tok(&self)->&Tok{&self.t[self.p]}
pub fn adv(&mut self){self.p+=1;}
pub fn e(&mut self,k:TK)->Result<(),String>{if self.tok().k==k{self.adv();Ok(())}else{Err(format!("expected {:?} at {}",k,self.tok().s.sl))}}
pub fn pi(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::Ident{return Err("id".into());}self.adv();Ok(AN::Ident(Box::new(AIdent{s:t.s,v:t.v})))}
pub fn pn(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::IntLit{return Err("int".into());}self.adv();let n:i64=t.v.parse().map_err(|_|"bad")?;Ok(AN::Int(Box::new(AInt{s:t.s,v:n})))}
pub fn ps(&mut self)->Result<AN,String>{let t=self.tok().clone();if t.k!=TK::StrLit{return Err("str".into());}self.adv();Ok(AN::StringLiteral(Box::new(AStringLiteral{s:t.s,v:t.v})))}
}

// ── AST Visitor ──

pub trait AstVisit<T>{
fn visit_program(&mut self,n:&AProgram)->T;
fn visit_stmt(&mut self,n:&AStmt)->T;
fn visit_var_decl(&mut self,n:&AVarDecl)->T;
fn visit_fn_decl(&mut self,n:&AFnDecl)->T;
fn visit_param_list(&mut self,n:&AParamList)->T;
fn visit_param(&mut self,n:&AParam)->T;
fn visit_block(&mut self,n:&ABlock)->T;
fn visit_if_stmt(&mut self,n:&AIfStmt)->T;
fn visit_while_stmt(&mut self,n:&AWhileStmt)->T;
fn visit_for_stmt(&mut self,n:&AForStmt)->T;
fn visit_return_stmt(&mut self,n:&AReturnStmt)->T;
fn visit_break_stmt(&mut self,n:&ABreakStmt)->T;
fn visit_continue_stmt(&mut self,n:&AContinueStmt)->T;
fn visit_expr_stmt(&mut self,n:&AExprStmt)->T;
fn visit_typ(&mut self,n:&AType)->T;
fn visit_type_spec(&mut self,n:&ATypeSpec)->T;
fn visit_expr(&mut self,n:&AExpr)->T;
fn visit_call_expr(&mut self,n:&ACallExpr)->T;
fn visit_expr_list(&mut self,n:&AExprList)->T;
fn visit_primary_expr(&mut self,n:&APrimaryExpr)->T;
fn visit_sizeof_expr(&mut self,n:&ASizeofExpr)->T;
}

pub struct AstWalk;
impl<T:Default> AstVisit<T> for AstWalk{
fn visit_program(&mut self,_n:&AProgram)->T{T::default()}
fn visit_stmt(&mut self,_n:&AStmt)->T{T::default()}
fn visit_var_decl(&mut self,_n:&AVarDecl)->T{T::default()}
fn visit_fn_decl(&mut self,_n:&AFnDecl)->T{T::default()}
fn visit_param_list(&mut self,_n:&AParamList)->T{T::default()}
fn visit_param(&mut self,_n:&AParam)->T{T::default()}
fn visit_block(&mut self,_n:&ABlock)->T{T::default()}
fn visit_if_stmt(&mut self,_n:&AIfStmt)->T{T::default()}
fn visit_while_stmt(&mut self,_n:&AWhileStmt)->T{T::default()}
fn visit_for_stmt(&mut self,_n:&AForStmt)->T{T::default()}
fn visit_return_stmt(&mut self,_n:&AReturnStmt)->T{T::default()}
fn visit_break_stmt(&mut self,_n:&ABreakStmt)->T{T::default()}
fn visit_continue_stmt(&mut self,_n:&AContinueStmt)->T{T::default()}
fn visit_expr_stmt(&mut self,_n:&AExprStmt)->T{T::default()}
fn visit_typ(&mut self,_n:&AType)->T{T::default()}
fn visit_type_spec(&mut self,_n:&ATypeSpec)->T{T::default()}
fn visit_expr(&mut self,_n:&AExpr)->T{T::default()}
fn visit_call_expr(&mut self,_n:&ACallExpr)->T{T::default()}
fn visit_expr_list(&mut self,_n:&AExprList)->T{T::default()}
fn visit_primary_expr(&mut self,_n:&APrimaryExpr)->T{T::default()}
fn visit_sizeof_expr(&mut self,_n:&ASizeofExpr)->T{T::default()}
}

// ── Lowering (Tree Transducer) ──

pub fn lower_node(ast:&AN)->Result<HN,String>{
match ast{
AN::Ident(a)=>Ok(HN::Ident(Box::new(HIdent{s:a.s}))),
AN::Int(a)=>Ok(HN::Int(Box::new(HInt{s:a.s,v:a.v}))),
AN::StringLiteral(a)=>Ok(HN::StringLiteral(Box::new(HStringLiteral{s:a.s,v:a.v.clone()}))),
AN::TypeSpec(_)=>Err("skip".into()),
AN::CallExpr(_)=>Err("skip".into()),
AN::Block(_)=>Err("skip".into()),
AN::VarDecl(_)=>Err("skip".into()),
AN::IfStmt(_)=>Err("skip".into()),
AN::Stmt(_)=>Err("skip".into()),
AN::ReturnStmt(_)=>Err("skip".into()),
AN::ParamList(_)=>Err("skip".into()),
AN::Program(_)=>Err("skip".into()),
AN::Param(_)=>Err("skip".into()),
AN::FnDecl(_)=>Err("skip".into()),
AN::Expr(_)=>Err("skip".into()),
AN::WhileStmt(_)=>Err("skip".into()),
AN::ExprStmt(_)=>Err("skip".into()),
AN::BreakStmt(_)=>Err("skip".into()),
AN::ContinueStmt(_)=>Err("skip".into()),
AN::ForStmt(_)=>Err("skip".into()),
AN::ExprList(_)=>Err("skip".into()),
AN::PrimaryExpr(_)=>Err("skip".into()),
AN::SizeofExpr(_)=>Err("skip".into()),
AN::Type(_)=>Err("skip".into()),
_=>Err("unknown node".into())
}
}

// ── Emit ──

pub fn emit_node(n:&HN)->Result<String,String>{
match n{
HN::Ident(_)=>Ok(String::new()),
HN::Int(a)=>Ok(format!("{}",a.v)),
HN::StringLiteral(a)=>Ok(a.v.clone()),
HN::HirInt(a)=>{
Ok(format!("%hir_int = add i64 0, %v"))
}
HN::HirIdent(a)=>{
Ok(format!("%n"))
}
_=>Err("no emit".into())
}
}


