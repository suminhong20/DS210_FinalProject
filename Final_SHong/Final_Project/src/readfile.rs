
pub mod readfile{

    use std::fs::File;
    use std::io::{BufRead, BufReader};
    type Vertex = usize;
    //function that reads the file and returns a vector containing the edges for each node
    pub fn read_file(path: &str) -> Vec<(usize, usize)> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let mut edges = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();
            let vertices: Vec<&str> = line.trim().split_whitespace().collect();
            if vertices.len() == 2 {
                let u = vertices[0].parse::<Vertex>().unwrap();
                let v = vertices[1].parse::<Vertex>().unwrap();
                edges.push((u, v));
            }
        }
        return edges;
    }
}