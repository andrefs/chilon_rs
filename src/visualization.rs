use log::{info, warn};
use oxigraph::{
    io::RdfFormat,
    model::NamedNode,
    sparql::{QueryEvaluationError, QueryResults, QuerySolution, SparqlEvaluator},
    store::Store,
};

use fs_extra::dir::copy;

use rio_turtle::TurtleParser;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{remove_dir_all, File, OpenOptions},
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

    TurtleParser::new(stream, None)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisData {
    nodes: Vec<VisNode>,
    edges: Vec<VisEdge>,
    aliases: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VisNodeType {
    Unknown,
    Blank,
    Namespace,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisNode {
    name: String,
    count: usize,
    node_type: VisNodeType,
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
    let qres1 = query_norm_triples(store.clone());
    let qres2 = query_aliases(store);

    let mut nodes = BTreeMap::<String, VisNode>::new();
    let mut edges = HashMap::<(String, String), Vec<VisEdge>>::new();
    let mut aliases = HashMap::<String, String>::new();

    if let Ok(QueryResults::Solutions(sols)) = qres1 {
        let _id_count = 0;

        for sol in sols.flatten() {
            proc_norm_triples(sol, &mut nodes, &mut edges);
        }
    }

    if let Ok(QueryResults::Solutions(sols)) = qres2 {
        for sol in sols.flatten() {
            proc_alias(sol, &mut aliases);
        }
    }

    let mut sorted_edges = edges.into_values().flatten().collect::<Vec<VisEdge>>();

    sorted_edges.sort_by(|a, b| b.count.cmp(&a.count));

    let mut sorted_nodes = nodes.into_values().collect::<Vec<_>>();
    sorted_nodes.sort_by(|a, b| b.count.cmp(&a.count));

    VisData {
        edges: sorted_edges,
        nodes: sorted_nodes,
        aliases,
    }
}

fn proc_alias(sol: QuerySolution, aliases: &mut HashMap<String, String>) {
    let mut alias = None;
    if let oxigraph::model::Term::NamedNode(n) = sol.get("alias").unwrap() {
        alias = get_fragment(n.clone());
    }
    let mut namespace = None;
    if let oxigraph::model::Term::NamedNode(n) = sol.get("namespace").unwrap() {
        namespace = Some(n.as_str().to_string());
    }

    if let (Some(alias_name), Some(namespace_name)) = (alias, namespace) {
        aliases.insert(alias_name, namespace_name);
    }
}

fn proc_norm_triples(
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
                node_type: match src_name.as_ref() {
                    "UNKNOWN" => VisNodeType::Unknown,
                    "BLANK" => VisNodeType::Blank,
                    _ => VisNodeType::Namespace,
                },
            })
            .count += occurs_val.parse::<usize>().unwrap();
        nodes
            .entry(tgt_name.clone())
            .or_insert_with(|| VisNode {
                name: tgt_name.clone(),
                count: 0,
                node_type: match tgt_name.as_ref() {
                    "UNKNOWN" => VisNodeType::Unknown,
                    "BLANK" => VisNodeType::Blank,
                    _ => VisNodeType::Namespace,
                },
            })
            .count += occurs_val.parse::<usize>().unwrap();

        let key = sort_pair(src_name.clone(), tgt_name.clone());
        let colliding = edges.entry(key.clone()).or_default();
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
    store.load_from_reader(RdfFormat::Turtle, stream).unwrap();
    store
}

fn query_norm_triples(store: Store) -> Result<QueryResults<'static>, QueryEvaluationError> {
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

    SparqlEvaluator::new()
        .parse_query(q)?
        .on_store(&store)
        .execute()
}

fn query_aliases(store: Store) -> Result<QueryResults<'static>, QueryEvaluationError> {
    let q = r#"
        BASE <http://andrefs.com/graph-summ/v1>
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX afsgs: <http://andrefs.com/graph-summ/v1#>
    
        SELECT ?alias ?namespace WHERE {
            ?alias <#namespacePrefix> ?namespace  .

        }
        "#;

    SparqlEvaluator::new()
        .parse_query(q)?
        .on_store(&store)
        .execute()
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
        .truncate(true)
        .open(file_path.clone())
        .unwrap();

    writeln!(fd, "{}", serde_json::to_string_pretty(&data).unwrap()).unwrap();
}

pub fn render_vis(data: &VisData, outf: &str) -> PathBuf {
    let render_dir = Path::new(".").join("chilon-viz");
    let tera = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("data", &data);

    let data_path = render_dir.join("src").join("data").join("raw-data.ts");

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
        .current_dir(render_dir.clone())
        .output()
        .expect("Failed to execute vite build");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    let src = render_dir.join("dist");
    let dst = Path::new(outf).join("dist");
    info!(
        "Copying {} to {}",
        src.to_string_lossy(),
        dst.to_string_lossy()
    );
    //rename(src, dst).unwrap();
    if dst.is_dir() {
        warn!(
            "Folder {} exists, removing before copying",
            dst.to_string_lossy()
        );
        remove_dir_all(dst).unwrap();
    }
    copy(src, outf, &Default::default()).unwrap();

    render_dir
}

pub fn vis_dev_server(dir: PathBuf) {
    info!("Opening dev env");
    let _output = Command::new("sh")
        .arg("-c")
        .arg("yarn dev")
        .current_dir(dir)
        .output()
        .expect("Failed to execute vite build");

    //io::stdout().write_all(&output.stdout).unwrap();
    //io::stderr().write_all(&output.stderr).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_pair_already_sorted() {
        assert_eq!(sort_pair("a".into(), "b".into()), ("a".into(), "b".into()));
    }

    #[test]
    fn sort_pair_reversed() {
        assert_eq!(sort_pair("b".into(), "a".into()), ("a".into(), "b".into()));
    }

    #[test]
    fn sort_pair_equal() {
        assert_eq!(
            sort_pair("same".into(), "same".into()),
            ("same".into(), "same".into())
        );
    }

    #[test]
    fn dump_json_writes_file() {
        let data = VisData {
            nodes: vec![VisNode {
                name: "ex".into(),
                count: 42,
                node_type: VisNodeType::Namespace,
            }],
            edges: vec![VisEdge {
                source: "ex".into(),
                target: "nt".into(),
                count: 10,
                label: "p".into(),
                is_datatype: false,
                link_num: 1,
            }],
            aliases: [("ex".into(), "http://ex.org/".into())].into(),
        };
        let dir = tempfile::TempDir::new().unwrap();
        dump_json(&data, dir.path().to_str().unwrap());

        let json_path = dir.path().join("vis-data.json");
        assert!(json_path.exists());

        let content = std::fs::read_to_string(json_path).unwrap();
        assert!(content.contains("ex"));
        assert!(content.contains("42"));
        assert!(content.contains("http://ex.org/"));
    }
}
