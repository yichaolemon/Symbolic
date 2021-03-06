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

  fn add_equiv_exp(&mut self, exp: Rc<Expression>, equiv: &'b Equivalence, reverse: bool) {
    self.equiv_exps.push((exp, equiv, reverse));
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
  // if after is already in the graph, we still add the edges but return false.
  pub fn add_node(&mut self, before: Rc<Expression>, after: Rc<Expression>, equiv: &'b Equivalence) -> bool {
    let node_before = self.map.get_mut(before.as_ref()).unwrap();
    node_before.add_equiv_exp(Rc::clone(&after), equiv, false);

    let (node_after, is_new) = match self.map.get_mut(after.as_ref()) {
      Some(node_after) => (node_after, false),
      None => {
        let node_after = Node::new(Rc::clone(&after));
        self.map.insert(after.as_ref().clone(), node_after);
        (self.map.get_mut(after.as_ref()).unwrap(), true)
      },
    };
    node_after.add_equiv_exp(before, equiv, true);
    is_new
  }

  fn bfs<E, F: FnMut(&Node, i32, &Expression) -> Result<(), E>>(&self, mut f: F) -> Result<(), E> {
    let mut visited_set = HashSet::new();
    let mut queue = VecDeque::new();
    visited_set.insert(Rc::clone(&self.root));
    queue.push_back((Rc::clone(&self.root), 0, Rc::clone(&self.root)));
    while !queue.is_empty() {
      let (exp, depth, backedge) = queue.pop_front().unwrap();
      let node = self.map.get(exp.as_ref()).unwrap();
      f(node, depth, backedge.as_ref())?;
      for (equiv_exp, _, _) in node.equiv_exps.iter() {
        if !visited_set.contains(equiv_exp) {
          visited_set.insert(Rc::clone(equiv_exp));
          queue.push_back((Rc::clone(equiv_exp), depth+1, Rc::clone(&exp)));
        }
      }
    };
    Ok(())
  }

  pub fn size(&self) -> usize {
    self.map.len()
  }
}

impl fmt::Display for Graph<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    self.bfs(|n, d, backedge| {
      let mut relevant_edge: Option<(&Equivalence, bool)> = None;
      for (e, equiv, reverse) in n.equiv_exps.iter() {
        if (**e) == *backedge {
          relevant_edge = Some((equiv, *reverse));
        }
      }
      match relevant_edge {
        Some((equiv, reverse)) => write!(f, "{}: {} (from {} via {} {:?})\n", d, n.exp, backedge, equiv, reverse),
        None => write!(f, "{}: {} (from {})\n", d, n.exp, backedge)
      }
    })
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
