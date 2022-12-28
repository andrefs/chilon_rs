use log::{debug, error};
use oxigraph::{
    io::GraphFormat,
    model::{GraphName, Quad},
    sparql::QueryResults,
    store::Store,
};
use rio_api::{
    model::{NamedNode, Term},
    parser::TriplesParser,
};
use rio_turtle::{TurtleError, TurtleParser};
use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};
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

pub struct VisNode {
    id: String,
    name: String,
    count: usize,
}

pub struct VisEdge {
    source: Option<String>,
    target: Option<String>,
    count: Option<usize>,
    label: Option<String>,
}

pub fn dump_json(path: String) {
    let file =
        File::open(path.clone()).unwrap_or_else(|e| panic!("Could not open file {}: {e}", path));
    let buf_reader = BufReader::new(file);
    debug!("extracting {:?}", path);
    let stream = BufReader::new(buf_reader);

    let store = Store::new().unwrap();

    let nodes = BTreeMap::<String, VisNode>::new();
    let edges = BTreeMap::<String, VisEdge>::new();

    let rdf_type = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
    let groups_link = "http://andrefs.com/graph-summ/GroupsLink";

    store
        .bulk_loader()
        .load_graph(stream, GraphFormat::Turtle, &GraphName::DefaultGraph, None)
        .unwrap();

    let q = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#> 
        PREFIX afsgs: <http://andrefs.com/graph-summ/v1#>
    
        SELECT ?id ?src ?label ?tgt ?occurs WHERE {
            ?id rdf:type afsgs:GroupsLink .
            ?id rdf:subject ?src .
            ?id rdf:predicate ?label .
            ?id rdf:object ?tgt .
            ?id afsgs:occurrences ?occurs .
        }
        "#;

    let qres = store.query(q);

    if let Ok(QueryResults::Solutions(mut sols)) = qres {
        for s in sols {
            if let Ok(sol) = s {
                println!(
                    //"{} {} {} {} {} ",
                    "{} {} {} {} {}",
                    sol.get("id").unwrap(),
                    sol.get("src").unwrap(),
                    sol.get("tgt").unwrap(),
                    sol.get("label").unwrap(),
                    sol.get("occurs").unwrap()
                )
            }
        }
    }
}
