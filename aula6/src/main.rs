// aula6/src/main.rs

use std::collections::VecDeque;

// Estrutura que representa um grafo não direcionado usando à lista de adjacências
struct Graph {
    vertices: usize,
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    // Cria um novo gráfico com o número especificado de vértices
    fn new (vertices:usize) -> Self {
        Graph {
            vertices,
            //inicializa um vetor com 'vertices' listas vazias
            adj_list: vec![Vec::new(); vertices], 
        }
    }

    // Adiciona uma aresta entre os vértices v e w (grafo não direcionado)
    fn add_edge(&mut self, v: usize, w: usize) {
        self.adj_list[v].push(w);
        self.adj_list[w].push(v);
    }

    // Função auxiliar para DFS recursivo
    fn dfs_util(&self, v: usize, visited: &mut Vec<bool>) {
        visited[v] = true;
        println!("Visitando vértice: {}", v);

        // Percorre todos os vizinhos do vértice atual
        for &neighbor in &self.adj_list[v] {
            if !visited[neighbor] {
                self.dfs_util(neighbor, visited);
            }
        }
    }

    // Executa a DFS a partir de um vértice inicial
    fn dfs(&self, start: usize) {
        let mut visited = vec![false; self.vertices];
        println!("DFS a partir do vértice: {}", start);
        self.dfs_util(start, &mut visited);
    }

    // Executa o BFS a partir de um vértice inicial
    fn bfs(&self, start: usize) {
        let mut visited = vec![false; self.vertices];
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        println!("BFS a partir do vértice: {}", start);
        while let Some(v) = queue.pop_front() {
            println!("Visitando vértice: {}", v);
            for &neighbor in &self.adj_list[v] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }
}

fn main() {
    // Cria um grafo com 5 vértices (0 a 4)
    let mut graph = Graph::new(5);

    // Adiciona arestas (grafo não direcionado)
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    // Realiza a travessia DFS a partir do vértice 0
    graph.dfs(0);
    println!("------------------");
    // Realiza a travessia BFS a partir do vértice 0
    graph.bfs(0);
}