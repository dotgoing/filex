use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use std::fs;
use std::fs::DirEntry;

mod lib;
use lib::*;

/// A tool to delete the oldest files
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "sean")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Delete(Delete),
    Show(Show),
}

/// Delete files according the given args
#[derive(Parser, Debug)]
struct Delete {
    /// Specify the path within which files should be deleted
    #[clap(short, long)]
    path: Option<String>,
    /// The prefix of file name
    #[clap(short = 's', long)]
    prefix: Option<String>,
    /// How many files can be keep
    #[clap(short, long)]
    num_keep: usize,
    /// How long ago files can be keep. -d 5 means 5 days ago
    #[clap(short, long)]
    days_to_keep: Option<DaysSec>,
    /// How much total size files can be keep. k/K m/M g/G. -m 2g3M4k means 2G+3M+4K
    #[clap(short, long,parse(try_from_str=parse_max_size))]
    max_size: Option<FileSize>,
    /// How much percent available disk space should be keep?
    #[clap(short, long,parse(try_from_str=parse_disk_percent))]
    free_disk: Option<DiskFreePercent>,
}

/// Show files which can be deleted according the given args
#[derive(Parser, Debug)]
struct Show {
    /// Specify the path within which files should be deleted
    #[clap(short, long)]
    path: Option<String>,
    /// The prefix of file name
    #[clap(short = 's', long)]
    prefix: Option<String>,
    /// How many files can be keep
    #[clap(short, long)]
    num_keep: usize,
    /// How long ago files can be keep. -d 5 means 5 days ago
    #[clap(short, long)]
    days_to_keep: Option<DaysSec>,
    /// How much total size files can be keep. k/K m/M g/G. -m 2g3M4k means 2G+3M+4K
    #[clap(short, long,parse(try_from_str=parse_max_size))]
    max_size: Option<FileSize>,
    /// How much percent available disk space should be keep?
    #[clap(short, long,parse(try_from_str=parse_disk_percent))]
    free_disk: Option<DiskFreePercent>,
}

fn parse_max_size(s: &str) -> Result<FileSize> {
    s.parse()
}

fn parse_disk_percent(s: &str) -> Result<DiskFreePercent> {
    s.parse()
}

/// 程序的入口函数，因为在 http 请求时我们使用了异步处理，所以这里引入 tokio
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Delete(ref args) => list_file(
            &args.path,
            &args.prefix,
            args.num_keep,
            &args.max_size,
            &args.days_to_keep,
            &args.free_disk,
            true,
        ),
        SubCommand::Show(ref args) => list_file(
            &args.path,
            &args.prefix,
            args.num_keep,
            &args.max_size,
            &args.days_to_keep,
            &args.free_disk,
            false,
        ),
    };
    Ok(())
}

fn list_file(
    path: &Option<String>,
    prefix: &Option<String>,
    num_keep: usize,
    total_size: &Option<FileSize>,
    days: &Option<DaysSec>,
    free_disk: &Option<DiskFreePercent>,
    delete: bool,
) {
    let path = match path {
        Some(it) => it,
        None => ".",
    };
    let prefix = match prefix {
        Some(it) => it,
        None => "",
    };
    let paths = fs::read_dir(path).unwrap();
    let paths: Vec<DirEntry> = paths
        .into_iter()
        .filter(|it| it.is_ok())
        .map(|it| it.unwrap())
        .collect();
    let mut paths: Vec<FileInfo> = paths
        .into_iter()
        .map(lib::parse_file)
        .filter(|it| it.is_ok())
        .map(|it| it.unwrap())
        .filter(|it| {
            it.entry
                .file_name()
                .and_then(|it| it.to_str())
                .map(|s| s.starts_with(&prefix))
                .unwrap_or(false)
        })
        .collect();

    count_file_size(&mut paths);

    let disk_info = lib::get_disk_info().ok();
    let mut files: Vec<&FileInfo> = vec![];
    for (index, file_info) in paths.iter().enumerate() {
        let log_too_many = index >= num_keep;
        let size_too_big = total_size
            .map(|it| it.size < file_info.acc_len)
            .unwrap_or(false);
        let log_too_old = days
            .map(|it| it.day_sec > file_info.elapsed)
            .unwrap_or(false);

        let free_disk_too_small = match (free_disk, &disk_info) {
            (Some(DiskFreePercent { percent }), Some(DiskInfo { total, available })) => {
                let need_to_keep = total * percent / 100;
                let new_available = available + file_info.reverse_acc_len;
                new_available > need_to_keep
            }
            _ => false,
        };
        if log_too_many || size_too_big || log_too_old || free_disk_too_small {
            files.push(&file_info);
        }
    }
    files.reverse();
    for file in files {
        if delete {
            match std::fs::remove_file(&file.entry) {
                Ok(()) => {
                    let now = Utc::now().format("%a %b %e %T %Y");
                    println!("{} delete {:?}", now, file.entry);
                }
                Err(err) => println!("fail to delete file {:?}. because {}", file.entry, err),
            }
        } else {
            println!("{:?}", file.entry);
        }
    }
}

fn count_file_size(paths: &mut Vec<FileInfo>) {
    paths.sort_by(|a, b| a.elapsed.cmp(&b.elapsed));
    let mut reverse_acc_size = 0;
    for file_info in paths.iter_mut() {
        reverse_acc_size += file_info.len;
        file_info.reverse_acc_len = reverse_acc_size;
    }
    paths.reverse();
    let mut acc_size = 0;
    for file_info in paths.iter_mut() {
        acc_size += file_info.len;
        file_info.acc_len = acc_size;
    }
    paths.reverse();
}
