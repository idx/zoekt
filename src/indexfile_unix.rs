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

// +build linux darwin

/*package zoekt

import (
    "fmt"
    "os"
    "syscall"
)*/
use memmap2::{Mmap, MmapOptions};
//use std::fs;
use std::{fs::File};
use std::u32;

/*type mmapedIndexFile struct {
    name string
    size uint32
    data []byte
}*/
pub struct IndexFile {
    pub name: String,
    pub size: u32,
    pub data: Mmap,
}

/*func (f *mmapedIndexFile) Read(off, sz uint32) ([]byte, error) {
    if off+sz > uint32(len(f.data)) {
        return nil, fmt.Errorf("out of bounds: %d, len %d", off+sz, len(f.data))
    }
    return f.data[off : off+sz], nil
}

func (f *mmapedIndexFile) Name() string {
    return f.name
}

func (f *mmapedIndexFile) Size() (uint32, error) {
    return f.size, nil
}

func (f *mmapedIndexFile) Close() {
    syscall.Munmap(f.data)
}*/

// NewIndexFile returns a new index file. The index file takes
// ownership of the passed in file, and may close it.
//func NewIndexFile(f *os.File) (IndexFile, error) {
pub fn new_index_file(filename: String) -> std::io::Result<IndexFile> {
    /*defer f.Close()

    fi, err := f.Stat()
    if err != nil {
        return nil, err
    }

    sz := fi.Size()
    if sz >= maxUInt32 {
        return nil, fmt.Errorf("file %s too large: %d", f.Name(), sz)
    }
    r := &mmapedIndexFile{
        name: f.Name(),
        size: uint32(sz),
    }

    rounded := (r.size + 4095) &^ 4095
    r.data, err = syscall.Mmap(int(f.Fd()), 0, int(rounded), syscall.PROT_READ, syscall.MAP_SHARED)
    if err != nil {
        return nil, err
    }

    return r, err*/
    let name = filename.clone();
    let f = File::open(filename)?;

    let sz = f.metadata().unwrap().len();
    if sz >= u32::MAX as u64 {}

    let rounded = (sz + 4095) & !4095;
    let r = IndexFile {
        name: name,
        size: sz as u32,
        data: unsafe { MmapOptions::new().len(rounded.try_into().unwrap()).map(&f) }?,
    };

    //let _mmap = unsafe { Mmap::map(&f) }?;

    Ok(r)
}
