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

//use clap::Parser;
use clap::{App, Arg};
use env_logger;
use log::{error, info};
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use std::fs::File;
use std::time::Duration;
//use std::thread;
use pprof;

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

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let matches = App::new("zoekt")
        .version("0.1.0")
        .arg(Arg::from_usage(
            "-s, --shard [shard] 'Search in a specific shard'",
        ))
        .arg(
            Arg::from_usage("--index_dir [index_dir] 'search for index files in `directory`")
                .default_value(&(dirs::home_dir().unwrap().display().to_string() + "/.zoekt")),
        )
        .arg(Arg::from_usage(
            "--cpu_profile [file] 'Write cpu profile to file`",
        ))
        .arg(
            Arg::from_usage("--profile_time [duration] 'run this long to gather stats.'")
                .default_value("time.Second"),
        )
        .arg(Arg::from_usage(
            "-v, --verbose 'Print some background data'",
        ))
        .arg(Arg::from_usage(
            "-r, --repo 'Print the repo before the file name'",
        ))
        .arg(Arg::from_usage(
            "-l, --list 'Print matching filenames only'",
        ))
        .arg(Arg::from_usage(
            "<QUERY> 'for example\n zoekt \'byte file:java -file:test\''",
        ))
        .get_matches();

    let _profile_time = if let Some(time) = matches.value_of("duration") {
        Duration::from_secs(time.parse().unwrap())
    } else {
        Duration::from_secs(0)
    };
    let mut _index: PathBuf = if let Some(index) = matches.value_of("index_dir") {
        PathBuf::from(index)
    } else {
        PathBuf::from(dirs::home_dir().unwrap().join(".zoekt"))
    };

    let verbose = matches.is_present("verbose");
    let _with_repo = matches.is_present("repo");
    let _list = matches.is_present("list");

    let pat: Option<&str> = matches.value_of("QUERY");

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
    /*let _searcher = zoekt::api::Searcher::default();
    if let Some(shard) = cli.shard {
        //searcher, err = loadShard(*shard, *verbose)*/
    //println!{"{}", shard}
    let _ret = load_shard("shard", verbose);
    /*Ok(_s) => {
            println!("OK")[]
        }
        Err(e) => {
            error!("{}", e);
            process::exit(1)
        }
    }*/
    //} else {
    //searcher, err = shards.NewDirectorySearcher(*index)
    //}

    let _query = match query::parse::parse(pat) {
        Ok(v) => {
            if verbose {
                info!("query: {}", v);
            }
        }
        Err(e) => {
            error!("{}", e);
            process::exit(1)
        }
    };

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
    if let Some(cpu_profile) = matches.value_of("cpu_profile") {
        let guard = pprof::ProfilerGuardBuilder::default().frequency(1000).blocklist(&["libc", "libgcc", "pthread", "vdso"]).build().unwrap();

        if let Ok(report) = guard.report().build() {
            let file = match File::create(cpu_profile) {
                Ok(file) => file,
                Err(_) => {
                    process::exit(1)
                },
           };
            let mut options = pprof::flamegraph::Options::default();
            options.image_width = Some(2500);
            report.flamegraph_with_options(file, &mut options).unwrap();
        }
    }

    if verbose {

    }
}
