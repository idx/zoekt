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
use strum::EnumString;

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

//func isSpace(c byte) bool {
fn is_space(c: char) -> bool {
    //    return c == ' ' || c == '\t'
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
        Tok::Case => {
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
        Tok::Repo => {
            expr = "&Repo{Pattern: text}".to_string();
        }
        /*case tokBranch:
        expr = &Branch{Pattern: text}*/
        Tok::Branch => {
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
        Tok::Text | Tok::Regex => {
            let q = regexp_query(text, false, false)?;
            expr = q;
        }
        Tok::File => {
            let q = regexp_query(text, false, true)?;
            expr = q;
        }

        Tok::Content => {
            let q = regexp_query(text, true, false)?;
            expr = q;
        }
        /*case tokLang:
        expr = &Language{Language: text}*/
        Tok::Lang => {
            expr = "&Language{Language: text}".to_string();
            //expr = Language { language: text };
        }

        /*case tokSym:
        if text == "" {
            return nil, 0, fmt.Errorf("the sym: atom must have an argument")
        }
        expr = &Symbol{&Substring{Pattern: text}}*/
        Tok::Sym => {
            if text.is_empty() {
                return Err("the sym: atom must have an argument".to_string());
            }
            expr = "&Symbol{&Substring{Pattern: text}}".to_string();
        }

        /*case tokParenClose:
          // Caller must consume paren.
        expr = nil*/
        Tok::ParenClose => {
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
        Tok::ParenOpen => {
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
        Tok::Negate => {
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
        if true { //
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
        if !tok.text.is_empty() && tok.r#type == Tok::ParenClose {
            break;
        } else if !tok.text.is_empty() && tok.r#type == Tok::Or {
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
    r#type: Tok,
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
//#[derive(EnumString, EnumVariantNames, IntoStaticStr)]
#[derive(EnumString, PartialEq)]
#[strum(serialize_all = "kebab-case")]
enum Tok {
    Text = 0,
    File = 1,
    Repo = 2,
    Case = 3,
    Branch = 4,
    ParenOpen = 5,
    ParenClose = 6,
    Error = 7,
    Negate = 8,
    Regex = 9,
    Or = 10,
    Content = 11,
    Lang = 12,
    Sym = 13,
}

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
static PREFIXES: phf::Map<&'static str, Tok> = phf_map! {
    "b:"       => Tok::Branch,
    "branch:"  => Tok::Branch,
    "c:"       => Tok::Content,
    "case:"    => Tok::Case,
    "content:" => Tok::Content,
    "f:"       => Tok::File,
    "file:"    => Tok::File,
    "r:"       => Tok::Repo,
    "regexV:"  => Tok::Regex,
    "repo:"    => Tok::Repo,
    "lang:"    => Tok::Lang,
    "sym:"     => Tok::Sym,
};

/*var reservedWords = map[string]int{
    "or": tokOr,
}*/
static RESERVED_WORDS: phf::Map<&'static str, Tok> = phf_map! {
    "or"       => Tok::Or,
};

impl Token<'_> {
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
            t.r#type = Tok::ParenOpen;
        }
        if t.text.len() == 1 && t.text.chars().nth(0) == Some(')') {
            t.r#type = Tok::ParenClose
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
            if false  { //
                continue;
            }

            t.text = t.text[pref.len()..].to_string();
            //
            break
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
        r#type: Tok::Text,
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
            r#type: Tok::Negate,
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
