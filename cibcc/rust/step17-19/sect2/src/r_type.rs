use crate::parser::{Function, Node, NodeKind};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
	Int,
	Ptr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Type {
	pub kind: TypeKind,
	pub base: Option<Box<Type>>,
}

impl Type {
	pub fn int_type() -> Self {
		Self {
			kind: TypeKind::Int,
			base: None,
		}
	}
	pub fn pointer_to(base: Type) -> Self {
		Self {
			kind: TypeKind::Ptr,
			base: Some(Box::new(base)),
		}
	}
}

pub fn add_type(prog: &mut Function) {
	let mut node_ptr = prog.node.as_deref_mut();
	while let Some(n) = node_ptr {
		visit(n);
		node_ptr = n.next.as_deref_mut();
	}
}

fn visit(node: &mut Node) {

	if let Some(n) = node.lhs.as_deref_mut() { visit(n); }
	if let Some(n) = node.rhs.as_deref_mut() { visit(n); }
	if let Some(n) = node.cond.as_deref_mut() { visit(n); }
	if let Some(n) = node.then.as_deref_mut() { visit(n); }
	if let Some(n) = node.els.as_deref_mut() { visit(n); }
	if let Some(n) = node.init.as_deref_mut() { visit(n); }
	if let Some(n) = node.inc.as_deref_mut() { visit(n); }
	if let Some(n) = node.body.as_deref_mut() { visit(n); }
	if let Some(n) = node.args.as_deref_mut() { visit(n); }

	match &node.kind {
		NodeKind::Mul | NodeKind::Div | NodeKind::EQ | NodeKind::NE |
		NodeKind::LT  | NodeKind::LE  | NodeKind::Funcall | NodeKind::Num(_) => {
			node.ty = Some(Type::int_type());
		}
		NodeKind::Var => {
			if let Some(ref var) = node.var {
				node.ty = Some(var.ty.clone());
			}
		}
		NodeKind::Add => {
			let lhs_is_ptr = node.lhs.as_ref().and_then(|n| n.ty.as_ref()).map_or(false, |t| t.kind == TypeKind::Ptr);
			let rhs_is_ptr = node.rhs.as_ref().and_then(|n| n.ty.as_ref()).map_or(false, |t| t.kind == TypeKind::Ptr);

			if rhs_is_ptr && !lhs_is_ptr {
				std::mem::swap(&mut node.lhs, &mut node.rhs);
			}
			if let Some(ref lhs) = node.lhs {
				node.ty = lhs.ty.clone();
			}
		}
		NodeKind::Sub => {
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
			if let Some(ref lhs) = node.lhs {
				if let Some(ref ty) = lhs.ty {
					node.ty = Some(Type::pointer_to(ty.clone()));
				}
			}
		}
		NodeKind::Deref => {
			if let Some(ref lhs) = node.lhs {
				if let Some(ref ty) = lhs.ty {
					if ty.kind == TypeKind::Ptr {
						node.ty = ty.base.as_ref().map(|b| *b.clone());
					} else {
						node.ty = Some(Type::int_type());
					}
				}
			}
		}
		_ => {}
	}
}

