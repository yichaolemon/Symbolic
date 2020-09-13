use crate::parser::Expression;
use std::collections::{HashMap, VecDeque, HashSet};
use crate::tree_transform::Equivalence;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Rc;
use std::ops::Deref;

/// Build a graph, where nodes are expressions, and edges are equivalences
struct Node<'b> {
	exp: Rc<Expression>,
	equiv_exps: Vec<(Rc<Expression>, &'b Equivalence, bool)>,
}

impl<'b> Node<'b> {
	fn new(exp: Rc<Expression>) -> Node<'b> {
		Node {
			exp,
			equiv_exps: Vec::new()
		}
	}

	fn add_equiv_exp(&mut self, exp: Rc<Expression>, equiv: &'b Equivalence, is_after: bool) {
		self.equiv_exps.push((exp, equiv, is_after));
	}
}

// 'b is lifetime of equivalences.
// Expressions are stored by
// The graph is just the structure.
// It takes references to externally owned Expressions and Equivalences,
// to avoid copying large expression trees.
pub struct Graph<'b> {
	map: HashMap<Expression, Node<'b>>,
	root: Rc<Expression>,
}

impl<'b> Graph<'b> {
	// before is already in the graph
	pub fn add_node(&mut self, before: Rc<Expression>, after: Rc<Expression>, equiv: &'b Equivalence) {
		let after_clone = after.as_ref().clone();
		let node_before = self.map.get_mut(before.as_ref()).unwrap();

		let mut node_after = Node::new(Rc::clone(&after));
		node_after.add_equiv_exp(before, equiv, false);
		node_before.add_equiv_exp(after, equiv, true);
		self.map.insert(after_clone, node_after);
	}

	fn bfs<E, F: FnMut(&Node) -> Result<(), E>>(&self, mut f: F) -> Result<(), E> {
		let mut visited_set = HashSet::new();
		let mut queue = VecDeque::new();
		visited_set.insert(Rc::clone(&self.root));
		queue.push_back(Rc::clone(&self.root));
		while !queue.is_empty() {
			let exp = queue.pop_front().unwrap();
			let node = self.map.get(exp.as_ref()).unwrap();
			f(node)?;
			for (equiv_exp, _, _) in node.equiv_exps.iter() {
				if !visited_set.contains(equiv_exp.as_ref()) {
					visited_set.insert(Rc::clone(&equiv_exp));
					queue.push_back(Rc::clone(&equiv_exp));
				}
			}
		};
		Ok(())
	}
}

impl fmt::Display for Graph<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.bfs(|n| write!(f, "{}\n", n.exp))
	}
}

pub fn create_graph<'b>(root: Rc<Expression>) -> Graph<'b> {
	let mut map = HashMap::new();
	let root_clone = root.deref().clone();
	let node = Node {
		exp: Rc::clone(&root),
		equiv_exps: Vec::new()
	};
	map.insert(root_clone, node);
	Graph { map, root }
}
