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

/*package zoekt

import (
    "encoding/binary"
    "encoding/json"
    "fmt"
    "log"
    "sort"
)*/
use crate::IndexFile;
use crate::IndexToc;
use crate::Section;
use crate::IndexMetadata;
use crate::SimpleSection;
use anyhow::Error;

// IndexFile is a file suitable for concurrent read access. For performance
// reasons, it allows a mmap'd implementation.
/*type IndexFile interface {
    Read(off uint32, sz uint32) ([]byte, error)
    Size() (uint32, error)
    Close()
    Name() string
}*/

// reader is a stateful file
/*type reader struct {
    r   IndexFile
    off uint32
}*/
pub struct Reader<'a> {
    pub r: &'a IndexFile,
    pub off: u32,
}

impl Reader<'_> {
/*func (r *reader) seek(off uint32) {
    r.off = off
}*/

//func (r *reader) U32() (uint32, error) {
    pub fn u32(&mut self) -> Result<u32, Error> {
    /*b, err := r.r.Read(r.off, 4)
    r.off += 4
    if err != nil {
        return 0, err
    }
    return binary.BigEndian.Uint32(b), nil*/
    let b = self.r.read(self.off, 4)?;
    self.off += 4;
    Ok(u32::from_le_bytes(b.as_slice().try_into()?))
}

/*func (r *reader) U64() (uint64, error) {
    b, err := r.r.Read(r.off, 8)
    r.off += 8
    if err != nil {
        return 0, err
    }
    return binary.BigEndian.Uint64(b), nil
}

func (r *reader) ReadByte() (byte, error) {
    b, err := r.r.Read(r.off, 1)
    r.off += 1
    if err != nil {
        return 0, err
    }
    return b[0], nil
}

func (r *reader) Varint() (uint64, error) {
    v, err := binary.ReadUvarint(r)
    if err != nil {
        return 0, err
    }
    return v, nil
}

func (r *reader) Str() (string, error) {
    slen, err := r.Varint()
    if err != nil {
        return "", err
    }
    b, err := r.r.Read(r.off, uint32(slen))
    if err != nil {
        return "", err
    }
    r.off += uint32(slen)
    return string(b), nil
}*/

//func (r *reader) readTOC(toc *indexTOC) error {
    fn read_toc(&mut self, _toc: &IndexToc<'_>) -> Result<(), Error> {
    //fn read_toc(&mut self, _toc: &IndexToc<'_>) {
    /*sz, err := r.r.Size()
    if err != nil {
        return err
    }
    r.off = sz - 8

    var tocSection simpleSection
    if err := tocSection.read(r); err != nil {
        return err
    }

    r.seek(tocSection.off)

    sectionCount, err := r.U32()
    if err != nil {
        return err
    }*/
    let sz = self.r.size;
    self.off = sz - 8;

    let mut toc_section: SimpleSection = Default::default();
    toc_section.read(self)?;

    /*if sectionCount == 0 {
        // tagged sections are indicated by a 0 sectionCount,
        // and then a list of string-tagged type-indicated sections.
        secs := toc.sectionsTagged()
        for r.off < tocSection.off+tocSection.sz {
            tag, err := r.Str()
            if err != nil {
                return err
            }
            kind, err := r.Varint()
            if err != nil {
                return err
            }
            sec := secs[tag]
            if sec != nil && sec.kind() == sectionKind(kind) {
                // happy path
                if err := sec.read(r); err != nil {
                    return err
                }
                continue
            }
            // error case: skip over unknown section
            if sec == nil {
                log.Printf("file %s TOC has unknown section %q", r.r.Name(), tag)
            } else {
                return fmt.Errorf("file %s TOC section %q expects kind %d, got kind %d", r.r.Name(), tag,
                    kind, sec.kind())
            }
            if kind == 0 {
                (&simpleSection{}).read(r)
            } else if kind == 1 {
                (&compoundSection{}).read(r)
            }
        }
    } else {
        // TODO: Remove this branch when ReaderMinFeatureVersion >= 10

        secs := toc.sections()

        if len(secs) != int(sectionCount) {
            return fmt.Errorf("section count mismatch: got %d want %d", sectionCount, len(secs))
        }

        for _, s := range secs {
            if err := s.read(r); err != nil {
                return err
            }
        }
    }
    return nil*/
        Ok(())
    }

