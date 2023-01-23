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
//use nom::number::complete::*;
//use std::fs;
use std::{fs::File, u32};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexFileError {
    #[error("out of bounds: {0}, len {1}")]
    Read(u32, usize)
}

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

impl IndexFile {
/*func (f *mmapedIndexFile) Read(off, sz uint32) ([]byte, error) {
    if off+sz > uint32(len(f.data)) {
        return nil, fmt.Errorf("out of bounds: %d, len %d", off+sz, len(f.data))
    }
    return f.data[off : off+sz], nil
}*/
    pub fn read(&self, off: u32, sz: u32) -> Result<Vec<u8>, IndexFileError> {
        if off + sz > self.data.len() as u32 {
            IndexFileError::Read(off + sz, self.data.len());
        }
        Ok(self.data[off as usize..(off + sz) as usize].to_vec())
    }

/*func (f *mmapedIndexFile) Name() string {
    return f.name
}*/
    pub fn name(&self) -> &String {
        &self.name
    }

/*func (f *mmapedIndexFile) Size() (uint32, error) {
    return f.size, nil
}*/
    pub fn size(&self) -> u32 {
        self.size        
    }

/*func (f *mmapedIndexFile) Close() {
    syscall.Munmap(f.data)
}*/
    pub fn close(&self) {

    }
}

// NewIndexFile returns a new index file. The index file takes
// ownership of the passed in file, and may close it.
//func NewIndexFile(f *os.File) (IndexFile,  error) {
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
