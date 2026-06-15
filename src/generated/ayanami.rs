// @generated
#[allow(non_camel_case_types,unused)]

#[derive(Clone,Copy,Debug)]
pub struct Span{pub sl:u32,pub sc:u32,pub el:u32,pub ec:u32}
impl Span{pub fn d()->Self{Self{sl:0,sc:0,el:0,ec:0}}pub fn mg(&self,o:&Span)->Self{Self{sl:self.sl,sc:self.sc,el:o.el,ec:o.ec}}}

#[derive(Clone,PartialEq,Debug)]
pub enum TK{Ident,IntLit,StrLit,Fn,Let,Ret,If,El,T,F,Wh,Fr,In,Mt,En,St,KINTERFACE,Im,Pb,Sh,Uq,Wk,Ex,Ip,As,Br,Co,Mut,Slf,Gt,Colon,Dt,Minus,Slash,Eq,FA,Ne,S,Us,RB,RP,Plus,Lt,EqEq,Le,Ge,LB,Star,CC,Ar,RBK,C,LP,LBK,Pct,EOF}

#[derive(Clone,Debug)]
pub struct Tok{pub k:TK,pub s:Span,pub v:String}

pub struct Lex{pub c:Vec<char>,pub p:usize,pub l:u32,pub col:u32}

