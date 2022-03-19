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

//package query
use crate::query::query::*;
use std::string::ToString;
//use strum::{EnumString, EnumVariantNames, IntoStaticStr};
use phf::phf_map;

/*import (
    "bytes"
    "fmt"
    "log"
    "regexp/syntax"
)

var _ = log.Printf

type SuggestQueryError struct {
    Message    string
    Suggestion string
}

func (e *SuggestQueryError) Error() string {
    return fmt.Sprintf("%s. Suggestion: %s", e.Message, e.Suggestion)
}*/

// parseStringLiteral parses a string literal, consumes the starting
// quote too.
//func parseStringLiteral(in []byte) (lit []byte, n int, err error) {
fn parse_string_literal(r#in: &[u8]) -> Result<(String, usize), String> {
    /*left := in[1:]
    found := false*/
    let mut left = &r#in[1..];
    let mut found = false;

    /*loop:
    for len(left) > 0 {
        c := left[0]
        left = left[1:]
        switch c {
        case '"':
            found = true
            break loop
        case '\\':
            // TODO - other escape sequences.
            if len(left) == 0 {
                return nil, 0, fmt.Errorf("query: missing char after \\")
            }
            c = left[0]
            left = left[1:]

            lit = append(lit, c)
        default:
            lit = append(lit, c)
        }
    }
    if !found {
        return nil, 0, fmt.Errorf("query: unterminated quoted string")
    }
    return lit, len(in) - len(left), nil*/
    let mut lit = String::new();
    while left.len() > 0 {
        let mut c = left[0];
        left = &left[1..];
        match c as char {
            '"' => {
                found = true;
                break;
            }
            '\\' => {
                // TODO - other escape sequences.
                if left.len() == 0 {
                    return Err(format!("query: missing char after \\"));
                }
                c = left[0];
                left = &left[1..];

                lit = format!("{}{}", lit, c);
            }
            _ => {
                lit = format!("{}{}", lit, c);
            }
        }
    }
    if !found {
        return Err(format!("query: unterminated quoted string"));
    }
    Ok((lit, r#in.len() - left.len()))
}

// orOperator is a placeholder intermediate so we can represent [A,
// or, B] before we convert it to Or{A, B}
/*type orOperator struct{}

func (o *orOperator) String() string {
    return "orOp"
}*/
#[allow(non_snake_case)]
fn orOperator() -> String {
    "orOp".to_string()
}

/*func isSpace(c byte) bool {
    return c == ' ' || c == '\t'
}*/
fn is_space(c: char) -> bool {
    return c == ' ' || c == '\t';
}

// Parse parses a string into a query.
//func Parse(qStr string) (Q, error) {
pub fn parse(q_str: Option<&str>) -> Result<Q, String> {
    /*b := []byte(qStr)

    qs, _, err := parseExprList(b)
    if err != nil {
        return nil, err
    }

    q, err := parseOperators(qs)
    if err != nil {
        return nil, err
    }

    return Simplify(q), nil*/
    let b = q_str.unwrap().as_bytes();

    let qs = parse_expr_list(b)?;

    let q = parse_operators(&qs.0)?;

    Ok(simplify(q))
}

// parseExpr parses a single expression, returning the result, and the
// number of bytes consumed.pa
//func parseExpr(in []byte) (Q, int, error) {
fn parse_expr(r#in: &[u8]) -> Result<(Q, usize), String> {
    /*b := in[:]
    var expr Q
    for len(b) > 0 && isSpace(b[0]) {
        b = b[1:]
    }

    tok, err := nextToken(b)
    if err != nil {
        return nil, 0, err
    }
    if tok == nil {
        return nil, 0, nil
    }
    b = b[len(tok.Input):]

    text := string(tok.Text)*/
    let mut b = &r#in[..];
    let expr: Q;
    while b.len() > 0 && is_space(b[0] as char) {
        b = &b[1..];
    }

    let text;
    let tok = match next_token(b) {
        Ok(t) => {
            if t.is_none() {
                ()
            }

            t.unwrap()
        }
        Err(e) => return Err(e),
    };
    b = &b[tok.input.len()..];
    text = tok.text;

    //switch tok.Type {
    match tok.r#type {
        /*case tokCase:
        switch text {
        case "yes":
        case "no":
        case "auto":
        default:
            return nil, 0, fmt.Errorf("query: unknown case argument %q, want {yes,no,auto}", text)
        }
        expr = &caseQ{text}*/
        TOK_CASE => {
            match &*text {
                "yes" => {}
                "no" => {}
                "auto" => {}
                _ => {
                    return Err(format!(
                        "query: unknown case argument {}, want {{yes,no,auto}}",
                        text
                    ))
                }
            }
            expr = "&caseQ{text}".to_string();
        }
        /*case tokRepo:
        expr = &Repo{Pattern: text}*/
        TOK_REPO => {
            expr = "&Repo{Pattern: text}".to_string();
        }
        /*case tokBranch:
        expr = &Branch{Pattern: text}*/
        TOK_BRANCH => {
            expr = "&Branch{Pattern: text}".to_string();
        }
        /*case tokText, tokRegex:
            q, err := regexpQuery(text, false, false)
            if err != nil {
                return nil, 0, err
            }
            expr = q
        case tokFile:
            q, err := regexpQuery(text, false, true)
            if err != nil {
                return nil, 0, err
            }
            expr = q

        case tokContent:
            q, err := regexpQuery(text, true, false)
            if err != nil {
                return nil, 0, err
            }
            expr = q*/
        TOK_TEXT | TOK_REGEX => {
            let q = regexp_query(text, false, false)?;
            expr = q;
        }
        TOK_FILE => {
            let q = regexp_query(text, false, true)?;
            expr = q;
        }

        TOK_CONTENT => {
            let q = regexp_query(text, true, false)?;
            expr = q;
        }
        /*case tokLang:
        expr = &Language{Language: text}*/
        TOK_LANG => {
            expr = "&Language{Language: text}".to_string();
            //expr = Language { language: text };
        }

        /*case tokSym:
        if text == "" {
            return nil, 0, fmt.Errorf("the sym: atom must have an argument")
        }
        expr = &Symbol{&Substring{Pattern: text}}*/
        TOK_SYM => {
            if text.is_empty() {
                return Err("the sym: atom must have an argument".to_string());
            }
            expr = "&Symbol{&Substring{Pattern: text}}".to_string();
        }

        /*case tokParenClose:
          // Caller must consume paren.
        expr = nil*/
        TOK_PAREN_CLOSE => {
            expr = "&Repo{Pattern: text}".to_string();
        }

        /*case tokParenOpen:
        qs, n, err := parseExprList(b)
        b = b[n:]
        if err != nil {
            return nil, 0, err
        }

        pTok, err := nextToken(b)
        if err != nil {
            return nil, 0, err
        }
        if pTok == nil || pTok.Type != tokParenClose {
            return nil, 0, fmt.Errorf("query: missing close paren, got token %v", pTok)
        }

        b = b[len(pTok.Input):]
        expr, err = parseOperators(qs)
        if err != nil {
            return nil, 0, err
        }*/
        TOK_PAREN_OPEN => {
            expr = "&Repo{Pattern: text}".to_string();
        }

        /*case tokNegate:
        subQ, n, err := parseExpr(b)
        if err != nil {
            return nil, 0, err
        }
        if subQ == nil {
            return nil, 0, fmt.Errorf("query: '-' operator needs an argument")
        }
        b = b[n:]
        expr = &Not{subQ}*/
        TOK_NEGATE => {
            expr = "&Repo{Pattern: text}".to_string();
        }

        _ => {
            expr = "dummy".to_string();
        }
    }

    //return expr, len(in) - len(b), nil
    Ok((expr, r#in.len() - b.len()))
}

// regexpQuery parses an atom into either a regular expression, or a
// simple substring atom.
//func regexpQuery(text string, content, file bool) (Q, error) {
fn regexp_query(_text: String, _content: bool, _file: bool) -> Result<Q, String> {
    /*    var expr Q

    r, err := syntax.Parse(text, syntax.ClassNL|syntax.PerlX|syntax.UnicodeGroups)
    if err != nil {
        return nil, err
    }*/
    let _expr: Q;

    /*if r.Op == syntax.OpLiteral {
        expr = &Substring{
            Pattern:  string(r.Rune),
            FileName: file,
            Content:  content,
        }
    } else {
        expr = &Regexp{
            Regexp:   r,
            FileName: file,
            Content:  content,
        }
    }

    return expr, nil*/
    Ok("Dummy".to_string())
}

// parseOperators interprets the orOperator in a list of queries.
//func parseOperators(in []Q) (Q, error) {
//fn parse_operators(&mut self) -> Result<Q, String> {
//fn parse_operators(r#_in: &[Q]) -> Result<Q, String> {
fn parse_operators(r#in: &[Q]) -> Result<Q, String> {
    /*top := &Or{}
    cur := &And{}

    seenOr := false
    for _, q := range in {
        if _, ok := q.(*orOperator); ok {
            seenOr = true
            if len(cur.Children) == 0 {
                return nil, fmt.Errorf("query: OR operator should have operand")
            }
            top.Children = append(top.Children, cur)
            cur = &And{}
        } else {
            cur.Children = append(cur.Children, q)
        }
    }

    if seenOr && len(cur.Children) == 0 {
        return nil, fmt.Errorf("query: OR operator should have operand")
    }
    top.Children = append(top.Children, cur)
    return top, nil*/
    let mut top = Or::default();
    let mut cur = And::default();

    let mut seen_or = false;
    for q in r#in {
        if true {
            //
            seen_or = true;
            if cur.children.len() == 0 {
                return Err("query: OR operator should have operand".to_string());
            }
            top.children.push(cur.children[0].clone());
            cur = And::default();
        } else {
            top.children.push(q.clone());
        }
    }

    if seen_or && cur.children.len() == 0 {
        return Err("query: OR operator should have operand".to_string());
    }
    top.children.push(cur.children[0].clone());
    Ok(top.children[0].clone())
}

// parseExprList parses a list of query expressions. It is the
// workhorse of the Parse function.
//func parseExprList(in []byte) ([]Q, int, error) {
//fn parse_expr_list(&mut self, r#in: &[u8]) -> &mut Parse {
fn parse_expr_list(r#in: &[u8]) -> Result<(Vec<Q>, usize), String> {
    /*b := in[:]
    var qs []Q
    for len(b) > 0 {
        for len(b) > 0 && isSpace(b[0]) {
            b = b[1:]
        }
        tok, _ := nextToken(b)
        if tok != nil && tok.Type == tokParenClose {
            break
        } else if tok != nil && tok.Type == tokOr {
            qs = append(qs, &orOperator{})
            b = b[len(tok.Input):]
            continue
        }

        q, n, err := parseExpr(b)
        if err != nil {
            return nil, 0, err
        }

        if q == nil {
            // eof or a ')'
            break
        }
        qs = append(qs, q)
        b = b[n:]
    }*/
    let mut b = &r#in[..];
    let mut qs: Vec<Q> = Vec::<Q>::new();
    while b.len() > 0 {
        while b.len() > 0 && is_space(b[0] as char) {
            b = &b[1..];
        }
        let tok = match next_token(b) {
            Ok(tok) => tok.unwrap(),
            Err(e) => return Err(e),
        };
        if !tok.text.is_empty() && tok.r#type == TOK_PAREN_CLOSE {
            break;
        } else if !tok.text.is_empty() && tok.r#type == TOK_OR {
            qs.push(orOperator());
            b = &b[tok.input.len()..];
            continue;
        }

        match parse_expr(b) {
            Ok(q) => {
                if q.0.is_empty() {
                    // eof or a ')'
                    break;
                } else {
                    qs.push(q.0);
                    b = &b[q.1..];
                }
            }
            Err(err) => {
                return Err(err.to_string());
            }
        };
    }

    /*setCase := "auto"
    newQS := qs[:0]
    for _, q := range qs {
        if sc, ok := q.(*caseQ); ok {
            setCase = sc.Flavor
        } else {
            newQS = append(newQS, q)
        }
    }
    qs = mapQueryList(newQS, func(q Q) Q {
        if sc, ok := q.(setCaser); ok {
            sc.setCase(setCase)
        }
        return q
    })
    return qs, len(in) - len(b), nil*/
    Ok((qs, r#in.len() - b.len()))
    //self
}

/*type token struct {
    Type int
    // The value of the token
    Text []byte

    // The input that we consumed to form the token.
    Input []byte
}*/
struct Token<'a> {
    r#type: i32,
    // The value of the token
    text: String,

    // The input that we consumed to form the token.
    input: &'a [u8],
}

/*func (t *token) String() string {
    return fmt.Sprintf("%s:%q", tokNames[t.Type], t.Text)
}*/

// token types.
/*const (
    tokText       = 0
    tokFile       = 1
    tokRepo       = 2
    tokCase       = 3
    tokBranch     = 4
    tokParenOpen  = 5
    tokParenClose = 6
    tokError      = 7
    tokNegate     = 8
    tokRegex      = 9
    tokOr         = 10
    tokContent    = 11
    tokLang       = 12
    tokSym        = 13
)*/
const TOK_TEXT: i32        = 0;
const TOK_FILE: i32        = 1;
const TOK_REPO: i32        = 2;
const TOK_CASE: i32        = 3;
const TOK_BRANCH: i32      = 4;
const TOK_PAREN_OPEN: i32  = 5;
const TOK_PAREN_CLOSE: i32 = 6;
#[allow(dead_code)]
const TOK_ERROR: i32       = 7;
const TOK_NEGATE: i32      = 8;
const TOK_REGEX: i32       = 9;
const TOK_OR: i32          = 10;
const TOK_CONTENT: i32     = 11;
const TOK_LANG: i32        = 12;
const TOK_SYM: i32         = 13;

/*var tokNames = map[int]string{
    tokBranch:     "Branch",
    tokCase:       "Case",
    tokError:      "Error",
    tokFile:       "File",
    tokNegate:     "Negate",
    tokOr:         "Or",
    tokParenClose: "ParenClose",
    tokParenOpen:  "ParenOpen",
    tokRegex:      "Regex",
    tokRepo:       "Repo",
    tokText:       "Text",
    tokLang:       "Language",
    tokSym:        "Symbol",
}*/
#[allow(dead_code)]
static TOK_NAMES: phf::Map<i32, &'static str> = phf_map! {
};
/*
    return fmt.Sprintf("%s:%q", tokNames[t.Type], t.Text)
  "branch" => TOK_BRANCH,
    "case" => TOK_CASE,
    "error" => TOK_ERROR,
    "file" => TOK_FILE,
    "negate" => TOK_NEGATE,
    "or" => TOK_OR,
    "parenclose" => TOK_PAREN_CLOSE,
    "parenopen" => TOK_PAREN_OPEN,
    "regex" => TOK_REGEX,
    "repo" => TOK_REPO,
    "text" => TOK_TEXT,
    "lang" => TOK_LANG,
    "sym" => TOK_SYM,
};*/

