use petgraph::data::Build;

pub fn run() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("day25.txt")?;

    println!("25:1 - {}", run_1(&input)?);
    println!("25:2 - {}", run_2(&input)?);

    Ok(())
}

fn run_1(input: &str) -> anyhow::Result<usize> {
    let graph = parse(input);
    // dbg!{&graph};
    println!("{:?}", petgraph::dot::Dot::with_config(&graph, &[petgraph::dot::Config::EdgeNoLabel]));
    todo!()
}

fn run_2(_input: &str) -> anyhow::Result<usize> {
    todo!()
}

fn parse(i: &str) -> petgraph::graph::UnGraph<String, ()> {
    let mut res = petgraph::graph::UnGraph::new_undirected();
    for line in i.lines() {
        let mut colon_s = line.split(':');
        let n1 = colon_s.next().unwrap();
        let i1 = res.node_indices().find(|n| res[*n]==n1).unwrap_or_else(|| res.add_node(n1.to_string()));
        let nbrs = colon_s.next().unwrap().split(' ');
        nbrs.filter(|s| !s.is_empty()).for_each(|n2| {
            let i2 = res.node_indices().find(|n| res[*n]==n2).unwrap_or_else(|| res.add_node(n2.to_string()));
            res.add_edge(i1, i2, ());
        });
    }

    res
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn day25_run_1() {
        assert_eq!(super::run_1(INPUT).unwrap(), 54);
    }

    #[test]
    fn day25_run_2() {}
}
