use anyhow::Result;
use clap::{parser, Parser};
use jwalk::rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use path_absolutize::Absolutize;
use std::path::{Component, Components, Path, PathBuf};
use strum_macros::Display;
use thiserror::Error;
use tracing::{error, info, span, trace, warn, Level};
use tracing_futures::Instrument;

// mkdir PATH/TO/DIR || path,list,for,multiple,dirs

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
  /// List all files in a directory
  #[clap(value_parser)]
  path: Vec<PathBuf>,
}

#[derive(Error, Debug)]
enum PathError {
  #[error("Path does not exist")]
  Exists,
  #[error("Path is a file")]
  NotDir,
  #[error("Path is not readable")]
  NotReadable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathTree {
  root: PathBuf,
  children: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PathNode {
  path: PathBuf,
  children: Vec<PathBuf>,
  exists: bool,
}

fn to_valid_path<T: Into<PathBuf>>(path: T) -> PathBuf {
  let path = path.into();
  let mut path = path.absolutize().unwrap();

  if !path.exists() {
    warn!("Path does not exist: {:?}", path);
  }

  return path.into();
}

fn path_to_components() {}

fn parse_path(path: &PathBuf) -> Result<(), PathError> {
  // let path_span = span!(Level::INFO, "[parsing path]");
  // let _enter = path_span.enter();

  if path.exists() {
    return Err(PathError::Exists);
  }
  if path.is_file() {
    return Err(PathError::NotDir);
  }

  let components = path.components().into_iter().collect::<Vec<_>>();
  info!(components = ?components);

  let mut trailing_path = PathBuf::new();
  for c in components {
    match c {
      Component::RootDir => {
        info!("[root][{:?}]", c);
        trailing_path.push(c);
      }
      Component::Normal(c) => {
        info!("[normal][{:?}]", c);
        trailing_path.push(c);
      }
      Component::CurDir => {
        info!("[current][{:?}]", c);
      }
      Component::Prefix(c) => {
        info!("[prefix][{:?}]", c);
      }
      Component::ParentDir => {
        info!("[parent][{:?}]", c);
        trailing_path.pop();
      }
      _ => {
        info!("c = {:?}", c);
      }
    }

    info!("[{:?}]", trailing_path);
  }
  info!("trailing_path = {:?}", trailing_path);
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::FmtSubscriber::builder()
    .with_ansi(true)
    .with_level(true)
    .event_format(tracing_subscriber::fmt::format().compact())
    .without_time()
    .init();

  let command = Cli::parse();
  let path = command.path;
  for p in path {
    match parse_path(&p) {
      Ok(()) => {
        info!("{}", p.display());
      }
      Err(e) => {
        error!("{}", e);
      }
    };
  }

  Ok(())
}
