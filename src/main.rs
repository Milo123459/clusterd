mod worker;
mod cluster;
mod spawn;

use std::sync::mpsc::{channel, Sender};
use std::io::Write;
use tracing_subscriber::{fmt::MakeWriter, FmtSubscriber, fmt::format::FmtSpan};
use crate::cluster::Cluster;
use crate::worker::Worker;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE).finish();
    // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
    // will be written to stdout.
    // completes the builder.
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let cluster = Cluster {
        binary: "ham".to_string(),
        name: "hamqq".to_string(),
    };

    let worker = Worker::new(cluster, vec!["yeas".to_string()]).unwrap();
    worker.start();
}