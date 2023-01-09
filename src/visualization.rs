use log::{debug, info};
use oxigraph::{io::GraphFormat, model::GraphName, sparql::QueryResults, store::Store};

use rio_turtle::TurtleParser;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};
use std::{io::Write, path::Path};
use url::Url;

use tera::{Context, Tera};

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
    links: Vec<VisEdge>,
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
}

pub fn build_data(outf: &str) -> VisData {
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
        "#;

    let qres = store.query(q);

    if let Ok(QueryResults::Solutions(mut sols)) = qres {
        let mut id_count = 0;

        for s in sols {
            if let Ok(sol) = s {
                let mut src = None;
                if let oxigraph::model::Term::NamedNode(n) = sol.get("src").unwrap() {
                    src = Some(
                        Url::parse(&n.clone().into_string())
                            .unwrap()
                            .fragment()
                            .unwrap()
                            .to_string(),
                    );
                }

                let mut tgt = None;
                if let oxigraph::model::Term::NamedNode(n) = sol.get("tgt").unwrap() {
                    tgt = Some(
                        Url::parse(&n.clone().into_string())
                            .unwrap()
                            .fragment()
                            .unwrap()
                            .to_string(),
                    );
                }

                let mut label = None;
                if let oxigraph::model::Term::NamedNode(n) = sol.get("label").unwrap() {
                    label = Some(
                        Url::parse(&n.clone().into_string())
                            .unwrap()
                            .fragment()
                            .unwrap()
                            .to_string(),
                    );
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
                    let src_id = if nodes2ids.contains_key(&src.clone().unwrap()) {
                        *nodes2ids.get(&src.clone().unwrap()).unwrap()
                    } else {
                        nodes2ids.insert(src.clone().unwrap(), id_count);
                        id_count += 1;
                        id_count
                    };
                    let tgt_id = if nodes2ids.contains_key(&tgt.clone().unwrap()) {
                        *nodes2ids.get(&tgt.clone().unwrap()).unwrap()
                    } else {
                        nodes2ids.insert(tgt.clone().unwrap(), id_count);
                        id_count += 1;
                        id_count
                    };

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

                    edges.push(VisEdge {
                        source: src_id,
                        target: tgt_id,
                        count: occurs.unwrap().parse().unwrap(),
                        label: label.clone().unwrap(),
                    });
                }
            }
        }
    }

    let data = VisData {
        links: edges,
        nodes: nodes.into_values().collect::<Vec<_>>(),
    };

    return data;
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
    let tera = Tera::new("templates/**/*").unwrap();
    let mut ctx = Context::new();
    ctx.insert("data", &data);

    let js_path = Path::new(".").join(outf).join("script.js");
    let html_path = Path::new(".").join(outf).join("index.html");
    let css_path = Path::new(".").join(outf).join("style.css");

    info!("Rendering visualization to {}", html_path.to_string_lossy());

    let js_fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(js_path.clone())
        .unwrap();

    let html_fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(html_path.clone())
        .unwrap();

    let css_fd = OpenOptions::new()
        .write(true)
        .create(true)
        .open(css_path.clone())
        .unwrap();

    tera.render_to("script.js", &ctx, js_fd).unwrap();
    tera.render_to("index.html", &Context::new(), html_fd)
        .unwrap();
    tera.render_to("style.css", &Context::new(), css_fd)
        .unwrap();
}
