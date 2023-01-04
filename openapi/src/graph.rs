use std::collections::{HashMap, HashSet};

use petgraph::dot::{Config, Dot};
use petgraph::Graph;

use crate::codegen::CodeGen;
use crate::schema_path::ComponentPath;

impl CodeGen {
    pub fn get_graphviz_dep_graph(&self) -> String {
        let graph = self.gen_dep_graph();
        format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]))
    }

    pub fn gen_dep_graph(&self) -> Graph<&ComponentPath, ()> {
        let mut graph: Graph<&ComponentPath, ()> = Graph::new();
        let mut node_map = HashMap::new();
        for obj in self.components.keys() {
            let node_ind = graph.add_node(obj);
            node_map.insert(obj, node_ind);
        }
        for (path, component) in &self.components {
            let mut dep_paths = HashSet::new();
            for dep in component.object.schema_deps() {
                // Don't clutter with self edges since they aren't particularly meaningful
                // in this context
                if dep != path {
                    dep_paths.insert(dep);
                }
            }
            for dep in dep_paths {
                graph.add_edge(*node_map.get(path).unwrap(), *node_map.get(dep).unwrap(), ());
            }
        }
        graph
    }
}