/*func (r *indexData) readSectionBlob(sec simpleSection) ([]byte, error) {
    return r.file.Read(sec.off, sec.sz)
}

func readSectionU32(f IndexFile, sec simpleSection) ([]uint32, error) {
    if sec.sz%4 != 0 {
        return nil, fmt.Errorf("barf: section size %% 4 != 0: sz %d ", sec.sz)
    }
    blob, err := f.Read(sec.off, sec.sz)
    if err != nil {
        return nil, err
    }
    arr := make([]uint32, 0, len(blob)/4)
    for len(blob) > 0 {
        arr = append(arr, binary.BigEndian.Uint32(blob))
        blob = blob[4:]
    }
    return arr, nil
}

func readSectionU64(f IndexFile, sec simpleSection) ([]uint64, error) {
    if sec.sz%8 != 0 {
        return nil, fmt.Errorf("barf: section size %% 8 != 0: sz %d ", sec.sz)
    }
    blob, err := f.Read(sec.off, sec.sz)
    if err != nil {
        return nil, err
    }
    arr := make([]uint64, 0, len(blob)/8)
    for len(blob) > 0 {
        arr = append(arr, binary.BigEndian.Uint64(blob))
        blob = blob[8:]
    }
    return arr, nil
}*/

//func (r *reader) readJSON(data interface{}, sec *simpleSection) error {
    fn read_json(&mut self, _sec: &SimpleSection) -> Result<(), Error> {
    /*blob, err := r.r.Read(sec.off, sec.sz)
    if err != nil {
        return err
    }

    return json.Unmarshal(blob, data)*/
        Ok(())
    }
}

