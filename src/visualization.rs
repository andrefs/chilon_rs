use log::{debug, info};
use oxigraph::{
    io::GraphFormat,
    model::{GraphName, NamedNode},
    sparql::{EvaluationError, QueryResults, QuerySolution},
    store::{StorageError, Store},
};

use rio_turtle::TurtleParser;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{rename, File, OpenOptions},
    io::{self, BufRead, BufReader},
    path::PathBuf,
    process::Command,
};
use std::{io::Write, path::Path};
use tera::{Context, Tera};
use url::Url;

pub fn load_summary(path: String) -> TurtleParser<impl BufRead> {
    let file =
        File::open(path.clone()).unwrap_or_else(|e| panic!("Could not open file {}: {e}", path));
    let buf_reader = BufReader::new(file);
    info!("extracting {:?}", path);
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
    name: String,
    count: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VisEdge {
    source: String,
    target: String,
    count: i32,
    label: String,
    is_datatype: bool,
    link_num: i32, // number signal indicates direction for edge path calcs
}

pub fn build_data(outf: &str) -> VisData {
    let store = load_store(outf);
    let qres = query_graph(store);

    let mut nodes = BTreeMap::<String, VisNode>::new();
    let mut edges = HashMap::<(String, String), Vec<VisEdge>>::new();

    if let Ok(QueryResults::Solutions(mut sols)) = qres {
        let mut id_count = 0;

        for s in sols {
            if let Ok(sol) = s {
                proc_solution(sol, &mut nodes, &mut edges);
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
    nodes: &mut BTreeMap<String, VisNode>,
    edges: &mut HashMap<(String, String), Vec<VisEdge>>,
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

    let mut is_datatype = false;
    if let oxigraph::model::Term::NamedNode(n) = sol.get("type").unwrap() {
        if let Some(type_val) = get_fragment(n.clone()) {
            if type_val == "DatatypeLink" {
                is_datatype = true
            }
        }
    }

    if let (Some(src_name), Some(tgt_name), Some(edge_label), Some(occurs_val)) =
        (src, tgt, label, occurs)
    {
        nodes
            .entry(src_name.clone())
            .or_insert_with(|| VisNode {
                name: src_name.clone(),
                count: 0,
            })
            .count += occurs_val.parse::<usize>().unwrap();
        nodes
            .entry(tgt_name.clone())
            .or_insert_with(|| VisNode {
                name: tgt_name.clone(),
                count: 0,
            })
            .count += occurs_val.parse::<usize>().unwrap();

        let key = sort_pair(src_name.clone(), tgt_name.clone());
        let colliding = edges.entry(key.clone()).or_insert_with(|| Vec::new());
        let signal = if src_name == key.0 { 1 } else { -1 };

        colliding.push(VisEdge {
            source: src_name.clone(),
            target: tgt_name.clone(),
            count: occurs_val.parse().unwrap(),
            label: edge_label,
            is_datatype,
            link_num: signal * (colliding.len() + 1) as i32,
        });
    }
}

fn sort_pair(a: String, b: String) -> (String, String) {
    match a.cmp(&b) {
        std::cmp::Ordering::Greater => (b, a),
        _ => (a, b),
    }
}

fn load_store(outf: &str) -> Store {
    let file_path = Path::new(".").join(outf).join("output.ttl");
    let file = File::open(file_path.clone())
        .unwrap_or_else(|e| panic!("Could not open file {}: {e}", file_path.to_string_lossy()));
    let buf_reader = BufReader::new(file);
    info!("extracting {:?}", file_path);
    let stream = BufReader::new(buf_reader);

    let store = Store::new().unwrap();

    store
        .bulk_loader()
        .load_graph(stream, GraphFormat::Turtle, &GraphName::DefaultGraph, None)
        .unwrap();
    store
}

fn query_graph(store: Store) -> Result<QueryResults, EvaluationError> {
    let q = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX afsgs: <http://andrefs.com/graph-summ/v1#>
    
        SELECT ?stmt_id ?src ?label ?tgt ?type ?occurs WHERE {
            VALUES ?type { afsgs:GroupsLink afsgs:DatatypeLink }
            ?stmt_id rdf:type ?type  .
            ?stmt_id rdf:subject ?src .
            ?stmt_id rdf:predicate ?label .
            ?stmt_id rdf:object ?tgt .
            ?stmt_id afsgs:occurrences ?occurs .
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

pub fn render_vis(data: &VisData, outf: &str) -> PathBuf {
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
        .arg("yarn build-no-tsc")
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

    return RENDER_DIR;
}

pub fn vis_dev_server(dir: PathBuf) {
    info!("Opening dev env");
    let output = Command::new("sh")
        .arg("-c")
        .arg("yarn dev")
        .current_dir(dir)
        .output()
        .expect("Failed to execute vite build");

    //io::stdout().write_all(&output.stdout).unwrap();
    //io::stderr().write_all(&output.stderr).unwrap();
}
