// region:    --- Modules

mod argc;
mod error;

pub use self::error::{Error, Result};

use chrono::{DateTime, Utc};
use argc::argc_app;
use clap::ArgMatches;
use globset::{Glob, GlobSetBuilder};
use std::collections::{BinaryHeap, HashMap};
use std::path::PathBuf;
use std::time::SystemTime;
use walkdir::WalkDir;
use std::cmp::Ordering;

// Current directory
const DIR: &str = "./";
const NUM_FILES: usize = 5;

fn main() {

	let argc = argc_app().get_matches();

	// Transforms ArgMatches to Options struct
	let options = match Options::from_argc(argc) {
		Ok(options) => options,
		Err(ex) => {
			println!("ERROR parsing input {}", ex);
			return;
		}
	};

	match exec(options) {
		Ok(_) => (),
		Err(ex) => {
			println!("ERROR - {}", ex);
		}
	}
}

#[derive(PartialEq, Eq)]
struct Entry {
	path: PathBuf,
	last_accessed_time: SystemTime,
	recent: bool
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
		if self.recent{
			other.last_accessed_time.cmp(&self.last_accessed_time)
		}else{
			self.last_accessed_time.cmp(&other.last_accessed_time)
		}
    }
}

struct Options {
	nums: usize,
	glob: Option<Vec<String>>,
	no_ext: bool,
	recent: bool
}


impl Options {
	fn from_argc(argc: ArgMatches) -> Result<Options> {
		// -- nums
		let nums: usize = match argc.get_one::<String>("nums") {
			None => NUM_FILES,
			Some(nums) => nums
				.parse::<usize>()
				.map_err(|_| Error::InvalidNumberOfFiles(nums.to_string()))?,
		};

		// -- glob
		let glob = argc
			.get_one::<String>("glob")
			.map(|glob| glob.split(',').map(|s| s.to_string()).collect::<Vec<String>>());

		// -- by_ext
		let no_ext = argc.get_flag("no-ext");

		let recent = !argc.get_flag("oldest");
		
		Ok(Options { nums, glob, no_ext, recent })
	}
}
/// Transforms system time to a formated string
fn time_to_string(time:&SystemTime) -> String {
	let duration_since_epoch = time.duration_since(SystemTime::UNIX_EPOCH)
		.expect("Time went backwards");

	let datetime: DateTime<Utc> = DateTime::<Utc>::from(SystemTime::UNIX_EPOCH + duration_since_epoch);
 	datetime.format("%Y-%m-%d   %H:%M:%S").to_string()
}

fn exec(options: Options) -> Result<()> {
	let mut total_files: u32 = 0;

	let mut pq = BinaryHeap::<Entry>::new();

	let glob = options
		.glob
		.map(|vs| {
			let mut builder = GlobSetBuilder::new();
			for v in vs {
				builder.add(Glob::new(&v)?);
			}
			builder.build()
		})
		.transpose()?;
	
	let mut by_ext: Option<HashMap<String, SystemTime>> = if !options.no_ext { 
														Some(HashMap::new()) 
													} else {
														None 
													};

	let entries = WalkDir::new(DIR)
		.into_iter()
		.filter_map(|e| e.ok())
		.filter(|e| {
			
			if let Some(glob) = &glob {
				glob.is_match(e.path()) 
			} else {
				true
			}
		});

	for entry in entries {
		let path = entry.path();
		if path.is_file() && !entry.path_is_symlink() {
			total_files += 1; 
			let last_accessed = entry.metadata()?.accessed();

			if let Some(by_ext) = &mut by_ext { 
				if let Some(ext) = path.extension() {
					let ext = ext.to_string_lossy().to_string();
					if let Ok(last_accessed) = last_accessed {
						by_ext.entry(ext)
							.and_modify(|prev| {
								if *prev < last_accessed {
									*prev = last_accessed;
								}
							})
							.or_insert(last_accessed);
					
						pq.push(Entry {
							path: entry.path().to_path_buf(),
							last_accessed_time: last_accessed,
							recent: options.recent
						});
					
						if pq.len() > options.nums {
							pq.pop();
						}
					}
				}
			}
		}
	}

	println!(
		"== Summary\nNumber of files {}",
		total_files
	);

	let order = if options.recent{
		"most recently"
	}else{
		"oldest"
	};

	if let Some(mut by_ext) = by_ext {
		println!("\n== Top {} {} accesed files by extension", options.nums,order);
		let mut by_ext: Vec<(String, SystemTime)> = by_ext.drain().collect();
		by_ext.sort_by(|a, b|{
			if options.recent{
				b.1.cmp(&a.1)
			}else{
				a.1.cmp(&b.1)
			}
		});
		
		for (i, (ext, time)) in by_ext.iter().enumerate() {
			if i < options.nums {
				println!("{:?} - {}", time_to_string(&time), ext);
			}
		}
	}

	println!("\n== Top {} {} accessed files", options.nums,order);

	for Entry { path, last_accessed_time,.. } in pq.iter() {
		println!("{:?} ---  {}", time_to_string(&last_accessed_time) , path.to_string_lossy());
	}
	Ok(())
}