/*func (r *reader) readIndexData(toc *indexTOC) (*indexData, error) {
    d := indexData{
        file:           r.r,
        ngrams:         map[ngram]simpleSection{},
        fileNameNgrams: map[ngram][]uint32{},
        branchIDs:      map[string]uint{},
        branchNames:    map[uint]string{},
    }

    blob, err := d.readSectionBlob(toc.metaData)
    if err != nil {
        return nil, err
    }

    if err := json.Unmarshal(blob, &d.metaData); err != nil {
        return nil, err
    }

    if d.metaData.IndexFormatVersion != IndexFormatVersion {
        return nil, fmt.Errorf("file is v%d, want v%d", d.metaData.IndexFormatVersion, IndexFormatVersion)
    }

    if d.metaData.IndexFeatureVersion < ReadMinFeatureVersion {
        return nil, fmt.Errorf("file is feature version %d, want feature version >= %d", d.metaData.IndexFeatureVersion, ReadMinFeatureVersion)
    }

    if d.metaData.IndexMinReaderVersion > FeatureVersion {
        return nil, fmt.Errorf("file needs read feature version >= %d, have read feature version %d", d.metaData.IndexMinReaderVersion, FeatureVersion)
    }

    blob, err = d.readSectionBlob(toc.repoMetaData)
    if err != nil {
        return nil, err
    }
    if err := json.Unmarshal(blob, &d.repoMetaData); err != nil {
        return nil, err
    }

    d.boundariesStart = toc.fileContents.data.off
    d.boundaries = toc.fileContents.relativeIndex()
    d.newlinesStart = toc.newlines.data.off
    d.newlinesIndex = toc.newlines.relativeIndex()
    d.docSectionsStart = toc.fileSections.data.off
    d.docSectionsIndex = toc.fileSections.relativeIndex()

    d.checksums, err = d.readSectionBlob(toc.contentChecksums)
    if err != nil {
        return nil, err
    }

    d.languages, err = d.readSectionBlob(toc.languages)
    if err != nil {
        return nil, err
    }

    d.ngrams, err = d.readNgrams(toc)
    if err != nil {
        return nil, err
    }

    d.fileBranchMasks, err = readSectionU64(d.file, toc.branchMasks)
    if err != nil {
        return nil, err
    }

    d.fileNameContent, err = d.readSectionBlob(toc.fileNames.data)
    if err != nil {
        return nil, err
    }

    d.fileNameIndex = toc.fileNames.relativeIndex()

    d.fileNameNgrams, err = d.readFileNameNgrams(toc)
    if err != nil {
        return nil, err
    }

    for j, br := range d.repoMetaData.Branches {
        id := uint(1) << uint(j)
        d.branchIDs[br.Name] = id
        d.branchNames[id] = br.Name
    }

    blob, err = d.readSectionBlob(toc.runeDocSections)
    if err != nil {
        return nil, err
    }
    d.runeDocSections = unmarshalDocSections(blob, nil)

    for sect, dest := range map[simpleSection]*[]uint32{
        toc.subRepos:        &d.subRepos,
        toc.runeOffsets:     &d.runeOffsets,
        toc.nameRuneOffsets: &d.fileNameRuneOffsets,
        toc.nameEndRunes:    &d.fileNameEndRunes,
        toc.fileEndRunes:    &d.fileEndRunes,
    } {
        if blob, err := d.readSectionBlob(sect); err != nil {
            return nil, err
        } else {
            *dest = fromSizedDeltas(blob, nil)
        }
    }

    keys := []string{""}
    for k := range d.repoMetaData.SubRepoMap {
        if k != "" { // we used to marshal "" in SubRepoMap. Prevent adding twice.
            keys = append(keys, k)
        }
    }
    sort.Strings(keys)
    d.subRepoPaths = keys

    d.languageMap = map[byte]string{}
    for k, v := range d.metaData.LanguageMap {
        d.languageMap[v] = k
    }

    if err := d.verify(); err != nil {
        return nil, err
    }

    d.calculateStats()
    return &d, nil
}

const ngramEncoding = 8

func (d *indexData) readNgrams(toc *indexTOC) (map[ngram]simpleSection, error) {
    textContent, err := d.readSectionBlob(toc.ngramText)
    if err != nil {
        return nil, err
    }
    postingsIndex := toc.postings.relativeIndex()

    ngrams := make(map[ngram]simpleSection, len(textContent)/ngramEncoding)
    for i := 0; i < len(textContent); i += ngramEncoding {
        j := i / ngramEncoding
        ng := ngram(binary.BigEndian.Uint64(textContent[i : i+ngramEncoding]))
        ngrams[ng] = simpleSection{
            toc.postings.data.off + postingsIndex[j],
            postingsIndex[j+1] - postingsIndex[j],
        }
    }

    return ngrams, nil
}

func (d *indexData) readFileNameNgrams(toc *indexTOC) (map[ngram][]uint32, error) {
    nameNgramText, err := d.readSectionBlob(toc.nameNgramText)
    if err != nil {
        return nil, err
    }

    fileNamePostingsData, err := d.readSectionBlob(toc.namePostings.data)
    if err != nil {
        return nil, err
    }

    fileNamePostingsIndex := toc.namePostings.relativeIndex()

    fileNameNgrams := make(map[ngram][]uint32, len(nameNgramText)/ngramEncoding)
    for i := 0; i < len(nameNgramText); i += ngramEncoding {
        j := i / ngramEncoding
        off := fileNamePostingsIndex[j]
        end := fileNamePostingsIndex[j+1]
        ng := ngram(binary.BigEndian.Uint64(nameNgramText[i : i+ngramEncoding]))
        fileNameNgrams[ng] = fromDeltas(fileNamePostingsData[off:end], nil)
    }

    return fileNameNgrams, nil
}

func (d *indexData) verify() error {
    // This is not an exhaustive check: the postings can easily
    // generate OOB acccesses, and are expensive to check, but this lets us rule out
    // other sources of OOB access.
    n := len(d.fileNameIndex)
    if n == 0 {
        return nil
    }

    n--
    for what, got := range map[string]int{
        "boundaries":        len(d.boundaries) - 1,
        "branch masks":      len(d.fileBranchMasks),
        "doc section index": len(d.docSectionsIndex) - 1,
        "newlines index":    len(d.newlinesIndex) - 1,
    } {
        if got != n {
            return fmt.Errorf("got %s %d, want %d", what, got, n)
        }
    }
    return nil
}

func (d *indexData) readContents(i uint32) ([]byte, error) {
    return d.readSectionBlob(simpleSection{
        off: d.boundariesStart + d.boundaries[i],
        sz:  d.boundaries[i+1] - d.boundaries[i],
    })
}

func (d *indexData) readContentSlice(off uint32, sz uint32) ([]byte, error) {
    // TODO(hanwen): cap result if it is at the end of the content
    // section.
    return d.readSectionBlob(simpleSection{
        off: d.boundariesStart + off,
        sz:  sz,
    })
}

func (d *indexData) readNewlines(i uint32, buf []uint32) ([]uint32, uint32, error) {
    sec := simpleSection{
        off: d.newlinesStart + d.newlinesIndex[i],
        sz:  d.newlinesIndex[i+1] - d.newlinesIndex[i],
    }
    blob, err := d.readSectionBlob(sec)
    if err != nil {
        return nil, 0, err
    }

    return fromSizedDeltas(blob, buf), sec.sz, nil
}

func (d *indexData) readDocSections(i uint32, buf []DocumentSection) ([]DocumentSection, uint32, error) {
    sec := simpleSection{
        off: d.docSectionsStart + d.docSectionsIndex[i],
        sz:  d.docSectionsIndex[i+1] - d.docSectionsIndex[i],
    }
    blob, err := d.readSectionBlob(sec)
    if err != nil {
        return nil, 0, err
    }

    return unmarshalDocSections(blob, buf), sec.sz, nil
}*/

