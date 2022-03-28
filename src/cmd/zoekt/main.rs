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
use zoekt::query;

use clap::{App, Arg};
use env_logger;
use log::{error, info};
use std::env;

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
			fmt.Printf("%s%s\n", r, f.FileName)
			continue
		}

		for _, m := range f.LineMatches {
			fmt.Printf("%s%s:%d:%s\n", r, f.FileName, m.LineNumber, m.Line)
		}
	}
}*/

//func loadShard(fn string, verbose bool) (zoekt.Searcher, error) {
fn load_shard(verbose: bool) -> Result<String, std::num::ParseIntError> {
	/*f, err := os.Open(fn)
	if err != nil {
		return nil, err
	}

	iFile, err := zoekt.NewIndexFile(f)
	if err != nil {
		return nil, err
	}*/
	#[cfg(target_family = "unix")]
	zoekt::indexfile_unix::new_index_file();

	/*s, err := zoekt.NewSearcher(iFile)
	if err != nil {
		iFile.Close()
		return nil, fmt.Errorf("NewSearcher(%s): %v", fn, err)
	}*/

	if verbose {
		/*repo, index, err := zoekt.ReadMetadata(iFile)
		if err != nil {
			iFile.Close()
			return nil, fmt.Errorf("ReadMetadata(%s): %v", fn, err)
		}
		log.Printf("repo metadata: %#v", repo)
		log.Printf("index metadata: %#v", index)*/
	}

	//return s, nil
	Ok(String::from("zoekt.Searcher"))
}

fn main() {
//	let args: Vec<_> = env::args().collect();
//	let s: String = args[0];
	env::set_var("RUST_LOG", "info");
	env_logger::init();
	let matches = App::new("zoekt")
	.version("0.1.0")
	.arg(Arg::from_usage("-s, --shard [shard] 'Search in a specific shard'"))
	//index := flag.String("index_dir",
	//filepath.Join(os.Getenv("HOME"), ".zoekt"), "search for index files in `directory`")
	.arg(Arg::from_usage("--cpu_profile 'Write cpu profile to `file`"))
	//profileTime := flag.Duration("profile_time", time.Second, "run this long to gather stats.")*/
	.arg(Arg::from_usage("-v, --verbose 'Print some background data'"))
	.arg(Arg::from_usage("-r, --repo 'Print the repo before the file name'"))
	.arg(Arg::from_usage("-l, --list 'Print matching filenames only'"))
	.arg(Arg::from_usage("<QUERY> 'for example\n zoekt \'byte file:java -file:test\''"))
	.get_matches();
	
	let _cpu_profile = matches.is_present("cpu_profile");
	let verbose = matches.is_present("verbose");
	let _with_repo = matches.is_present("repo");
	let _list = matches.is_present("list");

	let pat: Option<&str>  = matches.value_of("QUERY");

	/*var searcher zoekt.Searcher
	var err error
	if *shard != "" {
		searcher, err = loadShard(*shard, *verbose)
	} else {
		searcher, err = shards.NewDirectorySearcher(*index)
	}

	if err != nil {
		log.Fatal(err)
	}*/
	let _searcher = zoekt::api::Searcher::default();
	if let Some(_shard) = matches.value_of("shard") {
		//searcher, err = loadShard(*shard, *verbose)
		match load_shard(verbose) {
			Ok(_s) => {println!("OK")}
			Err(e) => {error!("{}", e)}
		}
	} else {
		//searcher, err = shards.NewDirectorySearcher(*index)
	}

	/*query, err := query.Parse(pat)
	if err != nil {
		log.Fatal(err)
	}
	if *verbose {
		log.Println("query:", query)
	}*/
	let query = query::parse::parse(pat);
	if let Err(e) = query {
		error!("{}", e);
	}	
	if verbose {
		info!("query: {}", "query");
	}

/*	var sOpts zoekt.SearchOptions
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
