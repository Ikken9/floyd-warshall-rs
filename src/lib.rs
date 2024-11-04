use std::collections::HashMap;

type VertexId = char;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub id: VertexId,
    pub edges: Vec<Edge>
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub to: VertexId,
    pub weight: u32
}

pub struct Graph {
    pub vertices: HashMap<VertexId, Vertex>
}

impl Graph {
    pub fn floyd_warshall(adj_matrix: &mut Vec<Vec<f64>>) {
        let n = adj_matrix.len();
        
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if adj_matrix[i][k] < f64::INFINITY && adj_matrix[k][j] < f64::INFINITY {
                        let new_distance = adj_matrix[i][k] + adj_matrix[k][j];
                        if new_distance < adj_matrix[i][j] {
                            adj_matrix[i][j] = new_distance;
                        }
                    }
                }
            }
        }
    }

    pub fn print_distance_matrix(adj_matrix: &Vec<Vec<f64>>) {
        let n = adj_matrix.len();
        
        print!("    ");
        for i in 0..n {
            print!("  {}  ", (b'A' + i as u8) as char);
        }
        println!();
        
        for i in 0..n {
            print!("{}  ", (b'A' + i as u8) as char);
            for j in 0..n {
                if adj_matrix[i][j] == f64::INFINITY {
                    print!(" INF ");
                } else {
                    print!("{:5.1}", adj_matrix[i][j]);
                }
            }
            println!();
        }
    }
}