// NewSearcher creates a Searcher for a single index file.  Search
// results coming from this searcher are valid only for the lifetime
// of the Searcher itself, ie. []byte members should be copied into
// fresh buffers if the result is to survive closing the shard.
//func NewSearcher(r IndexFile) (Searcher, error) {
pub fn new_searcher(_r: &IndexFile) {
    /*rd := &reader{r: r}

    var toc indexTOC
    if err := rd.readTOC(&toc); err != nil {
        return nil, err
    }
    indexData, err := rd.readIndexData(&toc)
    if err != nil {
        return nil, err
    }
    indexData.file = r
    return indexData, nil*/
}

// ReadMetadata returns the metadata of index shard without reading
// the index data. The IndexFile is not  closed.
//func ReadMetadata(inf IndexFile) (*Repository, *IndexMetadata, error) {
//pub fn read_metadata(inf: &IndexFile) -> Result<*Repository, *IndexMetadata, Error> {
pub fn read_metadata(inf: &IndexFile) -> Result<IndexMetadata, Error> {
    /*rd := &reader{r: inf}
    var toc indexTOC
    if err := rd.readTOC(&toc); err != nil {
        return nil, nil, err
    }

    var md IndexMetadata
    if err := rd.readJSON(&md, &toc.metaData); err != nil {
        return nil, nil, err
    }

    var repo Repository
    if err := rd.readJSON(&repo, &toc.repoMetaData); err != nil {
        return nil, nil, err
    }

    return &repo, &md, nil*/
	let mut rd = Reader {
		r: inf,
	    off: 0,
    };
    let toc: IndexToc = IndexToc::default();
    rd.read_toc(&toc)?;

    let md = IndexMetadata::default();
    //rd.read_json(&mut md, &toc.meta_data)?;
    rd.read_json(&toc.meta_data)?;

    //let mut repo = Repository::default();    
    //rd.read_json(&mut repo, &toc.repo_meta_data)?;

//    (0..10).for_each(|x| println!("{:016x}: {:02x}", x, inf.data[x]));
    //Ok((&repo, &md))
    Ok(md)
}
