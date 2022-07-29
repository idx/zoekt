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
use clap::Parser;
use std::path::PathBuf;

/*import (
    "flag"
    "fmt"
    "io/ioutil"
    "log"
    "os"
    "path/filepath"
    "runtime/pprof"
    "strings"

    "github.com/google/zoekt"
    "github.com/google/zoekt/build"
    "github.com/google/zoekt/cmd"
    "go.uber.org/automaxprocs/maxprocs"
)*/

/*type fileInfo struct {
    name string
    size int64
}*/
pub struct FileInfo {
    pub name: String,
    pub size: i64,
}

/*type fileAggregator struct {
    ignoreDirs map[string]struct{}
    sizeMax    int64
    sink       chan fileInfo
}*/
pub struct FileAggregator {
    //ignoreDirs map[string]struct{}
    pub size_max: i64, //sink       chan fileInfo
}

/*func (a *fileAggregator) add(path string, info os.FileInfo, err error) error {
    if err != nil {
        return err
    }

    if info.IsDir() {
        base := filepath.Base(path)
        if _, ok := a.ignoreDirs[base]; ok {
            return filepath.SkipDir
        }
    }

    if info.Mode().IsRegular() {
        a.sink <- fileInfo{path, info.Size()}
    }
    return nil
}*/

#[derive(Parser)]
//#[clap(author, version, about, long_about = None)]
#[clap(about, long_about = None)]
struct Cli {
    /// args
    #[clap(short, long)]
    args: Vec<String>,

     /// write cpu profile to file
    #[clap(long = "cpu_profile", value_name = "string")]
    cpu_profile: Option<PathBuf>,

    /// comma separated list of directories to ignore.
    #[clap(long = "ignore_dirs", value_name = "string")]
    ignore_dirs: Option<PathBuf>,
}

//func main() {
fn main() {
    /*	cpuProfile := flag.String("cpu_profile", "", "write cpu profile to file")
    ignoreDirs := flag.String("ignore_dirs", ".git,.hg,.svn", "comma separated list of directories to ignore.")
    flag.Parse()*/
    let cli = Cli::parse();

    // Tune GOMAXPROCS to match Linux container CPU quota.
    /*maxprocs.Set()

    opts := cmd.OptionsFromFlags()
    if *cpuProfile != "" {
        f, err := os.Create(*cpuProfile)
        if err != nil {
            log.Fatal(err)
        }
        pprof.StartCPUProfile(f)
        defer pprof.StopCPUProfile()
    }*/
    if let Some(cpu_profile) = cli.cpu_profile.as_deref() {
        println!("Value for cpu_profile: {}", cpu_profile.display());
    }

    /*ignoreDirMap := map[string]struct{}{}
    if *ignoreDirs != "" {
        dirs := strings.Split(*ignoreDirs, ",")
        for _, d := range dirs {
            d = strings.TrimSpace(d)
            if d != "" {
                ignoreDirMap[d] = struct{}{}
            }
        }
    }
    for _, arg := range flag.Args() {
        opts.RepositoryDescription.Source = arg
        if err := indexArg(arg, *opts, ignoreDirMap); err != nil {
            log.Fatal(err)
        }
    }*/
    match index_arg("arg") {
        Ok(_v) => {
            println!("Ok")
        },
        Err(_e) => {
            println!("Err")
        },
    }
}

//func indexArg(arg string, opts build.Options, ignore map[string]struct{}) error {
fn index_arg(_arg: &str)  -> Result<String, String> {
    /*dir, err := filepath.Abs(filepath.Clean(arg))
    if err != nil {
        return err
    }

    opts.RepositoryDescription.Name = filepath.Base(dir)
    builder, err := build.NewBuilder(opts)
    if err != nil {
        return err
    }
    defer builder.Finish()

    comm := make(chan fileInfo, 100)
    agg := fileAggregator{
        ignoreDirs: ignore,
        sink:       comm,
        sizeMax:    int64(opts.SizeMax),
    }

    go func() {
        if err := filepath.Walk(dir, agg.add); err != nil {
            log.Fatal(err)
        }
        close(comm)
    }()

    for f := range comm {
        displayName := strings.TrimPrefix(f.name, dir+"/")
        if f.size > int64(opts.SizeMax) && !opts.IgnoreSizeMax(displayName) {
            builder.Add(zoekt.Document{
                Name:       displayName,
                SkipReason: fmt.Sprintf("document size %d larger than limit %d", f.size, opts.SizeMax),
            })
            continue
        }
        content, err := ioutil.ReadFile(f.name)
        if err != nil {
            return err
        }

        builder.AddFile(displayName, content)
    }

    return builder.Finish()*/
	Ok(String::from("builder.Finish()"))
}
