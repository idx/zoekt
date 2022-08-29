// Copyright 2016 Google Inc. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//package main
//use zoekt::query;

use clap::Parser;
use env_logger;
use log::error;
use std::env;
use std::path::Path;
use std::path::PathBuf;
//use std::process;
//use std::time::Duration;

/*import (
    "context"
    "flag"
    "fmt"
    "log"
    "os"
    "path/filepath"
    "runtime/pprof"
    "time"

    "github.com/google/zoekt"
    "github.com/google/zoekt/query"
    "github.com/google/zoekt/shards"
)

func displayMatches(files []zoekt.FileMatch, pat string, withRepo bool, list bool) {
    for _, f := range files {
        r := ""
        if withRepo {
            r = f.Repository + "/"
        }
        if list {
            fmt.Printf("%s%s\n", r, f.FileName)N
            continue
        }

        for _, m := range f.LineMatches {
            fmt.Printf("%s%s:%d:%s\n", r, f.FileName, m.LineNumber, m.Line)
        }
    }
}*/

//func loadShard(fn string, verbose bool) (zoekt.Searcher, error) {
fn load_shard(r#fn: &str, verbose: bool) -> Result<String, std::num::ParseIntError> {
    /*f, err := os.Open(fn)
    if err != nil {
        return nil, err
    }

    iFile, err := zoekt.NewIndexFile(f)
    if err != nil {
        return nil, err
    }

    s, err := zoekt.NewSearcher(iFile)
    if err != nil {
        iFile.Close()
        return nil, fmt.Errorf("NewSearcher(%s): %v", fn, err)
    }

    if verbose {
        repo, index, err := zoekt.ReadMetadata(iFile)
        if err != nil {
            iFile.Close()
            return nil, fmt.Errorf("ReadMetadata(%s): %v", fn, err)
        }
        log.Printf("repo metadata: %#v", repo)
        log.Printf("index metadata: %#v", index)
    }

    return s, nil*/

    let f = Path::new(r#fn);
    #[cfg(target_family = "unix")]
    let _i_file = match zoekt::indexfile_unix::new_index_file(f) {
        Ok(_t) => {}
        Err(e) => {
            error!("{}", e)
        }
    };

    if verbose {
        /*repo, index, err := zoekt.ReadMetadata(iFile)
        if err != nil {
            iFile.Close()
            return nil, fmt.Errorf("ReadMetadata(%s): %v", fn, err)
        }
        log.Printf("repo metadata: %#v", repo)
        log.Printf("index metadata: %#v", index)*/
    }

    Ok(String::from("zoekt.Searcher"))
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Search in a specific shardSearch in a specific shard
    #[clap(long, value_name = "string")]
    shard: Option<String>,

    // index := flag.String("index_dir", filepath.Join(os.Getenv("HOME"), ".zoekt"), "search for index files in `directory`")
    /// search for index files in directory (default "~/.zoekt")
    #[clap(long, value_name = "file")]
    index_dir: Option<PathBuf>,
    
    // 	profileTime := flag.Duration("profile_time", time.Second, "run this long to gather stats.")
    /// write cpu profile to file
    #[clap(long, value_name = "file")]
    cpu_profile: Option<String>,

    /// run this long to gather stats. (default 1s)
    #[clap(long, value_name = "duration")]
    profile_time: Option<String>,

    /// Print some background data
    #[clap(short, long)]
    verbose: bool,

    /// Print the repo before the file name
    #[clap(short = 'r')]
    repo: bool,

    /// Print matching filenames only
    #[clap(short)]
    list: bool,

    /// for example
    /// 
    /// zoekt 'byte file:java -file:test'
    #[clap()]
    query: String,
    
}

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    let cli = Cli::parse();

    /*let profile_time = if let Some(time) = matches.value_of("duration") {
        Duration::from_secs(time.parse().unwrap())
    } else {
        Duration::from_secs(0)
    };*/
    /*let mut _index: PathBuf = if let Some(index) = matches.value_of("index_dir") {
        PathBuf::from(index)
    } else {
        PathBuf::from(dirs::home_dir().unwrap().join(".zoekt"))
    };*/

    /*let pat: Option<&str> = matches.value_of("QUERY");*/

    /*var searcher zoekt.Searcher
	var err error
	if *shard != "" {
		searcher, err = loadShard(*shard, *verbose)
	} else {
		searcher, err = shards.NewDirectorySearcher(*index)
	}

	if err != nil {
		log.Fatal(err)
	}

	query, err := query.Parse(pat)
	if err != nil {
		log.Fatal(err)
	}
	if *verbose {
		log.Println("query:", query)
	}*/
    let _searcher = zoekt::api::Searcher::default();
    if let Some(shard) = cli.shard {
        //searcher, err = loadShard(*shard, *verbose)*/
		println!{"{}", shard}
    	let _ret = load_shard("shard", cli.verbose);
    /*Ok(_s) => {
                println!("OK")[]
            }
            Err(e) => {
                error!("{}", e);
                process::exit(1)
            }
        }*/
    } else {
        //searcher, err = shards.NewDirectorySearcher(*index)
    }

    /*let _query = match query::parse::parse(pat) {
        Ok(v) => {
            if verbose {
                info!("query: {}", v);
            }
        }
        Err(e) => {
            error!("{}", e);
            process::exit(1)
        }
    };*/
    if cli.verbose {
        println!{"query:{}", cli.query};
    }

    /*var sOpts zoekt.SearchOptions
	sres, err := searcher.Search(context.Background(), query, &sOpts)
	if *cpuProfile != "" {
		// If profiling, do it another time so we measure with
		// warm caches.
		f, err := os.Create(*cpuProfile)
		if err != nil {
			log.Fatal(err)
		}
		defer f.Close()
		if *verbose {
			log.Println("Displaying matches...")
		}

		t := time.Now()
		pprof.StartCPUProfile(f)
		for {
			sres, _ = searcher.Search(context.Background(), query, &sOpts)
			if time.Since(t) > *profileTime {
				break
			}
		}
		pprof.StopCPUProfile()
	}

	if err != nil {
		log.Fatal(err)
	}

	displayMatches(sres.Files, pat, *withRepo, *list)
	if *verbose {
		log.Printf("stats: %#v", sres.Stats)
	}*/
}
