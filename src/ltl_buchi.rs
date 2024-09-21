use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use crate::LtlFormula;
use crate::LtlFormula::{*};

#[derive(Debug)]
pub struct OnTheFlyLtl {
    node_sequence_counter: u8,
    node_set: NodeSet
}

impl OnTheFlyLtl {
    pub fn new() -> Self {
        Self {
            node_sequence_counter: 0,
            node_set: NodeSet::new()
        }
    }

    pub fn create_graph(&mut self, ltl_formula: &LtlFormula<u8>) {
        self.node_set.clear();
        self.node_sequence_counter = 0;
        let formula_node = Node::init_formula(&self.generate_new_node_name(), ltl_formula);
        self.expand(formula_node);
    }

    fn generate_new_node_name(&mut self) -> String {
        self.node_sequence_counter += 1;
        self.node_sequence_counter.to_string()
    }

    fn expand(&mut self, node: Node) {
        if node.is_fully_expanded() {
            self.process_fully_expanded_node(node);
        } else {
            self.process_node(node);
        }
    }

    fn process_fully_expanded_node(&mut self, node: Node) {
        let fully_expanded_node_hash_code = node.calculate_hash_fully_expanded_node();
        if let Some(nd) = self.node_set.get_mut(&fully_expanded_node_hash_code) {
            nd.incoming.extend(node.incoming);
        } else {
            let node_name = node.name.clone();
            let node_next_formulae = node.next.clone();
            let new_node_name = self.generate_new_node_name();
            self.node_set.add(node);

            self.expand(
                Node {
                    name: new_node_name.clone(),
                    father: new_node_name,
                    incoming: vec![node_name],
                    new: node_next_formulae,
                    old: vec![],
                    next: vec![],
                }
            )
        }
    }

    fn process_node(&mut self, mut node: Node) {
        let new_formula = node.pop_new_formula().expect("Should not be fully expanded yet!");

        match &new_formula {
            Not(formula) if !formula.is_literal() => {
                panic!("The formula is not in normal form!");
            },

            formula if formula.is_literal() => {
                self.process_literal(node, formula.clone());
            },

            Until(_, _) | Release(_, _) | Or(_, _) => {
                self.split_node(node, new_formula.clone());
            },

            And(left_formula, right_formula) => {
                self.expand_conjunction(node, *left_formula.clone(), *right_formula.clone(), new_formula.clone());
            },

            Next(inner_next_formula) => {
                self.expand_next(node, new_formula.clone(), *inner_next_formula.clone());
            }

            _ => panic!()
        }
    }

    fn process_literal(&mut self, mut node: Node, literal: LtlFormula<u8>) {
        if Bottom == literal || node.contains_processed_formula(&LtlFormula::not(literal.clone())) {
            return;
        }

        let mut node_clone = node.clone();
        node_clone.name = self.generate_new_node_name();
        node_clone.old.push(literal);
        self.expand(node_clone);
    }

    fn split_node(&mut self, node: Node, formula: LtlFormula<u8>) {
        let new_1_formula = Self::new_1(&formula);
        let mut new_node_1_new = node.new.clone();
        if !node.old.contains(&new_1_formula) {
            new_node_1_new.push(new_1_formula);
        }
        let mut new_node_1_old = node.old.clone();
        new_node_1_old.push(formula.clone());
        let mut new_node_1_next = node.next.clone();
        new_node_1_next.extend(Self::next_1(&formula));
        let node_1 = Node {
            name: self.generate_new_node_name(),
            father: node.name.clone(),
            incoming: node.incoming.clone(),
            new: new_node_1_new,
            old: new_node_1_old,
            next: new_node_1_next
        };

        let mut new_node_2_new = node.new.clone();
        for new_2_formula in Self::new_2(&formula) {
            if !node.old.contains(&new_2_formula) {
                new_node_2_new.push(new_2_formula);
            }
        }
        let mut new_node_2_old = node.old.clone();
        new_node_2_old.push(formula.clone());
        let node_2 = Node {
            name: self.generate_new_node_name(),
            father: node.name.clone(),
            incoming: node.incoming.clone(),
            new: new_node_2_new,
            old: new_node_2_old,
            next: node.next.clone()
        };

        self.expand(node_1);
        self.expand(node_2);
    }

    fn new_1(ltl_formula: &LtlFormula<u8>) -> LtlFormula<u8> {
        match ltl_formula {
            Until(left_formula, _) => *left_formula.clone(),
            Release(_, right_formula) => *right_formula.clone(),
            Or(left_formula, _) => *left_formula.clone(),
            _ => panic!()
        }
    }

