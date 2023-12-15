pub mod bfs{
    pub type Vertex = usize;
    pub type ListOfEdges = Vec<(Vertex,Vertex)>;
    pub type AdjacencyLists = Vec<Vec<Vertex>>;

    #[derive(Debug)]
    pub struct Graph {
        pub n: usize, // number of vertices
        outedges: AdjacencyLists, // Adjacency List
    }

    //reverse edges
    pub fn reverse_edges(list:&ListOfEdges)
        -> ListOfEdges {
        let mut new_list: Vec<(usize, usize)> = vec![];
        for (u,v) in list {
            new_list.push((*v,*u));
        }
        new_list
    }
    //generate adjacency list
    pub fn generate_adjacency_list (n: &usize, edges: &Vec<(usize, usize)>) -> AdjacencyLists{
        let mut graph_list: AdjacencyLists = vec![vec![];*n];
        println!("{}", graph_list.len());
        for (v,w) in edges.iter() {
            graph_list[*v].push(*w);
            graph_list[*w].push(*v);
        };
        return graph_list;
    }
    impl Graph {
        //add directed edges to graphs
        pub fn add_directed_edges(&mut self,
                          edges:&ListOfEdges) {
            for (u,v) in edges {
                self.outedges[*u].push(*v);
            }
        }
        //sorts the graph 
        pub fn sort_graph_lists(&mut self) {
            for adj_l in self.outedges.iter_mut() {
                adj_l.sort();
            }
        }
        //creates a directed graph from list of edge
        pub fn create_directed(n:usize,edges:&ListOfEdges) -> Graph {
            let mut graph = Graph{n,outedges:vec![vec![];n]};
            graph.add_directed_edges(edges);
            graph.sort_graph_lists();
            graph                                        
        }
        //creates an undirected graph from list of edge
        pub fn create_undirected(n:usize,edges:&ListOfEdges) -> Graph {
            let mut graph = Self::create_directed(n,edges);
            graph.add_directed_edges(&reverse_edges(edges));
            graph.sort_graph_lists();
            graph                                        
        }
    
        
    }

    use std::collections::VecDeque;
    pub fn compute_distances_bfs(start: Vertex, graph: &Graph) -> Vec<usize>{
        let mut distance: Vec<Option<usize>> = vec![None;graph.n];
        distance[start] = Some(0);
        let mut queue: VecDeque<Vertex> = VecDeque::new();
        queue.push_back(start);
        
        while let Some(v) = queue.pop_front(){
            for u in graph.outedges[v].iter(){
                if let None = distance[*u]{
                    distance[*u] = Some(distance[v].unwrap() + 1);
                    queue.push_back(*u);
                }
            }
        }
          let mut distancesfromnode: Vec<usize> = Vec::new();
        for v in 0..graph.n{
            if distance[v].unwrap() == 0{
            }
            else{
                distancesfromnode.push(distance[v].unwrap());
            }
        }
        return distancesfromnode
    }

}


