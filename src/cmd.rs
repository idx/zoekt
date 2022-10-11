// Copyright 2019 Google Inc. All rights reserved.
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

/*package cmd

import (
	"flag"
	"fmt"
	"os"
	"path/filepath"

	"github.com/google/zoekt"
	"github.com/google/zoekt/build"
)*/
use crate::build;

/*var (
	version = flag.Bool("version", false, "Print version number")
	opts    = &build.Options{}
)

func init() {
	opts.Flags(flag.CommandLine)
}*/

//func OptionsFromFlags() *build.Options {
pub fn options_from_flags() -> build::builder::Options {
	/*if *version {
		name := filepath.Base(os.Args[0])
		fmt.Printf("%s version %q\n", name, zoekt.Version)
		os.Exit(0)[]
	}

	opts.SetDefaults()
	return opts*/
	build::builder::Options {
		index_dir: String::from("index_dir"),
		repository_description: String::from(""),
    }
}
