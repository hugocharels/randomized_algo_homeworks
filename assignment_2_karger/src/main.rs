mod core;

fn main() {
    let mut g = core::Graph::new();
    g.add_edge(0, 1);
    g.add_edge(0, 2);
    g.add_edge(1, 2);
    g.add_edge(1, 3);
    g.add_edge(2, 3);
    g.add_edge(0, 1);
    println!("{:?}", g);
}