    fn new_2(ltl_formula: &LtlFormula<u8>) -> Vec<LtlFormula<u8>> {
        match ltl_formula {
            Until(_, right_formula) => vec![*right_formula.clone()],
            Release(left_formula, right_formula) => vec![*left_formula.clone(), *right_formula.clone()],
            Or(_, right_formula) => vec![*right_formula.clone()],
            _ => panic!()
        }
    }

    fn next_1(ltl_formula: &LtlFormula<u8>) -> Vec<LtlFormula<u8>> {
        match ltl_formula {
            Until(left_formula, right_formula) => vec![Until(left_formula.clone(), right_formula.clone())],
            Release(left_formula, right_formula) => vec![Release(left_formula.clone(), right_formula.clone())],
            Or(_, _) => vec![],
            _ => panic!()
        }
    }

    fn expand_conjunction(
        &mut self,
        node: Node,
        left_formula: LtlFormula<u8>,
        right_formula: LtlFormula<u8>,
        formula: LtlFormula<u8>
    ) {
        let mut new_node_new = node.new.clone();
        if !new_node_new.contains(&left_formula) {
            new_node_new.push(left_formula);
        }
        if !new_node_new.contains(&right_formula) {
            new_node_new.push(right_formula);
        }

        let mut new_node_old = node.old.clone();
        new_node_old.push(formula);

        let new_node = Node {
            name: node.name.clone(),
            father: node.father.clone(),
            incoming: node.incoming.clone(),
            new: new_node_new,
            old: new_node_old,
            next: node.next.clone()
        };

        self.expand(new_node);
    }

    fn expand_next(&mut self, node: Node, next_formula: LtlFormula<u8>, inner_next_formula: LtlFormula<u8>) {
        let mut new_node_old = node.old.clone();
        new_node_old.push(next_formula);
        let mut new_node_next = node.next.clone();
        new_node_next.push(inner_next_formula);

        let new_node = Node {
            name: node.name.clone(),
            father: node.father.clone(),
            incoming: node.incoming.clone(),
            old: new_node_old,
            next: new_node_next,
            new: node.new.clone()
        };

        self.expand(new_node);
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    name: String,
    father: String,
    incoming: Vec<String>,
    old: Vec<LtlFormula<u8>>,
    next: Vec<LtlFormula<u8>>,
    new: Vec<LtlFormula<u8>>,
}

impl Node {
    const INIT: &'static str = "INIT";

    pub fn init_formula(name: &str, ltl_formula: &LtlFormula<u8>) -> Self {
        Self {
            name: name.to_string(),
            father: name.to_string(),
            incoming: vec![Self::INIT.to_string()],
            old: vec![],
            next: vec![],
            new: vec![ltl_formula.clone()],
        }
    }

    pub fn is_fully_expanded(&self) -> bool {
        self.new.is_empty()
    }

    pub fn pop_new_formula(&mut self) -> Option<LtlFormula<u8>> {
        self.new.pop()
    }

    pub fn contains_processed_formula(&self, ltl_formula: &LtlFormula<u8>) -> bool {
        self.old.contains(ltl_formula)
    }

    pub fn calculate_hash_fully_expanded_node(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.old.hash(state);
        self.next.hash(state);
    }
}

#[derive(Debug)]
struct NodeSet {
    node_by_fully_expanded_node_hash_code: HashMap<u64, Node>,
}

impl NodeSet {
    fn new() -> Self {
        Self {
            node_by_fully_expanded_node_hash_code: HashMap::with_capacity(10),
        }
    }

    fn clear(&mut self) {
        self.node_by_fully_expanded_node_hash_code.clear()
    }

    fn add(&mut self, node: Node) -> bool {
        let fully_expanded_node_hash_code = node.calculate_hash_fully_expanded_node();

        if self.contains(&node) {
            return false;
        }

        self.node_by_fully_expanded_node_hash_code.insert(fully_expanded_node_hash_code, node);
        true
    }

    fn contains(&self, node: &Node) -> bool {
        let fully_expanded_node_hash_code = node.calculate_hash_fully_expanded_node();
        self.node_by_fully_expanded_node_hash_code.contains_key(&fully_expanded_node_hash_code)
    }

    fn get_mut(&mut self, fully_expanded_node_hash_code: &u64) -> Option<&mut Node> {
        self.node_by_fully_expanded_node_hash_code.get_mut(fully_expanded_node_hash_code)
    }
}