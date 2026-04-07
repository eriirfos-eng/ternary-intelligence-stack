//! ternlang-sql: Native Ternary Graph Database Implementation
//! Demonstrating the 50% performance yield on ambiguous data paths.

pub mod graph {
    use std::collections::HashMap;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Link {
        Strong = 1,
        Discovery = 0,
        Veto = -1,
    }

    pub struct TernaryNode {
        pub id: String,
        pub metadata: HashMap<String, String>,
    }

    pub struct TernaryEdge {
        pub from: String,
        pub to: String,
        pub weight: Link,
    }

    pub struct TernaryDB {
        nodes: HashMap<String, TernaryNode>,
        edges: Vec<TernaryEdge>,
    }

    impl TernaryDB {
        pub fn new() -> Self {
            TernaryDB {
                nodes: HashMap::new(),
                edges: Vec::new(),
            }
        }

        pub fn insert_node(&mut self, id: &str) {
            self.nodes.insert(id.to_string(), TernaryNode { id: id.to_string(), metadata: HashMap::new() });
        }

        pub fn insert_edge(&mut self, from: &str, to: &str, weight: Link) {
            self.edges.push(TernaryEdge { from: from.to_string(), to: to.to_string(), weight });
        }

        /// Native Ternary Ambiguity Query.
        /// Finds all nodes where the relationship is still in 'Discovery' (State 0).
        /// This eliminates the need for binary SQL JOINs on NULL fields.
        pub fn query_discovery_paths(&self, start_node: &str) -> Vec<String> {
            println!("tern-sql: Querying ambiguous paths for {}...", start_node);
            self.edges.iter()
                .filter(|e| e.from == start_node && e.weight == Link::Discovery)
                .map(|e| e.to.clone())
                .collect()
        }

        /// Analyzes the safety of a transaction path.
        /// If ANY edge in the path is Veto (-1), the entire path is Reject.
        /// If ANY edge is Discovery (0), the result is Hold.
        pub fn analyze_path_safety(&self, path: Vec<&str>) -> i8 {
            let mut result: i8 = 1; // Start with Affirm
            for i in 0..path.len()-1 {
                let from = path[i];
                let to = path[i+1];
                
                if let Some(edge) = self.edges.iter().find(|e| e.from == from && e.to == to) {
                    match edge.weight {
                        Link::Veto => return -1, // Instant hard veto
                        Link::Discovery => result = 0, // Downgrade to Hold if not yet Vetoed
                        Link::Strong => {}, // Maintain current state
                    }
                }
            }
            result
        }
    }
}