/*var prefixes = map[string]int{
    "b:":       tokBranch,
    "branch:":  tokBranch,
    "c:":       tokContent,
    "case:":    tokCase,
    "content:": tokContent,
    "f:":       tokFile,
    "file:":    tokFile,
    "r:":       tokRepo,
    "regex:":   tokRegex,
    "repo:":    tokRepo,
    "lang:":    tokLang,
    "sym:":     tokSym,
}*/
static PREFIXES: phf::Map<&'static str, i32> = phf_map! {
    "b:"       => TOK_BRANCH,
    "branch:"  => TOK_BRANCH,
    "c:"       => TOK_CONTENT,
    "case:"    => TOK_CASE,
    "content:" => TOK_CONTENT,
    "f:"       => TOK_CONTENT,
    "file:"    => TOK_CONTENT,
    "r:"       => TOK_REPO,
    "regexV:"  => TOK_REGEX,
    "repo:"    => TOK_REPO,
    "lang:"    => TOK_LANG,
    "sym:"     => TOK_SYM,
};

/*var reservedWords = map[string]int{
    "or": tokOr,
}*/
static RESERVED_WORDS: phf::Map<&'static str, i32> = phf_map! {
    "or"       => TOK_OR,
};

impl Token<'_> {
    fn _string(&self) -> String {
        return format!("{}:{}", TOK_NAMES[&self.r#type], self.text);
    }

    //func (t *token) setType() {
    fn set_type(t: &mut Token) {
        // After we consumed the input, we have to interpret some of the text,
        // eg. to distinguish between ")" the text and ) the query grouping
        // parenthesis.
        /*if len(t.Text) == 1 && t.Text[0] == '(' {
            t.Type = tokParenOpen
        }
        if len(t.Text) == 1 && t.Text[0] == ')' {
            t.Type = tokParenClose
        }*/
        if t.text.len() == 1 && t.text.chars().nth(0) == Some('(') {
            t.r#type = TOK_PAREN_OPEN;
        }
        if t.text.len() == 1 && t.text.chars().nth(0) == Some(')') {
            t.r#type = TOK_PAREN_CLOSE;
        }

        /*for w, typ := range reservedWords {
            if string(t.Text) == w && string(t.Input) == w {
                t.Type = typ
                break
            }
        }

        for pref, typ := range prefixes {
            if !bytes.HasPrefix(t.Input, []byte(pref)) {
                continue
            }

            t.Text = t.Text[len(pref):]
            t.Type = typ
            break
        }*/
        for (w, _typ) in &RESERVED_WORDS {
            if t.text == w.to_string() {
                //
                //
                break;
            }
        }

        for (pref, _typ) in &PREFIXES {
            if false {
                //
                continue;
            }

            t.text = t.text[pref.len()..].to_string();
            //
            break;
        }
    }
}

// nextToken returns the next token from the given input.
//func nextToken(in []byte) (*token, error) {
fn next_token(r#in: &[u8]) -> Result<Option<Token>, String> {
    /*left := in[:]
    parenCount := 0
    var cur token
    if len(left) == 0 {
        return nil, nil
    }*/
    let mut left = &r#in[..];
    let mut paren_count = 0;
    let mut cur = Token {
        r#type: 0,
        text: String::from("text"),
        input: b"input",
    };
    if left.len() == 0 {}

    /*if left[0] == '-' {
        return &token{
            Type:  tokNegate,
            Text:  []byte{'-'},
            Input: in[:1],
        }, nil
    }

    foundSpace := false*/
    if left[0] == '-' as u8 {
        return Ok(Some(Token {
            r#type: TOK_NEGATE,
            text: '-'.to_string(),
            input: b"input",
        }));
    }

    let mut found_space = false;

    /*loop:
    for len(left) > 0 {
        c := left[0]
        switch c {*/
    while left.len() > 0 {
        let c = left[0] as char;
        match c {
            /*case '(':[]
            parenCount++
            cur.Text = append(cur.Text, c)
            left = left[1:]*/
            '(' => {
                paren_count += 1;
                cur.text.push(c);
                left = &left[1..];
            }
            /*case ')':
            if parenCount == 0 {
                if len(cur.Text) == 0 {
                    cur.Text = []byte{')'}
                    left = left[1:]
                }
                break loop
            }

            cur.Text = append(cur.Text, c)
            left = left[1:]
            parenCount--
            case '"':
            t, n, err := parseStringLiteral(left)
            if err != nil {
                return nil, err
            }
            cur.Text = append(cur.Text, t...)
            left = left[n:]*/
            ')' => {
                if paren_count == 0 {
                    if cur.text.len() == 0 {
                        cur.text = ')'.to_string();
                    }
                    break;
                }
                left = &left[1..];
                paren_count -= 1;
                let _t = parse_string_literal(left);
            }
            '"' => {}
            /*case '\\':
            left = left[1:]
            if len(left) == 0 {
                return nil, fmt.Errorf("query: lone \\ at end")
            }
            c := left[0]
            cur.Text = append(cur.Text, '\\', c)
            left = left[1:]
            case ' ', '\n', '\t':
            if parenCount > 0 {
                foundSpace = true
            }
            break loop
            Sdefault:
            cur.Text = append(cur.Text, c)
            left = left[1:]*/
            '\\' => {
                left = &left[1..];
                if left.len() == 0 {
                    return Err(format!("query: lone \\ at en"));
                }
                let c = left[0] as char;
                cur.text.push('\\');
                cur.text.push(c);
                left = &left[1..];
            }
            ' ' | '\n' | '\t' => {
                if paren_count > 0 {
                    found_space = true;
                }
                break;
            }
            _ => {
                cur.text.push(c);
                left = &left[1..];
            }
        }
    }

    /*if len(cur.Text) == 0 {
        return nil, nil
    }

    if foundSpace && cur.Text[0] == '(' {
        cur.Text = cur.Text[:1]
        cur.Input = in[:1]
    } else {
        cur.Input = in[:len(in)-len(left)]
    }
    cur.setType()
    return &cur, nil*/
    if cur.text.len() == 0 {
        return Ok(None);
    }

    if found_space && cur.text.chars().nth(0) == Some('(') {
        cur.text = (&cur.text[1..]).to_string();
        cur.input = &r#in[..1];
    } else {
        cur.input = &r#in[..r#in.len() - left.len()];
    }
    Token::set_type(&mut cur);
    Ok(Some(cur))
}