impl Lex{pub fn new(s:&str)->Self{Self{c:s.chars().collect(),p:0,l:1,col:1}}
pub fn tkz(&mut self)->Vec<Tok>{let mut t=Vec::new();loop{self.skip();if self.p>=self.c.len(){break}match self.c[self.p]{
'"'=>t.push(self.rs()),
c if c.is_ascii_digit()=>t.push(self.rn()),
c if c.is_alphabetic()||c=='_'=>t.push(self.ri()),
'!'=>t.push(self.rf("!=",TK::Ne)),
'%'=>t.push(self.rf("%",TK::Pct)),
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
':'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]==':'{t.push(self.rf("::",TK::CC))}
else{t.push(self.rf(":",TK::Colon))}
}
';'=>t.push(self.rf(";",TK::S)),
'<'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf("<=",TK::Le))}
else{t.push(self.rf("<",TK::Lt))}
}
'='=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf("==",TK::EqEq))}
if self.p+1<self.c.len()&&self.c[self.p+1]=='>'{t.push(self.rf("=>",TK::FA))}
else{t.push(self.rf("=",TK::Eq))}
}
'>'=>{
if self.p+1<self.c.len()&&self.c[self.p+1]=='='{t.push(self.rf(">=",TK::Ge))}
else{t.push(self.rf(">",TK::Gt))}
}
'['=>t.push(self.rf("[",TK::LBK)),
']'=>t.push(self.rf("]",TK::RBK)),
'{'=>t.push(self.rf("{",TK::LB)),
'}'=>t.push(self.rf("}",TK::RB)),
_=>panic!("bad '{}'",self.c[self.p])}}
t.push(Tok{k:TK::EOF,s:Span::d(),v:String::new()});t}
fn skip(&mut self){loop{let c=self.c.get(self.p);match c{Some(' '|'\t'|'\r')=>{self.p+=1;self.col+=1;}Some('\n')=>{self.p+=1;self.l+=1;self.col=1;}Some('/')if self.p+1<self.c.len()&&self.c[self.p+1]=='/'=>{while self.p<self.c.len(){if self.c[self.p]=='\n'{break}self.p+=1;}}_=>{break}}}}
fn rs(&mut self)->Tok{let(sl,sc)=(self.l,self.col);self.p+=1;let mut v=String::new();while self.p<self.c.len()&&self.c[self.p]!='"'{if self.c[self.p]=='\\'{self.p+=1;self.col+=1;match self.c.get(self.p){Some('n')=>v.push('\n'),Some('t')=>v.push('\t'),Some('0')=>v.push('\0'),Some(c)=>v.push(*c),None=>{}}}else{v.push(self.c[self.p]);}self.p+=1;self.col+=1;}if self.p<self.c.len(){self.p+=1;}Tok{k:TK::StrLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn rn(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_ascii_digit()||self.c[self.p]=='.'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();Tok{k:TK::IntLit,s:Span{sl,sc,el:self.l,ec:self.col},v}}
fn ri(&mut self)->Tok{let(sl,sc)=(self.l,self.col);let sp=self.p;while self.p<self.c.len()&&(self.c[self.p].is_alphanumeric()||self.c[self.p]=='_'){self.p+=1;self.col+=1;}let v:String=self.c[sp..self.p].iter().collect();let k=match v.as_str(){"fn"=>TK::Fn,"let"=>TK::Let,"return"=>TK::Ret,"if"=>TK::If,"else"=>TK::El,"true"=>TK::T,"false"=>TK::F,"while"=>TK::Wh,"for"=>TK::Fr,"in"=>TK::In,"match"=>TK::Mt,"enum"=>TK::En,"struct"=>TK::St,"interface"=>TK::KINTERFACE,"impl"=>TK::Im,"pub"=>TK::Pb,"shared"=>TK::Sh,"unique"=>TK::Uq,"weak"=>TK::Wk,"extern"=>TK::Ex,"import"=>TK::Ip,"as"=>TK::As,"break"=>TK::Br,"continue"=>TK::Co,"mut"=>TK::Mut,"self"=>TK::Slf,_=>TK::Ident};Tok{k,s:Span{sl,sc,el:self.l,ec:self.col},v}}
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
pub enum AN{Ident(Box<AIdent>),Int(Box<AInt>),StringLiteral(Box<AStringLiteral>),Program(Box<AProgram>),Item(Box<AItem>),Import(Box<AImport>),FnDecl(Box<AFnDecl>),FnParam(Box<AFnParam>),ParamList(Box<AParamList>),Vis(Box<AVis>),StructDef(Box<AStructDef>),Field(Box<AField>),FieldList(Box<AFieldList>),EnumDef(Box<AEnumDef>),Variant(Box<AVariant>),VariantList(Box<AVariantList>),InterfaceDef(Box<AInterfaceDef>),IfaceMethod(Box<AIfaceMethod>),SelfParam(Box<ASelfParam>),ImplBlock(Box<AImplBlock>),Method(Box<AMethod>),GenericParams(Box<AGenericParams>),GenericParam(Box<AGenericParam>),Type(Box<AType>),TypeBase(Box<ATypeBase>),SharedType(Box<ASharedType>),UniqueType(Box<AUniqueType>),WeakType(Box<AWeakType>),FnType(Box<AFnType>),ArrayType(Box<AArrayType>),TypeList(Box<ATypeList>),Block(Box<ABlock>),Stmt(Box<AStmt>),VarDecl(Box<AVarDecl>),ReturnStmt(Box<AReturnStmt>),IfStmt(Box<AIfStmt>),WhileStmt(Box<AWhileStmt>),ForStmt(Box<AForStmt>),MatchStmt(Box<AMatchStmt>),MatchArm(Box<AMatchArm>),BreakStmt(Box<ABreakStmt>),ContinueStmt(Box<AContinueStmt>),ExprStmt(Box<AExprStmt>),Pattern(Box<APattern>),PatternArgs(Box<APatternArgs>),Expr(Box<AExpr>),BinaryExpr(Box<ABinaryExpr>),UnaryExpr(Box<AUnaryExpr>),CallExpr(Box<ACallExpr>),FieldExpr(Box<AFieldExpr>),IndexExpr(Box<AIndexExpr>),MatchExpr(Box<AMatchExpr>),IfExpr(Box<AIfExpr>),BlockExpr(Box<ABlockExpr>),StructLiteral(Box<AStructLiteral>),FieldInit(Box<AFieldInit>),FieldInitList(Box<AFieldInitList>),ArrayLiteral(Box<AArrayLiteral>),ExprList(Box<AExprList>),BoolLiteral(Box<ABoolLiteral>),}

#[derive(Clone,Debug)]
pub struct AProgram{pub s:Span,pub item:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AItem{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AImport{pub s:Span,pub string_literal:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFnDecl{pub s:Span,pub vis:Option<Box<AN>>,pub ident:Box<AN>,pub generic_params:Option<Box<AN>>,pub param_list:Box<AN>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFnParam{pub s:Span,pub typ:Box<AN>,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AParamList{pub s:Span,pub fn_param:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AVis{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AStructDef{pub s:Span,pub vis:Option<Box<AN>>,pub ident:Box<AN>,pub generic_params:Option<Box<AN>>,pub field_list:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AField{pub s:Span,pub typ:Box<AN>,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFieldList{pub s:Span,pub field:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AEnumDef{pub s:Span,pub vis:Option<Box<AN>>,pub ident:Box<AN>,pub generic_params:Option<Box<AN>>,pub variant_list:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AVariant{pub s:Span,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AVariantList{pub s:Span,pub variant:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AInterfaceDef{pub s:Span,pub vis:Option<Box<AN>>,pub ident:Box<AN>,pub generic_params:Option<Box<AN>>,pub iface_method_list:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AIfaceMethod{pub s:Span,pub ident:Box<AN>,pub self_param:Box<AN>,pub param_list:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ASelfParam{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AImplBlock{pub s:Span,pub vis:Option<Box<AN>>,pub generic_params:Option<Box<AN>>,pub typ:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AMethod{pub s:Span,pub vis:Option<Box<AN>>,pub ident:Box<AN>,pub generic_params:Option<Box<AN>>,pub self_param:Box<AN>,pub param_list:Box<AN>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AGenericParams{pub s:Span,pub generic_param:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AGenericParam{pub s:Span,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AType{pub s:Span,}

#[derive(Clone,Debug)]
pub struct ATypeBase{pub s:Span,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ASharedType{pub s:Span,pub typ:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AUniqueType{pub s:Span,pub typ:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AWeakType{pub s:Span,pub typ:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFnType{pub s:Span,pub type_list:Option<Box<AN>>,}

#[derive(Clone,Debug)]
pub struct AArrayType{pub s:Span,pub typ:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ATypeList{pub s:Span,pub typ:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ABlock{pub s:Span,pub stmt:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AVarDecl{pub s:Span,pub ident:Box<AN>,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AReturnStmt{pub s:Span,pub expr:Option<Box<AN>>,}

#[derive(Clone,Debug)]
pub struct AIfStmt{pub s:Span,pub expr:Box<AN>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AWhileStmt{pub s:Span,pub expr:Box<AN>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AForStmt{pub s:Span,pub ident:Box<AN>,pub expr:Box<AN>,pub expr_1:Box<AN>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AMatchStmt{pub s:Span,pub expr:Box<AN>,pub match_arm:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AMatchArm{pub s:Span,pub pattern:Box<AN>,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ABreakStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AContinueStmt{pub s:Span,}

#[derive(Clone,Debug)]
pub struct AExprStmt{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct APattern{pub s:Span,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct APatternArgs{pub s:Span,pub pattern:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AExpr{pub s:Span,}

#[derive(Clone,Debug)]
pub struct ABinaryExpr{pub s:Span,pub expr:Box<AN>,pub operator:Box<AN>,pub expr_1:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AUnaryExpr{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ACallExpr{pub s:Span,pub ident:Box<AN>,pub expr_list:Option<Box<AN>>,}

#[derive(Clone,Debug)]
pub struct AFieldExpr{pub s:Span,pub expr:Box<AN>,pub ident:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AIndexExpr{pub s:Span,pub expr:Box<AN>,pub expr_1:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AMatchExpr{pub s:Span,pub expr:Box<AN>,pub match_arm:Vec<AN>,}

#[derive(Clone,Debug)]
pub struct AIfExpr{pub s:Span,pub expr:Box<AN>,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ABlockExpr{pub s:Span,pub block:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AStructLiteral{pub s:Span,pub typ:Box<AN>,pub field_init_list:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFieldInit{pub s:Span,pub ident:Box<AN>,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AFieldInitList{pub s:Span,pub field_init:Box<AN>,}

#[derive(Clone,Debug)]
pub struct AArrayLiteral{pub s:Span,pub expr_list:Option<Box<AN>>,}

#[derive(Clone,Debug)]
pub struct AExprList{pub s:Span,pub expr:Box<AN>,}

#[derive(Clone,Debug)]
pub struct ABoolLiteral{pub s:Span,}

// HIR
#[derive(Clone,Debug)]
pub struct HIdent{pub s:Span}
#[derive(Clone,Debug)]
pub struct HInt{pub s:Span,pub v:i64}
#[derive(Clone,Debug)]
pub struct HStringLiteral{pub s:Span,pub v:String}

#[derive(Clone,Debug)]
pub enum HN{Ident(Box<HIdent>),Int(Box<HInt>),StringLiteral(Box<HStringLiteral>)}

// Parser
pub struct P{pub t:Vec<Tok>,pub p:usize}
impl P{pub fn new(t:Vec<Tok>)->Self{Self{t,p:0}}
fn op_prec(&self,k:&TK)->u32{match k{
TK::Plus => 5,
TK::Minus => 5,
TK::Star => 6,
TK::Slash => 6,
TK::Pct => 6,
TK::EqEq => 3,
TK::Ne => 3,
TK::Lt => 3,
TK::Gt => 3,
TK::Le => 3,
TK::Ge => 3,
TK::Eq => 2,
TK::LBK => 1,
TK::RBK => 1,
_=>0}}
pub fn pprogram(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let mut items:Vec<AN>=Vec::new();
while let Ok(v)=self.pitem(){items.push(v)}
Ok(AN::Program(Box::new(AProgram {s:Span::d(),item:items,})))
}
pub fn pitem(&mut self)->Result<AN,String>{
if let Ok(v)=self.pfn_decl(){return Ok(v)}
if let Ok(v)=self.pstruct_def(){return Ok(v)}
if let Ok(v)=self.penum_def(){return Ok(v)}
if let Ok(v)=self.pinterface_def(){return Ok(v)}
if let Ok(v)=self.pimpl_block(){return Ok(v)}
if let Ok(v)=self.pimport(){return Ok(v)}
Err(format!("no alt for Item at {}",self.tok().s.sl))
}
pub fn pimport(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Ip)?;
let a0=self.ps()?;
self.e(TK::S)?;
Ok(AN::Import(Box::new(AImport {s:Span::d(),string_literal:Box::new(a0),})))
}
pub fn pfn_decl(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0 = self.pvis().ok();
self.e(TK::Fn)?;
let a1=self.pi()?;
let a2 = self.pgeneric_params().ok();
self.e(TK::LP)?;
let a3=self.pparam_list()?;
self.e(TK::RP)?;
let a4=self.pblock()?;
Ok(AN::FnDecl(Box::new(AFnDecl {s:Span::d(),vis:a0.map(|v|Box::new(v)),ident:Box::new(a1),generic_params:a2.map(|v|Box::new(v)),param_list:Box::new(a3),block:Box::new(a4),})))
}
pub fn pfn_param(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
let a1=self.pi()?;
Ok(AN::FnParam(Box::new(AFnParam {s:Span::d(),typ:Box::new(a0),ident:Box::new(a1),})))
}
pub fn pparam_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pfn_param()?;
Ok(AN::ParamList(Box::new(AParamList {s:Span::d(),fn_param:Box::new(a0),})))
}
pub fn pvis(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Pb)?;
Ok(AN::Vis(Box::new(AVis {s:Span::d(),})))
}
pub fn pstruct_def(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0 = self.pvis().ok();
self.e(TK::St)?;
let a1=self.pi()?;
let a2 = self.pgeneric_params().ok();
self.e(TK::LB)?;
let a3=self.pfield_list()?;
self.e(TK::RB)?;
Ok(AN::StructDef(Box::new(AStructDef {s:Span::d(),vis:a0.map(|v|Box::new(v)),ident:Box::new(a1),generic_params:a2.map(|v|Box::new(v)),field_list:Box::new(a3),})))
}
pub fn pfield(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
let a1=self.pi()?;
Ok(AN::Field(Box::new(AField {s:Span::d(),typ:Box::new(a0),ident:Box::new(a1),})))
}
pub fn pfield_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let mut fields:Vec<AN>=Vec::new();
while let Ok(v)=self.pfield(){fields.push(v)}
Ok(AN::FieldList(Box::new(AFieldList {s:Span::d(),field:fields,})))
}
pub fn penum_def(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0 = self.pvis().ok();
self.e(TK::En)?;
let a1=self.pi()?;
let a2 = self.pgeneric_params().ok();
self.e(TK::LB)?;
let a3=self.pvariant_list()?;
self.e(TK::RB)?;
Ok(AN::EnumDef(Box::new(AEnumDef {s:Span::d(),vis:a0.map(|v|Box::new(v)),ident:Box::new(a1),generic_params:a2.map(|v|Box::new(v)),variant_list:Box::new(a3),})))
}
pub fn pvariant(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
Ok(AN::Variant(Box::new(AVariant {s:Span::d(),ident:Box::new(a0),})))
}
pub fn pvariant_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pvariant()?;
Ok(AN::VariantList(Box::new(AVariantList {s:Span::d(),variant:Box::new(a0),})))
}
pub fn pinterface_def(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0 = self.pvis().ok();
self.e(TK::KINTERFACE)?;
let a1=self.pi()?;
let a2 = self.pgeneric_params().ok();
self.e(TK::LB)?;
let a3=self.pi()?;
self.e(TK::RB)?;
Ok(AN::InterfaceDef(Box::new(AInterfaceDef {s:Span::d(),vis:a0.map(|v|Box::new(v)),ident:Box::new(a1),generic_params:a2.map(|v|Box::new(v)),iface_method_list:Box::new(a3),})))
}
pub fn piface_method(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Fn)?;
let a0=self.pi()?;
self.e(TK::LP)?;
let a1=self.pself_param()?;
self.e(TK::C)?;
let a2=self.pparam_list()?;
self.e(TK::RP)?;
self.e(TK::S)?;
Ok(AN::IfaceMethod(Box::new(AIfaceMethod {s:Span::d(),ident:Box::new(a0),self_param:Box::new(a1),param_list:Box::new(a2),})))
}
pub fn pself_param(&mut self)->Result<AN,String>{
let _s=self.tok().s;
//TODO
self.e(TK::Slf)?;
Ok(AN::SelfParam(Box::new(ASelfParam {s:Span::d(),})))
}
pub fn pimpl_block(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0 = self.pvis().ok();
self.e(TK::Im)?;
let a1 = self.pgeneric_params().ok();
let a2=self.ptyp()?;
Ok(AN::ImplBlock(Box::new(AImplBlock {s:Span::d(),vis:a0.map(|v|Box::new(v)),generic_params:a1.map(|v|Box::new(v)),typ:Box::new(a2),})))
}
pub fn pmethod(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0 = self.pvis().ok();
self.e(TK::Fn)?;
let a1=self.pi()?;
let a2 = self.pgeneric_params().ok();
self.e(TK::LP)?;
let a3=self.pself_param()?;
self.e(TK::C)?;
let a4=self.pparam_list()?;
self.e(TK::RP)?;
let a5=self.pblock()?;
Ok(AN::Method(Box::new(AMethod {s:Span::d(),vis:a0.map(|v|Box::new(v)),ident:Box::new(a1),generic_params:a2.map(|v|Box::new(v)),self_param:Box::new(a3),param_list:Box::new(a4),block:Box::new(a5),})))
}
pub fn pgeneric_params(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::LBK)?;
let a0=self.pgeneric_param()?;
self.e(TK::RBK)?;
Ok(AN::GenericParams(Box::new(AGenericParams {s:Span::d(),generic_param:Box::new(a0),})))
}
pub fn pgeneric_param(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
Ok(AN::GenericParam(Box::new(AGenericParam {s:Span::d(),ident:Box::new(a0),})))
}
pub fn ptyp(&mut self)->Result<AN,String>{
if let Ok(v)=self.ptype_base(){return Ok(v)}
if let Ok(v)=self.pshared_type(){return Ok(v)}
if let Ok(v)=self.punique_type(){return Ok(v)}
if let Ok(v)=self.pweak_type(){return Ok(v)}
if let Ok(v)=self.pfn_type(){return Ok(v)}
if let Ok(v)=self.parray_type(){return Ok(v)}
Err(format!("no alt for Type at {}",self.tok().s.sl))
}
pub fn ptype_base(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
Ok(AN::TypeBase(Box::new(ATypeBase {s:Span::d(),ident:Box::new(a0),})))
}
pub fn pshared_type(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Sh)?;
let a0=self.ptyp()?;
Ok(AN::SharedType(Box::new(ASharedType {s:Span::d(),typ:Box::new(a0),})))
}
pub fn punique_type(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Uq)?;
let a0=self.ptyp()?;
Ok(AN::UniqueType(Box::new(AUniqueType {s:Span::d(),typ:Box::new(a0),})))
}
pub fn pweak_type(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Wk)?;
let a0=self.ptyp()?;
Ok(AN::WeakType(Box::new(AWeakType {s:Span::d(),typ:Box::new(a0),})))
}
pub fn pfn_type(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Fn)?;
self.e(TK::LP)?;
let a0 = self.ptype_list().ok();
self.e(TK::RP)?;
Ok(AN::FnType(Box::new(AFnType {s:Span::d(),type_list:a0.map(|v|Box::new(v)),})))
}
pub fn parray_type(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::LBK)?;
let a0=self.ptyp()?;
self.e(TK::RBK)?;
Ok(AN::ArrayType(Box::new(AArrayType {s:Span::d(),typ:Box::new(a0),})))
}
pub fn ptype_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
Ok(AN::TypeList(Box::new(ATypeList {s:Span::d(),typ:Box::new(a0),})))
}
pub fn pblock(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::LB)?;
let mut stmts:Vec<AN>=Vec::new();
while let Ok(v)=self.pstmt(){stmts.push(v)}
self.e(TK::RB)?;
Ok(AN::Block(Box::new(ABlock {s:Span::d(),stmt:stmts,})))
}
pub fn pstmt(&mut self)->Result<AN,String>{
if let Ok(v)=self.pvar_decl(){return Ok(v)}
if let Ok(v)=self.preturn_stmt(){return Ok(v)}
if let Ok(v)=self.pif_stmt(){return Ok(v)}
if let Ok(v)=self.pwhile_stmt(){return Ok(v)}
if let Ok(v)=self.pfor_stmt(){return Ok(v)}
if let Ok(v)=self.pmatch_stmt(){return Ok(v)}
if let Ok(v)=self.pbreak_stmt(){return Ok(v)}
if let Ok(v)=self.pcontinue_stmt(){return Ok(v)}
if let Ok(v)=self.pexpr_stmt(){return Ok(v)}
if let Ok(v)=self.pblock(){return Ok(v)}
Err(format!("no alt for Stmt at {}",self.tok().s.sl))
}
pub fn pvar_decl(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
self.e(TK::Eq)?;
let a1=self.pexpr()?;
self.e(TK::S)?;
Ok(AN::VarDecl(Box::new(AVarDecl {s:Span::d(),ident:Box::new(a0),expr:Box::new(a1),})))
}
pub fn preturn_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Ret)?;
let a0 = self.pexpr().ok();
self.e(TK::S)?;
Ok(AN::ReturnStmt(Box::new(AReturnStmt {s:Span::d(),expr:a0.map(|v|Box::new(v)),})))
}
pub fn pif_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::If)?;
let a0=self.pexpr()?;
let a1=self.pblock()?;
Ok(AN::IfStmt(Box::new(AIfStmt {s:Span::d(),expr:Box::new(a0),block:Box::new(a1),})))
}
pub fn pwhile_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Wh)?;
let a0=self.pexpr()?;
let a1=self.pblock()?;
Ok(AN::WhileStmt(Box::new(AWhileStmt {s:Span::d(),expr:Box::new(a0),block:Box::new(a1),})))
}
pub fn pfor_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Fr)?;
let a0=self.pi()?;
self.e(TK::In)?;
self.e(TK::LP)?;
let a1=self.pexpr()?;
self.e(TK::C)?;
let a2=self.pexpr()?;
self.e(TK::RP)?;
let a3=self.pblock()?;
Ok(AN::ForStmt(Box::new(AForStmt {s:Span::d(),ident:Box::new(a0),expr:Box::new(a1),expr_1:Box::new(a2),block:Box::new(a3),})))
}
pub fn pmatch_stmt(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Mt)?;
let a0=self.pexpr()?;
self.e(TK::LB)?;
let mut match_arms:Vec<AN>=Vec::new();
while let Ok(v)=self.pmatch_arm(){match_arms.push(v)}
self.e(TK::RB)?;
Ok(AN::MatchStmt(Box::new(AMatchStmt {s:Span::d(),expr:Box::new(a0),match_arm:match_arms,})))
}
pub fn pmatch_arm(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ppattern()?;
self.e(TK::FA)?;
let a1=self.pexpr()?;
self.e(TK::C)?;
Ok(AN::MatchArm(Box::new(AMatchArm {s:Span::d(),pattern:Box::new(a0),expr:Box::new(a1),})))
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
pub fn ppattern(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
Ok(AN::Pattern(Box::new(APattern {s:Span::d(),ident:Box::new(a0),})))
}
pub fn ppattern_args(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ppattern()?;
Ok(AN::PatternArgs(Box::new(APatternArgs {s:Span::d(),pattern:Box::new(a0),})))
}
pub fn pexpr(&mut self)->Result<AN,String>{
if let Ok(v)=self.pbinary_expr(){return Ok(v)}
if let Ok(v)=self.punary_expr(){return Ok(v)}
if let Ok(v)=self.pcall_expr(){return Ok(v)}
if let Ok(v)=self.pfield_expr(){return Ok(v)}
if let Ok(v)=self.pindex_expr(){return Ok(v)}
if let Ok(v)=self.pmatch_expr(){return Ok(v)}
if let Ok(v)=self.pif_expr(){return Ok(v)}
if let Ok(v)=self.pblock_expr(){return Ok(v)}
if matches!(self.tok().k,TK::Ident){return self.pi()}
if matches!(self.tok().k,TK::IntLit){return self.pn()}
//TODO alt float_literal
if matches!(self.tok().k,TK::StrLit){return self.ps()}
//TODO alt char_literal
if let Ok(v)=self.pbool_literal(){return Ok(v)}
if let Ok(v)=self.pstruct_literal(){return Ok(v)}
if let Ok(v)=self.parray_literal(){return Ok(v)}
Err(format!("no alt for Expr at {}",self.tok().s.sl))
}
pub fn pbinary_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
let a1=self.pi()?;
let a2=self.pexpr()?;
Ok(AN::BinaryExpr(Box::new(ABinaryExpr {s:Span::d(),expr:Box::new(a0),operator:Box::new(a1),expr_1:Box::new(a2),})))
}
pub fn punary_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
//TODO
let a0=self.pexpr()?;
Ok(AN::UnaryExpr(Box::new(AUnaryExpr {s:Span::d(),expr:Box::new(a0),})))
}
pub fn pcall_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
self.e(TK::LP)?;
let a1 = self.pexpr_list().ok();
self.e(TK::RP)?;
Ok(AN::CallExpr(Box::new(ACallExpr {s:Span::d(),ident:Box::new(a0),expr_list:a1.map(|v|Box::new(v)),})))
}
pub fn pfield_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
self.e(TK::Dt)?;
let a1=self.pi()?;
Ok(AN::FieldExpr(Box::new(AFieldExpr {s:Span::d(),expr:Box::new(a0),ident:Box::new(a1),})))
}
pub fn pindex_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
self.e(TK::LBK)?;
let a1=self.pexpr()?;
self.e(TK::RBK)?;
Ok(AN::IndexExpr(Box::new(AIndexExpr {s:Span::d(),expr:Box::new(a0),expr_1:Box::new(a1),})))
}
pub fn pmatch_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::Mt)?;
let a0=self.pexpr()?;
self.e(TK::LB)?;
let mut match_arms:Vec<AN>=Vec::new();
while let Ok(v)=self.pmatch_arm(){match_arms.push(v)}
self.e(TK::RB)?;
Ok(AN::MatchExpr(Box::new(AMatchExpr {s:Span::d(),expr:Box::new(a0),match_arm:match_arms,})))
}
pub fn pif_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::If)?;
let a0=self.pexpr()?;
let a1=self.pblock()?;
Ok(AN::IfExpr(Box::new(AIfExpr {s:Span::d(),expr:Box::new(a0),block:Box::new(a1),})))
}
pub fn pblock_expr(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pblock()?;
Ok(AN::BlockExpr(Box::new(ABlockExpr {s:Span::d(),block:Box::new(a0),})))
}
pub fn pstruct_literal(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.ptyp()?;
self.e(TK::LB)?;
let a1=self.pfield_init_list()?;
self.e(TK::RB)?;
Ok(AN::StructLiteral(Box::new(AStructLiteral {s:Span::d(),typ:Box::new(a0),field_init_list:Box::new(a1),})))
}
pub fn pfield_init(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pi()?;
self.e(TK::Eq)?;
let a1=self.pexpr()?;
Ok(AN::FieldInit(Box::new(AFieldInit {s:Span::d(),ident:Box::new(a0),expr:Box::new(a1),})))
}
pub fn pfield_init_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pfield_init()?;
Ok(AN::FieldInitList(Box::new(AFieldInitList {s:Span::d(),field_init:Box::new(a0),})))
}
pub fn parray_literal(&mut self)->Result<AN,String>{
let _s=self.tok().s;
self.e(TK::LBK)?;
let a0 = self.pexpr_list().ok();
self.e(TK::RBK)?;
Ok(AN::ArrayLiteral(Box::new(AArrayLiteral {s:Span::d(),expr_list:a0.map(|v|Box::new(v)),})))
}
pub fn pexpr_list(&mut self)->Result<AN,String>{
let _s=self.tok().s;
let a0=self.pexpr()?;
Ok(AN::ExprList(Box::new(AExprList {s:Span::d(),expr:Box::new(a0),})))
}
pub fn pbool_literal(&mut self)->Result<AN,String>{
if matches!(self.tok().k,TK::T){
let _s=self.tok().s;
self.e(TK::T)?;
return Ok(AN::BoolLiteral(Box::new(ABoolLiteral{s:Span::d(),})));
}
if matches!(self.tok().k,TK::F){
let _s=self.tok().s;
self.e(TK::F)?;
return Ok(AN::BoolLiteral(Box::new(ABoolLiteral{s:Span::d(),})));
}
Err(format!("no alt for BoolLiteral at {}",self.tok().s.sl))
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
fn visit_item(&mut self,n:&AItem)->T;
fn visit_import(&mut self,n:&AImport)->T;
fn visit_fn_decl(&mut self,n:&AFnDecl)->T;
fn visit_fn_param(&mut self,n:&AFnParam)->T;
fn visit_param_list(&mut self,n:&AParamList)->T;
fn visit_vis(&mut self,n:&AVis)->T;
fn visit_struct_def(&mut self,n:&AStructDef)->T;
fn visit_field(&mut self,n:&AField)->T;
fn visit_field_list(&mut self,n:&AFieldList)->T;
fn visit_enum_def(&mut self,n:&AEnumDef)->T;
fn visit_variant(&mut self,n:&AVariant)->T;
fn visit_variant_list(&mut self,n:&AVariantList)->T;
fn visit_interface_def(&mut self,n:&AInterfaceDef)->T;
fn visit_iface_method(&mut self,n:&AIfaceMethod)->T;
fn visit_self_param(&mut self,n:&ASelfParam)->T;
fn visit_impl_block(&mut self,n:&AImplBlock)->T;
fn visit_method(&mut self,n:&AMethod)->T;
fn visit_generic_params(&mut self,n:&AGenericParams)->T;
fn visit_generic_param(&mut self,n:&AGenericParam)->T;
fn visit_typ(&mut self,n:&AType)->T;
fn visit_type_base(&mut self,n:&ATypeBase)->T;
fn visit_shared_type(&mut self,n:&ASharedType)->T;
fn visit_unique_type(&mut self,n:&AUniqueType)->T;
fn visit_weak_type(&mut self,n:&AWeakType)->T;
fn visit_fn_type(&mut self,n:&AFnType)->T;
fn visit_array_type(&mut self,n:&AArrayType)->T;
fn visit_type_list(&mut self,n:&ATypeList)->T;
fn visit_block(&mut self,n:&ABlock)->T;
fn visit_stmt(&mut self,n:&AStmt)->T;
fn visit_var_decl(&mut self,n:&AVarDecl)->T;
fn visit_return_stmt(&mut self,n:&AReturnStmt)->T;
fn visit_if_stmt(&mut self,n:&AIfStmt)->T;
fn visit_while_stmt(&mut self,n:&AWhileStmt)->T;
fn visit_for_stmt(&mut self,n:&AForStmt)->T;
fn visit_match_stmt(&mut self,n:&AMatchStmt)->T;
fn visit_match_arm(&mut self,n:&AMatchArm)->T;
fn visit_break_stmt(&mut self,n:&ABreakStmt)->T;
fn visit_continue_stmt(&mut self,n:&AContinueStmt)->T;
fn visit_expr_stmt(&mut self,n:&AExprStmt)->T;
fn visit_pattern(&mut self,n:&APattern)->T;
fn visit_pattern_args(&mut self,n:&APatternArgs)->T;
fn visit_expr(&mut self,n:&AExpr)->T;
fn visit_binary_expr(&mut self,n:&ABinaryExpr)->T;
fn visit_unary_expr(&mut self,n:&AUnaryExpr)->T;
fn visit_call_expr(&mut self,n:&ACallExpr)->T;
fn visit_field_expr(&mut self,n:&AFieldExpr)->T;
fn visit_index_expr(&mut self,n:&AIndexExpr)->T;
fn visit_match_expr(&mut self,n:&AMatchExpr)->T;
fn visit_if_expr(&mut self,n:&AIfExpr)->T;
fn visit_block_expr(&mut self,n:&ABlockExpr)->T;
fn visit_struct_literal(&mut self,n:&AStructLiteral)->T;
fn visit_field_init(&mut self,n:&AFieldInit)->T;
fn visit_field_init_list(&mut self,n:&AFieldInitList)->T;
fn visit_array_literal(&mut self,n:&AArrayLiteral)->T;
fn visit_expr_list(&mut self,n:&AExprList)->T;
fn visit_bool_literal(&mut self,n:&ABoolLiteral)->T;
}

pub struct AstWalk;
impl<T:Default> AstVisit<T> for AstWalk{
fn visit_program(&mut self,_n:&AProgram)->T{T::default()}
fn visit_item(&mut self,_n:&AItem)->T{T::default()}
fn visit_import(&mut self,_n:&AImport)->T{T::default()}
fn visit_fn_decl(&mut self,_n:&AFnDecl)->T{T::default()}
fn visit_fn_param(&mut self,_n:&AFnParam)->T{T::default()}
fn visit_param_list(&mut self,_n:&AParamList)->T{T::default()}
fn visit_vis(&mut self,_n:&AVis)->T{T::default()}
fn visit_struct_def(&mut self,_n:&AStructDef)->T{T::default()}
fn visit_field(&mut self,_n:&AField)->T{T::default()}
fn visit_field_list(&mut self,_n:&AFieldList)->T{T::default()}
fn visit_enum_def(&mut self,_n:&AEnumDef)->T{T::default()}
fn visit_variant(&mut self,_n:&AVariant)->T{T::default()}
fn visit_variant_list(&mut self,_n:&AVariantList)->T{T::default()}
fn visit_interface_def(&mut self,_n:&AInterfaceDef)->T{T::default()}
fn visit_iface_method(&mut self,_n:&AIfaceMethod)->T{T::default()}
fn visit_self_param(&mut self,_n:&ASelfParam)->T{T::default()}
fn visit_impl_block(&mut self,_n:&AImplBlock)->T{T::default()}
fn visit_method(&mut self,_n:&AMethod)->T{T::default()}
fn visit_generic_params(&mut self,_n:&AGenericParams)->T{T::default()}
fn visit_generic_param(&mut self,_n:&AGenericParam)->T{T::default()}
fn visit_typ(&mut self,_n:&AType)->T{T::default()}
fn visit_type_base(&mut self,_n:&ATypeBase)->T{T::default()}
fn visit_shared_type(&mut self,_n:&ASharedType)->T{T::default()}
fn visit_unique_type(&mut self,_n:&AUniqueType)->T{T::default()}
fn visit_weak_type(&mut self,_n:&AWeakType)->T{T::default()}
fn visit_fn_type(&mut self,_n:&AFnType)->T{T::default()}
fn visit_array_type(&mut self,_n:&AArrayType)->T{T::default()}
fn visit_type_list(&mut self,_n:&ATypeList)->T{T::default()}
fn visit_block(&mut self,_n:&ABlock)->T{T::default()}
fn visit_stmt(&mut self,_n:&AStmt)->T{T::default()}
fn visit_var_decl(&mut self,_n:&AVarDecl)->T{T::default()}
fn visit_return_stmt(&mut self,_n:&AReturnStmt)->T{T::default()}
fn visit_if_stmt(&mut self,_n:&AIfStmt)->T{T::default()}
fn visit_while_stmt(&mut self,_n:&AWhileStmt)->T{T::default()}
fn visit_for_stmt(&mut self,_n:&AForStmt)->T{T::default()}
fn visit_match_stmt(&mut self,_n:&AMatchStmt)->T{T::default()}
fn visit_match_arm(&mut self,_n:&AMatchArm)->T{T::default()}
fn visit_break_stmt(&mut self,_n:&ABreakStmt)->T{T::default()}
fn visit_continue_stmt(&mut self,_n:&AContinueStmt)->T{T::default()}
fn visit_expr_stmt(&mut self,_n:&AExprStmt)->T{T::default()}
fn visit_pattern(&mut self,_n:&APattern)->T{T::default()}
fn visit_pattern_args(&mut self,_n:&APatternArgs)->T{T::default()}
fn visit_expr(&mut self,_n:&AExpr)->T{T::default()}
fn visit_binary_expr(&mut self,_n:&ABinaryExpr)->T{T::default()}
fn visit_unary_expr(&mut self,_n:&AUnaryExpr)->T{T::default()}
fn visit_call_expr(&mut self,_n:&ACallExpr)->T{T::default()}
fn visit_field_expr(&mut self,_n:&AFieldExpr)->T{T::default()}
fn visit_index_expr(&mut self,_n:&AIndexExpr)->T{T::default()}
fn visit_match_expr(&mut self,_n:&AMatchExpr)->T{T::default()}
fn visit_if_expr(&mut self,_n:&AIfExpr)->T{T::default()}
fn visit_block_expr(&mut self,_n:&ABlockExpr)->T{T::default()}
fn visit_struct_literal(&mut self,_n:&AStructLiteral)->T{T::default()}
fn visit_field_init(&mut self,_n:&AFieldInit)->T{T::default()}
fn visit_field_init_list(&mut self,_n:&AFieldInitList)->T{T::default()}
fn visit_array_literal(&mut self,_n:&AArrayLiteral)->T{T::default()}
fn visit_expr_list(&mut self,_n:&AExprList)->T{T::default()}
fn visit_bool_literal(&mut self,_n:&ABoolLiteral)->T{T::default()}
}


