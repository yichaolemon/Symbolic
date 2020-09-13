use crate::parser::Expression;
use std::collections::{HashMap, VecDeque, HashSet};
use crate::tree_transform::Equivalence;
use std::fmt;
use std::fmt::Formatter;

/// Build a graph, where nodes are expressions, and edges are equivalences
struct Node<'a, 'b> {
	exp: &'a Expression,
	equiv_exps: Vec<(&'a Expression, &'b Equivalence, bool)>,
}

impl<'a, 'b> Node<'a, 'b> {
	fn new(exp: &'a Expression) -> Node<'a, 'b> {
		Node {
			exp,
			equiv_exps: Vec::new()
		}
	}

	fn add_equiv_exp(&mut self, exp: &'a Expression, equiv: &'b Equivalence, is_after: bool) {
		self.equiv_exps.push((exp, equiv, is_after));
	}
}

struct Graph<'a, 'b> {
	map: HashMap<Expression, Node<'a, 'b>>,
	root: &'a Expression,
}

impl<'a, 'b> Graph<'a, 'b> {
	// before is already in the graph
	pub fn add_node(&mut self, before: &'a Expression, after: Expression, equiv: &'b Equivalence) {
		let node_before = self.map.get_mut(before).unwrap();

		let mut node_after = Node::new(&after);
		node_after.add_equiv_exp(before, equiv, false);
		node_before.add_equiv_exp(&after, equiv, true);
		self.map.insert(after, node_after);
	}

	fn bfs<E, F: Fn(&Node) -> Result<(), E>>(&self, f: F) -> Result<(), E> {
		let mut visited_set = HashSet::new();
		let mut queue = VecDeque::new();
		let add_to_queue = |e| if !visited_set.contains(e) {
			visited_set.insert(e);
			queue.push_back(e)
		};
		add_to_queue(self.root);
		while !queue.is_empty() {
			let exp = queue.pop_front().unwrap();
			let node = self.map.get(exp).unwrap();
			f(node)?;
			for (equiv_exp, _, _) in node.equiv_exps.into_iter() {
				add_to_queue(equiv_exp);
			}
		};
		Ok(())
	}
}

impl fmt::Display for Graph<'_, '_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		self.bfs(|n| write!(f, "{}\n", n.exp))
	}
}

pub fn create_graph<'a, 'b>(root: Expression) -> Graph<'a, 'b> {
	let mut map = HashMap::new();
	let node = Node {
		exp: &root,
		equiv_exps: Vec::new()
	};
	map.insert(root, node);
	Graph { map, root: &root}
}
