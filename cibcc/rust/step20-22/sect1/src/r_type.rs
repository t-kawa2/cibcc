use crate::parser::{Function, Node, NodeKind};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
	Int,
	Ptr,
	Array,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
	pub kind: TypeKind,
	pub base: Option<Box<Type>>,
	pub array_size: i64,
}

impl Type {
	pub fn new_type(kind: TypeKind, base: Option<Box<Type>>, array_size: i64) -> Self {
		Self {
			kind,
			base,
			array_size,
		}
	}
	pub fn size(&self) -> usize {
		match self.kind {
			TypeKind::Int => 8,
			TypeKind::Ptr => 8,
			TypeKind::Array => (self.base.as_ref().map_or(0, |b| b.size())) * self.array_size as usize,
		}
	}
	pub fn int_type() -> Self {
		Self {
			kind: TypeKind::Int,
			base: None,
			array_size: 0,
		}
	}
	pub fn pointer_to(base: Type) -> Self {
		Self {
			kind: TypeKind::Ptr,
			base: Some(Box::new(base)),
			array_size: 0,
		}
	}
}

pub fn add_type(prog: &mut Function) {
	if let Some(ref mut n) = prog.node {
		visit(n);
	}
}

pub fn array_of(base: Type, size: i64) -> Type {
	Type::new_type(TypeKind::Array, Some(Box::new(base)), size)
}

pub fn size_of(ty: &Type) -> i64 {
	match ty.kind {
		TypeKind::Int | TypeKind::Ptr => 8,
		TypeKind::Array => {
			let base_size = ty.base.as_ref().map_or(0, |b| size_of(b));
			base_size * ty.array_size
		}
	}
}

fn visit(node: &mut Node) {

	if let Some(ref mut n) = node.lhs { visit(n); }
	if let Some(ref mut n) = node.rhs { visit(n); }
	if let Some(ref mut n) = node.cond { visit(n); }
	if let Some(ref mut n) = node.then { visit(n); }
	if let Some(ref mut n) = node.els { visit(n); }
	if let Some(ref mut n) = node.init { visit(n); }
	if let Some(ref mut n) = node.inc { visit(n); }
	if let Some(ref mut n) = node.body { visit(n); }
	if let Some(ref mut n) = node.args { visit(n); }

	match &node.kind {
		NodeKind::Mul | NodeKind::Div | NodeKind::EQ | NodeKind::NE |
		NodeKind::LT  | NodeKind::LE  | NodeKind::Funcall => {
			node.ty = Some(Type::int_type());
		}
		NodeKind::Num(_) => {
			node.ty = Some(Type::int_type());
		}
		NodeKind::Var => {
			if let Some(ref var) = node.var {
				node.ty = Some(var.ty.clone());
			}
		}
		NodeKind::Add => {
			let rhs_is_ptr = node.rhs.as_ref()
				.and_then(|n| n.ty.as_ref())
				.and_then(|t| t.base.as_ref())
				.is_some();

			if rhs_is_ptr {
				std::mem::swap(&mut node.lhs, &mut node.rhs);
			}

			let lhs_ty = node.lhs.as_ref().and_then(|n| n.ty.as_ref());
			let lhs_base = lhs_ty.and_then(|t| t.base.as_ref());
			let rhs_ty = node.rhs.as_ref().and_then(|n| n.ty.as_ref());
			let rhs_base = rhs_ty.and_then(|t| t.base.as_ref());

			if lhs_base.is_some() && rhs_base.is_some() {
				eprintln!("invalid pointer arithmetic operands");
			}

			node.ty = node.lhs.as_ref().and_then(|n| n.ty.clone());
		}
		NodeKind::Sub => {
			if let Some(rhs_ty) = node.rhs.as_ref().and_then(|n| n.ty.as_ref()) {
				if let Some(_rhs_base) = rhs_ty.base.as_ref() {
					eprintln!("invalid pointer arithmetic operands");
				}
			}
			if let Some(ref lhs) = node.lhs {
				node.ty = lhs.ty.clone();
			}
		}
		NodeKind::Assign => {
			if let Some(ref lhs) = node.lhs {
				node.ty = lhs.ty.clone();
			}
		}
		NodeKind::Addr => {
			if let Some(ref ty) = node.lhs.as_ref().and_then(|n| n.ty.as_ref()) {
				if ty.kind == TypeKind::Array {
					if let Some(ref base_ty) = ty.base {
						node.ty = Some(Type::pointer_to((**base_ty).clone()));
					}
				} else {
					node.ty = Some(Type::pointer_to((*ty).clone()));
				}
			}
		}
		NodeKind::Deref => {
			if let Some(ref ty) = node.lhs.as_ref().and_then(|n| n.ty.as_ref()) {
				if let Some(ref base) = ty.base {
					node.ty = Some((**base).clone());
				}
			}
		}
		_ => {}
	}
}

