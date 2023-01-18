use log::{debug, info};
use oxigraph::{
    io::GraphFormat,
    model::{GraphName, NamedNode},
    sparql::{EvaluationError, QueryResults, QuerySolution},
    store::Store,
};

use rio_turtle::TurtleParser;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{rename, File, OpenOptions},
    io::{self, BufRead, BufReader},
    process::Command,
};
use std::{io::Write, path::Path};
use tera::{Context, Tera};
use url::Url;

pub fn load_summary(path: String) -> TurtleParser<impl BufRead> {
    let file =
        File::open(path.clone()).unwrap_or_else(|e| panic!("Could not open file {}: {e}", path));
    let buf_reader = BufReader::new(file);
    debug!("extracting {:?}", path);
    let stream = BufReader::new(buf_reader);
    let parser = TurtleParser::new(stream, None);
    return parser;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisData {
    nodes: Vec<VisNode>,
    edges: Vec<VisEdge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisNode {
    id: i32,
    name: String,
    count: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VisEdge {
    source: i32,
    target: i32,
    count: i32,
    label: String,
    link_num: i32,
}

pub fn build_data(outf: &str) -> VisData {
    let qres = query_graph(outf);

    let mut nodes = BTreeMap::<i32, VisNode>::new();
    let mut nodes2ids = BTreeMap::<String, i32>::new();
    let mut edges = HashMap::<(i32, i32), Vec<VisEdge>>::new();

    if let Ok(QueryResults::Solutions(mut sols)) = qres {
        let mut id_count = 0;

        for s in sols {
            if let Ok(sol) = s {
                proc_solution(sol, &mut nodes, &mut edges, &mut nodes2ids, &mut id_count);
            }
        }
    }

    println!("edges {:#?}", edges);

    let mut sorted_edges = edges
        .into_iter()
        .map(|(_, v)| v)
        .flatten()
        .collect::<Vec<VisEdge>>();

    sorted_edges.sort_by(|a, b| b.count.cmp(&a.count));

    let mut sorted_nodes = nodes.into_values().collect::<Vec<_>>();
    sorted_nodes.sort_by(|a, b| b.count.cmp(&a.count));

    let data = VisData {
        edges: sorted_edges,
        nodes: sorted_nodes,
    };

    return data;
}

fn proc_solution(
    sol: QuerySolution,
    nodes: &mut BTreeMap<i32, VisNode>,
    edges: &mut HashMap<(i32, i32), Vec<VisEdge>>,
    nodes2ids: &mut BTreeMap<String, i32>,
    id_count: &mut i32,
) {
    let mut src = None;
    if let oxigraph::model::Term::NamedNode(n) = sol.get("src").unwrap() {
        src = get_fragment(n.clone());
    }

    let mut tgt = None;
    if let oxigraph::model::Term::NamedNode(n) = sol.get("tgt").unwrap() {
        tgt = get_fragment(n.clone());
    }

    let mut label = None;
    if let oxigraph::model::Term::NamedNode(n) = sol.get("label").unwrap() {
        label = get_fragment(n.clone());
    }

    let mut occurs = None;
    if let oxigraph::model::Term::Literal(l) = sol.get("occurs").unwrap() {
        occurs = Some(l.value());
    }

    if src.clone().is_some()
        && tgt.clone().is_some()
        && label.clone().is_some()
        && occurs.clone().is_some()
    {
        println!("XXXXXXXX {:?}", src);
        let src_id = *nodes2ids.entry(src.clone().unwrap()).or_insert_with(|| {
            let cur_id = id_count.clone();
            *id_count += 1;
            cur_id
        });

        let tgt_id = *nodes2ids.entry(tgt.clone().unwrap()).or_insert_with(|| {
            let cur_id = id_count.clone();
            *id_count += 1;
            cur_id
        });

        nodes
            .entry(src_id)
            .or_insert_with(|| VisNode {
                id: src_id,
                name: src.clone().unwrap(),
                count: 0,
            })
            .count += occurs.unwrap().parse::<usize>().unwrap();
        nodes
            .entry(tgt_id)
            .or_insert_with(|| VisNode {
                id: tgt_id,
                name: tgt.clone().unwrap(),
                count: 0,
            })
            .count += occurs.unwrap().parse::<usize>().unwrap();

        let key = sort_pair(src_id, tgt_id);
        let colliding = edges.entry(key).or_insert_with(|| Vec::new());
        colliding.push(VisEdge {
            source: src_id,
            target: tgt_id,
            count: occurs.unwrap().parse().unwrap(),
            label: label.clone().unwrap(),
            link_num: colliding.len() as i32,
        });

        for edge in colliding {
            edge.link_num += 1;
        }
    }
}

fn sort_pair(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

fn query_graph(outf: &str) -> Result<QueryResults, EvaluationError> {
    let file_path = Path::new(".").join(outf).join("output.ttl");
    let file = File::open(file_path.clone())
        .unwrap_or_else(|e| panic!("Could not open file {}: {e}", file_path.to_string_lossy()));
    let buf_reader = BufReader::new(file);
    debug!("extracting {:?}", file_path);
    let stream = BufReader::new(buf_reader);

    let store = Store::new().unwrap();

    let mut nodes = BTreeMap::<i32, VisNode>::new();
    let mut edges = Vec::<VisEdge>::new();
    let mut nodes2ids = BTreeMap::<String, i32>::new();

    let rdf_type = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let groups_link = "http://andrefs.com/graph-summ/GroupsLink";

    store
        .bulk_loader()
        .load_graph(stream, GraphFormat::Turtle, &GraphName::DefaultGraph, None)
        .unwrap();

    let q = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX afsgs: <http://andrefs.com/graph-summ/v1#>
    
        SELECT ?stmt_id ?src ?label ?tgt ?occurs WHERE {
            ?id rdf:type afsgs:GroupsLink .
            ?id rdf:subject ?src .
            ?id rdf:predicate ?label .
            ?id rdf:object ?tgt .
            ?id afsgs:occurrences ?occurs .
        }
        ORDER BY DESC(?occurs)
        "#;

    let qres = store.query(q);
    return qres;
}

fn get_fragment(n: NamedNode) -> Option<String> {
    Some(
        Url::parse(&n.into_string())
            .unwrap()
            .fragment()
            .unwrap()
            .to_string(),
    )
}

pub fn dump_json(data: &VisData, outf: &str) {
    let file_path = Path::new(".").join(outf).join("vis-data.json");
    info!(
        "Saving visualization data to {}",
        file_path.to_string_lossy()
    );

    let mut fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path.clone())
        .unwrap();

    writeln!(fd, "{}", serde_json::to_string_pretty(&data).unwrap()).unwrap();
}

pub fn render_vis(data: &VisData, outf: &str) {
    let RENDER_DIR = Path::new(".").join("chilon-viz");
    let tera = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("data", &data);

    let data_path = RENDER_DIR.join("src").join("data").join("raw-data.ts");

    info!("Copying data to {}", data_path.to_string_lossy());

    let data_fd = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(data_path.clone())
        .unwrap();

    tera.render_to("raw-data.ts", &ctx, data_fd).unwrap();

    info!("Building Vite");
    let output = Command::new("sh")
        .arg("-c")
        .arg("yarn build")
        .current_dir(RENDER_DIR.clone())
        .output()
        .expect("Failed to execute vite build");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let src = RENDER_DIR.join("dist");
    let dst = Path::new(outf).join("dist");
    info!(
        "Copying {} to {}",
        src.to_string_lossy(),
        dst.to_string_lossy()
    );
    rename(src, dst).unwrap();

    info!("Opening dev env");
    let output = Command::new("sh")
        .arg("-c")
        .arg("yarn dev")
        .current_dir(RENDER_DIR.clone())
        .output()
        .expect("Failed to execute vite build");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
