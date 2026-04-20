//! Generated parser
//!
//! Do not edit manually.

use super::ast::*;
use super::super::common::span::{Span, ParseError, Result};
use super::tokens::{TokenKind, lookup_keyword};

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
    pub span: Span,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    visiting: std::collections::HashSet<(usize, &'static str)>,
    /// GLR-style exclusion set for ambiguous parses.
    /// Contains (start_pos, rule_name, end_pos) tuples to skip.
    excluded_parses: std::collections::HashSet<(usize, &'static str, usize)>,
    /// Stack of (rule_name, entry_pos) for current parse context.
    /// Used by greedy loops to check if their endpoint is excluded.
    rule_context: Vec<(&'static str, usize)>,
    /// LR head position for SCC group 0
    lr_head_0: Option<usize>,
    /// LR head position for SCC group 1
    lr_head_1: Option<usize>,
    /// LR head position for SCC group 2
    lr_head_2: Option<usize>,
    lr_argument: Option<(usize, usize, Argument)>,
    lr_argument_member: Option<(usize, usize, ArgumentMember)>,
    lr_argument_value: Option<(usize, usize, ArgumentValue)>,
    lr_binary_operator_expression: Option<(usize, usize, BinaryOperatorExpression)>,
    lr_bracket_expression: Option<(usize, usize, BracketExpression)>,
    lr_classification_expression: Option<(usize, usize, ClassificationExpression)>,
    lr_collect_expression: Option<(usize, usize, CollectExpression)>,
    lr_conditional_binary_operator_expression: Option<(usize, usize, ConditionalBinaryOperatorExpression)>,
    lr_feature_chain_expression: Option<(usize, usize, FeatureChainExpression)>,
    lr_filter_package: Option<(usize, usize, FilterPackage)>,
    lr_function_operation_expression: Option<(usize, usize, FunctionOperationExpression)>,
    lr_import_declaration: Option<(usize, usize, ImportDeclaration)>,
    lr_index_expression: Option<(usize, usize, IndexExpression)>,
    lr_namespace_import: Option<(usize, usize, NamespaceImport)>,
    lr_non_feature_chain_primary_argument_member: Option<(usize, usize, NonFeatureChainPrimaryArgumentMember)>,
    lr_non_feature_chain_primary_expression: Option<(usize, usize, NonFeatureChainPrimaryExpression)>,
    lr_owned_expression: Option<(usize, usize, OwnedExpression)>,
    lr_primary_argument: Option<(usize, usize, PrimaryArgument)>,
    lr_primary_argument_member: Option<(usize, usize, PrimaryArgumentMember)>,
    lr_primary_argument_value: Option<(usize, usize, PrimaryArgumentValue)>,
    lr_primary_expression: Option<(usize, usize, PrimaryExpression)>,
    lr_select_expression: Option<(usize, usize, SelectExpression)>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: 0,
            visiting: std::collections::HashSet::new(),
            excluded_parses: std::collections::HashSet::new(),
            rule_context: Vec::new(),
            lr_head_0: None,
            lr_head_1: None,
            lr_head_2: None,
            lr_argument: None,
            lr_argument_member: None,
            lr_argument_value: None,
            lr_binary_operator_expression: None,
            lr_bracket_expression: None,
            lr_classification_expression: None,
            lr_collect_expression: None,
            lr_conditional_binary_operator_expression: None,
            lr_feature_chain_expression: None,
            lr_filter_package: None,
            lr_function_operation_expression: None,
            lr_import_declaration: None,
            lr_index_expression: None,
            lr_namespace_import: None,
            lr_non_feature_chain_primary_argument_member: None,
            lr_non_feature_chain_primary_expression: None,
            lr_owned_expression: None,
            lr_primary_argument: None,
            lr_primary_argument_member: None,
            lr_primary_argument_value: None,
            lr_primary_expression: None,
            lr_select_expression: None,
        }
    }


    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns true if this (pos, rule) pair is new (not already being visited).
    /// Inserts it into the visiting set.
    #[inline]
    fn enter_rule(&mut self, rule: &'static str) -> bool {
        self.visiting.insert((self.pos, rule))
    }

    #[inline]
    fn leave_rule(&mut self, pos: usize, rule: &'static str) {
        self.visiting.remove(&(pos, rule));
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn current_span(&self) -> Span {
        self.current().map(|t| t.span).unwrap_or_default()
    }

    fn advance(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, kind: TokenKind) -> Result<&Token> {
        match self.current() {
            Some(t) if t.kind == kind => {
                self.pos += 1;
                Ok(&self.tokens[self.pos - 1])
            }
            Some(t) => Err(ParseError {
                message: format!("expected {:?}, got {:?}", kind, t.kind),
                span: t.span,
            }),
            None => Err(ParseError {
                message: format!("expected {:?}, got EOF", kind),
                span: Span::default(),
            }),
        }
    }

    fn expect_any(&mut self) -> Result<&Token> {
        self.advance().ok_or_else(|| ParseError {
            message: "unexpected EOF".into(),
            span: Span::default(),
        })
    }

    fn save(&self) -> usize {
        self.pos
    }

    fn restore(&mut self, pos: usize) {
        self.pos = pos;
    }

    /// GLR-style: Check if a specific parse result is excluded.
    fn is_parse_excluded(&self, start_pos: usize, rule: &'static str, end_pos: usize) -> bool {
        self.excluded_parses.contains(&(start_pos, rule, end_pos))
    }

    /// GLR-style: Exclude a specific parse result so it won't be used on retry.
    fn exclude_parse(&mut self, start_pos: usize, rule: &'static str, end_pos: usize) {
        self.excluded_parses.insert((start_pos, rule, end_pos));
    }

    /// GLR-style: Clear all exclusions (called when starting a new top-level parse).
    #[allow(dead_code)]
    fn clear_exclusions(&mut self) {
        self.excluded_parses.clear();
    }

    /// Push a rule onto the context stack (called at rule entry).
    #[inline]
    fn push_rule_context(&mut self, rule: &'static str, entry_pos: usize) {
        self.rule_context.push((rule, entry_pos));
    }

    /// Pop a rule from the context stack (called at rule exit).
    #[inline]
    fn pop_rule_context(&mut self) {
        self.rule_context.pop();
    }

    /// Check if the current position is excluded for ANY rule in the context stack.
    /// Used by greedy loops to stop early when continuing would produce an excluded result.
    fn is_current_pos_excluded(&self) -> bool {
        let cur_pos = self.pos;
        for &(rule, entry_pos) in &self.rule_context {
            if self.excluded_parses.contains(&(entry_pos, rule, cur_pos)) {
                return true;
            }
        }
        false
    }

    fn parse_cross_ref(&mut self) -> Result<QualifiedNameRef> {
        // Parse qualified name as cross-reference: Name ('::' Name)*
        // Only consume '::' if the next token after it is a strict name token,
        // so that '::' '*' and '::' '**' remain available for the caller.
        // Uses is_name_token() (not is_name_compatible()) to avoid greedily
        // consuming keywords that serve as structural delimiters in the
        // enclosing grammar rule (e.g. 'then', 'to', 'accept', 'if', 'do').
        let start = self.current_span();
        let mut path = Vec::new();
        
        loop {
            if let Some(t) = self.current() {
                if t.kind.is_name_token() {
                    path.push(t.text.clone());
                    self.advance();
                    // Peek: only consume '::' if followed by another name segment
                    if self.current().map(|t| t.kind == TokenKind::ColonColon).unwrap_or(false) {
                        if self.tokens.get(self.pos + 1).map(|t| t.kind.is_name_token()).unwrap_or(false) {
                            self.advance(); // consume '::'
                            continue;
                        }
                    }
                }
            }
            break;
        }
        
        if path.is_empty() {
            return Err(ParseError {
                message: "expected qualified name".into(),
                span: start,
            });
        }

        let end = self.current_span();
        Ok(QualifiedNameRef { path, span: start.merge(end) })
    }

    /// Parse `Identification`
    pub fn parse_identification(&mut self) -> Result<Identification> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Identification") {
            return Err(ParseError { message: "left-recursive entry into Identification".into(), span: self.current_span() });
        }
        self.push_rule_context("Identification", _entry_pos);
        let _result: Result<Identification> = (|| {
        let start = self.current_span();
        let mut declared_name = None;
        let mut declared_short_name = None;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Lt)?;
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            declared_short_name = Some(v);
            self.expect(TokenKind::Gt)?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            declared_name = Some(v);
            Ok(())
        })().map_err(|e| { self.restore(saved); e });

        let end = self.current_span();
        Ok(Identification {
            span: start.merge(end),
            declared_name,
            declared_short_name,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Identification");
        _result
    }

    /// Parse `RelationshipBody`
    pub fn parse_relationship_body(&mut self) -> Result<RelationshipBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("RelationshipBody") {
            return Err(ParseError { message: "left-recursive entry into RelationshipBody".into(), span: self.current_span() });
        }
        self.push_rule_context("RelationshipBody", _entry_pos);
        let _result: Result<RelationshipBody> = (|| {
        let start = self.current_span();
        let mut relationship_owned_element = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LBrace)?;
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_relationship_owned_element()?;
                    relationship_owned_element.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                relationship_owned_element.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            self.expect(TokenKind::RBrace)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Semi)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(RelationshipBody {
            span: start.merge(end),
            relationship_owned_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "RelationshipBody");
        _result
    }

    /// Parse `RelationshipOwnedElement`
    pub fn parse_relationship_owned_element(&mut self) -> Result<RelationshipOwnedElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("RelationshipOwnedElement") {
            return Err(ParseError { message: "left-recursive entry into RelationshipOwnedElement".into(), span: self.current_span() });
        }
        self.push_rule_context("RelationshipOwnedElement", _entry_pos);
        let _result: Result<RelationshipOwnedElement> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_related_element()?;
            owned_related_element.push(v);
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_annotation()?;
            owned_relationship.push(v);
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(RelationshipOwnedElement {
            span: start.merge(end),
            owned_related_element,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "RelationshipOwnedElement");
        _result
    }

    /// Parse `OwnedRelatedElement`
    pub fn parse_owned_related_element(&mut self) -> Result<OwnedRelatedElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedRelatedElement") {
            return Err(ParseError { message: "left-recursive entry into OwnedRelatedElement".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedRelatedElement", _entry_pos);
        let _result: Result<OwnedRelatedElement> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(OwnedRelatedElement, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_non_feature_element() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedRelatedElement::NonFeatureElement(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature_element() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedRelatedElement::FeatureElement(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected OwnedRelatedElement".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedRelatedElement");
        _result
    }

    /// Parse `Dependency`
    pub fn parse_dependency(&mut self) -> Result<Dependency> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Dependency") {
            return Err(ParseError { message: "left-recursive entry into Dependency".into(), span: self.current_span() });
        }
        self.push_rule_context("Dependency", _entry_pos);
        let _result: Result<Dependency> = (|| {
        let start = self.current_span();
        let mut client = Vec::new();
        let mut owned_relationship = Vec::new();
        let mut supplier = Vec::new();
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_annotation()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.expect(TokenKind::Dependency)?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_identification()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            self.expect(TokenKind::From)?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.parse_cross_ref()?;
        client.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_cross_ref()?;
                client.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.expect(TokenKind::To)?;
        let v = self.parse_cross_ref()?;
        supplier.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_cross_ref()?;
                supplier.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Dependency {
            span: start.merge(end),
            client,
            owned_relationship,
            supplier,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Dependency");
        _result
    }

    /// Parse `Annotation`
    pub fn parse_annotation(&mut self) -> Result<Annotation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Annotation") {
            return Err(ParseError { message: "left-recursive entry into Annotation".into(), span: self.current_span() });
        }
        self.push_rule_context("Annotation", _entry_pos);
        let _result: Result<Annotation> = (|| {
        let start = self.current_span();
        let mut annotated_element_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        annotated_element_opt = Some(v);

        let end = self.current_span();
        Ok(Annotation {
            span: start.merge(end),
            annotated_element: annotated_element_opt.ok_or_else(|| ParseError { message: "missing annotated_element".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Annotation");
        _result
    }

    /// Parse `OwnedAnnotation`
    pub fn parse_owned_annotation(&mut self) -> Result<OwnedAnnotation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedAnnotation") {
            return Err(ParseError { message: "left-recursive entry into OwnedAnnotation".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedAnnotation", _entry_pos);
        let _result: Result<OwnedAnnotation> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_annotating_element()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(OwnedAnnotation {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedAnnotation");
        _result
    }

    /// Parse `AnnotatingElement`
    pub fn parse_annotating_element(&mut self) -> Result<AnnotatingElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("AnnotatingElement") {
            return Err(ParseError { message: "left-recursive entry into AnnotatingElement".into(), span: self.current_span() });
        }
        self.push_rule_context("AnnotatingElement", _entry_pos);
        let _result: Result<AnnotatingElement> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(AnnotatingElement, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_comment() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((AnnotatingElement::Comment(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_documentation() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((AnnotatingElement::Documentation(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_textual_representation() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((AnnotatingElement::TextualRepresentation(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_metadata_feature() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((AnnotatingElement::MetadataFeature(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected AnnotatingElement".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "AnnotatingElement");
        _result
    }

    /// Parse `Comment`
    pub fn parse_comment(&mut self) -> Result<Comment> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Comment") {
            return Err(ParseError { message: "left-recursive entry into Comment".into(), span: self.current_span() });
        }
        self.push_rule_context("Comment", _entry_pos);
        let _result: Result<Comment> = (|| {
        let start = self.current_span();
        let mut body = String::new();
        let mut locale = None;
        let mut owned_relationship = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Comment)?;
            self.parse_identification()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::About)?;
                let v = self.parse_annotation()?;
                owned_relationship.push(v);
                let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
                loop {
                    let saved = self.save();
                    let ok: std::result::Result<(), ParseError> = (|| {
                        self.expect(TokenKind::Comma)?;
                        let v = self.parse_annotation()?;
                        owned_relationship.push(v);
                        Ok(())
                    })();
                    if ok.is_err() { self.restore(saved); break; }
                    if self.save() == saved { break; } // no progress
                    _glr_stop_positions.push(self.pos);
                }
                while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                    _glr_stop_positions.pop();
                    self.pos = *_glr_stop_positions.last().unwrap();
                }
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Locale)?;
            let v = self.expect(TokenKind::String)?.text.clone();
            locale = Some(v);
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.expect(TokenKind::BlockComment)?.text.clone();
        body = v;

        let end = self.current_span();
        Ok(Comment {
            span: start.merge(end),
            body,
            locale,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Comment");
        _result
    }

    /// Parse `Documentation`
    pub fn parse_documentation(&mut self) -> Result<Documentation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Documentation") {
            return Err(ParseError { message: "left-recursive entry into Documentation".into(), span: self.current_span() });
        }
        self.push_rule_context("Documentation", _entry_pos);
        let _result: Result<Documentation> = (|| {
        let start = self.current_span();
        let mut body = String::new();
        let mut locale = None;
        self.expect(TokenKind::Doc)?;
        self.parse_identification()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Locale)?;
            let v = self.expect(TokenKind::String)?.text.clone();
            locale = Some(v);
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.expect(TokenKind::BlockComment)?.text.clone();
        body = v;

        let end = self.current_span();
        Ok(Documentation {
            span: start.merge(end),
            body,
            locale,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Documentation");
        _result
    }

    /// Parse `TextualRepresentation`
    pub fn parse_textual_representation(&mut self) -> Result<TextualRepresentation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TextualRepresentation") {
            return Err(ParseError { message: "left-recursive entry into TextualRepresentation".into(), span: self.current_span() });
        }
        self.push_rule_context("TextualRepresentation", _entry_pos);
        let _result: Result<TextualRepresentation> = (|| {
        let start = self.current_span();
        let mut body = String::new();
        let mut language = String::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Rep)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Language)?;
        let v = self.expect(TokenKind::String)?.text.clone();
        language = v;
        let v = self.expect(TokenKind::BlockComment)?.text.clone();
        body = v;

        let end = self.current_span();
        Ok(TextualRepresentation {
            span: start.merge(end),
            body,
            language,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TextualRepresentation");
        _result
    }

    /// Parse `RootNamespace`
    /// Entry point
    pub fn parse_root_namespace(&mut self) -> Result<RootNamespace> {
        let _entry_pos = self.pos;
        if !self.enter_rule("RootNamespace") {
            return Err(ParseError { message: "left-recursive entry into RootNamespace".into(), span: self.current_span() });
        }
        self.push_rule_context("RootNamespace", _entry_pos);
        let _result: Result<RootNamespace> = (|| {
        let start = self.current_span();
        let mut namespace_body_element = Vec::new();
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_namespace_body_element()?;
                namespace_body_element.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            namespace_body_element.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(RootNamespace {
            span: start.merge(end),
            namespace_body_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "RootNamespace");
        _result
    }

    /// Parse `Namespace`
    pub fn parse_namespace(&mut self) -> Result<Namespace> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Namespace") {
            return Err(ParseError { message: "left-recursive entry into Namespace".into(), span: self.current_span() });
        }
        self.push_rule_context("Namespace", _entry_pos);
        let _result: Result<Namespace> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.parse_namespace_declaration()?;
        self.parse_namespace_body()?;

        let end = self.current_span();
        Ok(Namespace {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Namespace");
        _result
    }

    /// Parse `NamespaceDeclaration`
    pub fn parse_namespace_declaration(&mut self) -> Result<NamespaceDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamespaceDeclaration") {
            return Err(ParseError { message: "left-recursive entry into NamespaceDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("NamespaceDeclaration", _entry_pos);
        let _result: Result<NamespaceDeclaration> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Namespace)?;
        self.parse_identification()?;

        let end = self.current_span();
        Ok(NamespaceDeclaration {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamespaceDeclaration");
        _result
    }

    /// Parse `NamespaceBody`
    pub fn parse_namespace_body(&mut self) -> Result<NamespaceBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamespaceBody") {
            return Err(ParseError { message: "left-recursive entry into NamespaceBody".into(), span: self.current_span() });
        }
        self.push_rule_context("NamespaceBody", _entry_pos);
        let _result: Result<NamespaceBody> = (|| {
        let start = self.current_span();
        let mut namespace_body_element = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LBrace)?;
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_namespace_body_element()?;
                    namespace_body_element.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                namespace_body_element.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            self.expect(TokenKind::RBrace)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Semi)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(NamespaceBody {
            span: start.merge(end),
            namespace_body_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamespaceBody");
        _result
    }

    /// Parse `NamespaceBodyElement`
    pub fn parse_namespace_body_element(&mut self) -> Result<NamespaceBodyElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamespaceBodyElement") {
            return Err(ParseError { message: "left-recursive entry into NamespaceBodyElement".into(), span: self.current_span() });
        }
        self.push_rule_context("NamespaceBodyElement", _entry_pos);
        let _result: Result<NamespaceBodyElement> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_namespace_member()?;
            owned_relationship.push(NamespaceBodyElementOwnedRelationshipMember::NamespaceMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_alias_member()?;
            owned_relationship.push(NamespaceBodyElementOwnedRelationshipMember::AliasMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_import()?;
            owned_relationship.push(NamespaceBodyElementOwnedRelationshipMember::Import(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(NamespaceBodyElement {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamespaceBodyElement");
        _result
    }

    /// Parse `MemberPrefix`
    pub fn parse_member_prefix(&mut self) -> Result<MemberPrefix> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MemberPrefix") {
            return Err(ParseError { message: "left-recursive entry into MemberPrefix".into(), span: self.current_span() });
        }
        self.push_rule_context("MemberPrefix", _entry_pos);
        let _result: Result<MemberPrefix> = (|| {
        let start = self.current_span();
        let mut visibility = None;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = self.parse_visibility_indicator()?;
            visibility = Some(Box::new(v));
            Ok(())
        })().map_err(|e| { self.restore(saved); e });

        let end = self.current_span();
        Ok(MemberPrefix {
            span: start.merge(end),
            visibility,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MemberPrefix");
        _result
    }

    /// Parse `VisibilityIndicator`
    pub fn parse_visibility_indicator(&mut self) -> Result<VisibilityIndicator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("VisibilityIndicator") {
            return Err(ParseError { message: "left-recursive entry into VisibilityIndicator".into(), span: self.current_span() });
        }
        self.push_rule_context("VisibilityIndicator", _entry_pos);
        let _result: Result<VisibilityIndicator> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Public)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Private)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Protected)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(VisibilityIndicator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "VisibilityIndicator");
        _result
    }

    /// Parse `NamespaceMember`
    pub fn parse_namespace_member(&mut self) -> Result<NamespaceMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamespaceMember") {
            return Err(ParseError { message: "left-recursive entry into NamespaceMember".into(), span: self.current_span() });
        }
        self.push_rule_context("NamespaceMember", _entry_pos);
        let _result: Result<NamespaceMember> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(NamespaceMember, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_non_feature_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NamespaceMember::NonFeatureMember(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_namespace_feature_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NamespaceMember::NamespaceFeatureMember(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected NamespaceMember".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamespaceMember");
        _result
    }

    /// Parse `NonFeatureMember`
    pub fn parse_non_feature_member(&mut self) -> Result<NonFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NonFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into NonFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("NonFeatureMember", _entry_pos);
        let _result: Result<NonFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.parse_member_prefix()?;
        let v = self.parse_member_element()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(NonFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NonFeatureMember");
        _result
    }

    /// Parse `NamespaceFeatureMember`
    pub fn parse_namespace_feature_member(&mut self) -> Result<NamespaceFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamespaceFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into NamespaceFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("NamespaceFeatureMember", _entry_pos);
        let _result: Result<NamespaceFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.parse_member_prefix()?;
        let v = self.parse_feature_element()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(NamespaceFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamespaceFeatureMember");
        _result
    }

    /// Parse `AliasMember`
    pub fn parse_alias_member(&mut self) -> Result<AliasMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("AliasMember") {
            return Err(ParseError { message: "left-recursive entry into AliasMember".into(), span: self.current_span() });
        }
        self.push_rule_context("AliasMember", _entry_pos);
        let _result: Result<AliasMember> = (|| {
        let start = self.current_span();
        let mut member_element_opt: Option<_> = None;
        let mut member_name = None;
        let mut member_short_name = None;
        self.parse_member_prefix()?;
        self.expect(TokenKind::Alias)?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Lt)?;
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            member_short_name = Some(v);
            self.expect(TokenKind::Gt)?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            member_name = Some(v);
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::For)?;
        let v = self.parse_cross_ref()?;
        member_element_opt = Some(v);
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(AliasMember {
            span: start.merge(end),
            member_element: member_element_opt.ok_or_else(|| ParseError { message: "missing member_element".into(), span: start })?,
            member_name,
            member_short_name,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "AliasMember");
        _result
    }

    /// Parse `QualifiedName`
    pub fn parse_qualified_name(&mut self) -> Result<QualifiedName> {
        let _entry_pos = self.pos;
        if !self.enter_rule("QualifiedName") {
            return Err(ParseError { message: "left-recursive entry into QualifiedName".into(), span: self.current_span() });
        }
        self.push_rule_context("QualifiedName", _entry_pos);
        let _result: Result<QualifiedName> = (|| {
        let start = self.current_span();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Dollar)?;
            self.expect(TokenKind::ColonColon)?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        loop {
            let saved = self.save();
            let body_ok: std::result::Result<(), ParseError> = (|| {
                match self.current() {
                    Some(t) if t.kind.is_name_token() => { self.pos += 1; }
                    Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                    None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
                }
                self.expect(TokenKind::ColonColon)?;
                Ok(())
            })();
            if body_ok.is_err() {
                self.restore(saved);
                break; // Loop body failed, exit
            }
            if self.save() == saved { break; } // No progress, exit
            // Loop body succeeded - check if remainder can still parse
            let pos_after_body = self.save();
            let remainder_ok = (|| -> std::result::Result<(), ParseError> {
                match self.current() {
                    Some(t) if t.kind.is_name_token() => { self.pos += 1; }
                    Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                    None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
                }
                Ok(())
            })().is_ok();
            self.restore(pos_after_body);
            if !remainder_ok {
                // Remainder can't parse after consuming this iteration
                // Backtrack and leave input for remainder
                self.restore(saved);
                break;
            }
            // Remainder can parse, keep going
        }
        match self.current() {
            Some(t) if t.kind.is_name_token() => { self.pos += 1; }
            Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
            None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
        }

        let end = self.current_span();
        Ok(QualifiedName {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "QualifiedName");
        _result
    }

    /// Parse `Import`
    pub fn parse_import(&mut self) -> Result<Import> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Import") {
            return Err(ParseError { message: "left-recursive entry into Import".into(), span: self.current_span() });
        }
        self.push_rule_context("Import", _entry_pos);
        let _result: Result<Import> = (|| {
        let start = self.current_span();
        let mut is_import_all = false;
        let mut visibility_opt: Option<_> = None;
        let v = self.parse_visibility_indicator()?;
        visibility_opt = Some(Box::new(v));
        self.expect(TokenKind::Import)?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::All)?;
            is_import_all = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_import_declaration()?;
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Import {
            span: start.merge(end),
            is_import_all,
            visibility: visibility_opt.ok_or_else(|| ParseError { message: "missing visibility".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Import");
        _result
    }

    /// Parse body of `ImportDeclaration` (left-recursive helper)
    fn parse_import_declaration_body(&mut self) -> Result<ImportDeclaration> {
        let alt_saved = self.save();
        let mut best: Option<(ImportDeclaration, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_membership_import() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((ImportDeclaration::MembershipImport(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_namespace_import() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((ImportDeclaration::NamespaceImport(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected ImportDeclaration".into(), span: self.current_span() })
        }
    }

    /// Parse `ImportDeclaration` (left-recursive, seed-grow)
    pub fn parse_import_declaration(&mut self) -> Result<ImportDeclaration> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "ImportDeclaration")) {
            if let Some((start, end, ref result)) = self.lr_import_declaration {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into ImportDeclaration".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_2 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "ImportDeclaration"));
        self.push_rule_context("ImportDeclaration", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_2;
            self.lr_head_2 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_import_declaration_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_import_declaration = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_import_declaration_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_import_declaration = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ImportDeclaration"));
                self.lr_head_2 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ImportDeclaration"));
                self.lr_head_2 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_import_declaration_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "ImportDeclaration"));
            result
        }
    }

    /// Parse `MembershipImport`
    pub fn parse_membership_import(&mut self) -> Result<MembershipImport> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MembershipImport") {
            return Err(ParseError { message: "left-recursive entry into MembershipImport".into(), span: self.current_span() });
        }
        self.push_rule_context("MembershipImport", _entry_pos);
        let _result: Result<MembershipImport> = (|| {
        let start = self.current_span();
        let mut imported_membership_opt: Option<_> = None;
        let mut is_recursive = false;
        let v = self.parse_cross_ref()?;
        imported_membership_opt = Some(v);
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::ColonColon)?;
            self.expect(TokenKind::StarStar)?;
            is_recursive = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });

        let end = self.current_span();
        Ok(MembershipImport {
            span: start.merge(end),
            imported_membership: imported_membership_opt.ok_or_else(|| ParseError { message: "missing imported_membership".into(), span: start })?,
            is_recursive,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MembershipImport");
        _result
    }

    /// Parse body of `NamespaceImport` (left-recursive helper)
    fn parse_namespace_import_body(&mut self) -> Result<NamespaceImport> {
        let start = self.current_span();
        let mut imported_namespace = None;
        let mut is_recursive = false;
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            imported_namespace = Some(NamespaceImportImportedNamespaceMember::QualifiedNameRef(v));
            self.expect(TokenKind::ColonColon)?;
            self.expect(TokenKind::Star)?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::ColonColon)?;
                self.expect(TokenKind::StarStar)?;
                is_recursive = true;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_filter_package()?;
            imported_namespace = Some(NamespaceImportImportedNamespaceMember::FilterPackage(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(NamespaceImport {
            span: start.merge(end),
            imported_namespace,
            is_recursive,
        })
    }

    /// Parse `NamespaceImport` (left-recursive, seed-grow)
    pub fn parse_namespace_import(&mut self) -> Result<NamespaceImport> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "NamespaceImport")) {
            if let Some((start, end, ref result)) = self.lr_namespace_import {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into NamespaceImport".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_2 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "NamespaceImport"));
        self.push_rule_context("NamespaceImport", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_2;
            self.lr_head_2 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_namespace_import_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_namespace_import = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_namespace_import_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_namespace_import = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "NamespaceImport"));
                self.lr_head_2 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "NamespaceImport"));
                self.lr_head_2 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_namespace_import_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "NamespaceImport"));
            result
        }
    }

    /// Parse body of `FilterPackage` (left-recursive helper)
    fn parse_filter_package_body(&mut self) -> Result<FilterPackage> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_import_declaration()?;
        owned_relationship.push(FilterPackageOwnedRelationshipMember::ImportDeclaration(Box::new(v)));
        let mut _glr_stop_positions: Vec<usize> = Vec::new();
        let v = self.parse_filter_package_member()?;
        owned_relationship.push(FilterPackageOwnedRelationshipMember::FilterPackageMember(Box::new(v)));
        _glr_stop_positions.push(self.pos);
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_filter_package_member()?;
                owned_relationship.push(FilterPackageOwnedRelationshipMember::FilterPackageMember(Box::new(v)));
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(FilterPackage {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `FilterPackage` (left-recursive, seed-grow)
    pub fn parse_filter_package(&mut self) -> Result<FilterPackage> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "FilterPackage")) {
            if let Some((start, end, ref result)) = self.lr_filter_package {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into FilterPackage".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_2 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "FilterPackage"));
        self.push_rule_context("FilterPackage", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_2;
            self.lr_head_2 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_filter_package_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_filter_package = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_filter_package_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_filter_package = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "FilterPackage"));
                self.lr_head_2 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "FilterPackage"));
                self.lr_head_2 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_filter_package_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "FilterPackage"));
            result
        }
    }

    /// Parse `FilterPackageMember`
    pub fn parse_filter_package_member(&mut self) -> Result<FilterPackageMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FilterPackageMember") {
            return Err(ParseError { message: "left-recursive entry into FilterPackageMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FilterPackageMember", _entry_pos);
        let _result: Result<FilterPackageMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.expect(TokenKind::LBracket)?;
        let v = self.parse_owned_expression()?;
        owned_related_element.push(v);
        self.expect(TokenKind::RBracket)?;

        let end = self.current_span();
        Ok(FilterPackageMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FilterPackageMember");
        _result
    }

    /// Parse `MemberElement`
    pub fn parse_member_element(&mut self) -> Result<MemberElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MemberElement") {
            return Err(ParseError { message: "left-recursive entry into MemberElement".into(), span: self.current_span() });
        }
        self.push_rule_context("MemberElement", _entry_pos);
        let _result: Result<MemberElement> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(MemberElement, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_annotating_element() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((MemberElement::AnnotatingElement(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_non_feature_element() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((MemberElement::NonFeatureElement(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected MemberElement".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MemberElement");
        _result
    }

    /// Parse `NonFeatureElement`
    pub fn parse_non_feature_element(&mut self) -> Result<NonFeatureElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NonFeatureElement") {
            return Err(ParseError { message: "left-recursive entry into NonFeatureElement".into(), span: self.current_span() });
        }
        self.push_rule_context("NonFeatureElement", _entry_pos);
        let _result: Result<NonFeatureElement> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(NonFeatureElement, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_dependency() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Dependency(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_namespace() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Namespace(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_type_() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Type(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_classifier() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Classifier(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_data_type() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::DataType(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_class() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Class(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_structure() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Structure(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_metaclass() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Metaclass(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_association() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Association(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_association_structure() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::AssociationStructure(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_interaction() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Interaction(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_behavior() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Behavior(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_function() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Function(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_predicate() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Predicate(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_multiplicity() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Multiplicity(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_package() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Package(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_library_package() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::LibraryPackage(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_specialization() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Specialization(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_conjugation() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Conjugation(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_subclassification() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Subclassification(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_disjoining() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Disjoining(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature_inverting() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::FeatureInverting(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature_typing() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::FeatureTyping(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_subsetting() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Subsetting(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_redefinition() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::Redefinition(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_type_featuring() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureElement::TypeFeaturing(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected NonFeatureElement".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NonFeatureElement");
        _result
    }

    /// Parse `FeatureElement`
    pub fn parse_feature_element(&mut self) -> Result<FeatureElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureElement") {
            return Err(ParseError { message: "left-recursive entry into FeatureElement".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureElement", _entry_pos);
        let _result: Result<FeatureElement> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(FeatureElement, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Feature(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_step() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Step(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Expression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_boolean_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::BooleanExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_invariant() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Invariant(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_connector() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Connector(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_binding_connector() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::BindingConnector(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_succession() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Succession(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_flow() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::Flow(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_succession_flow() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureElement::SuccessionFlow(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected FeatureElement".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureElement");
        _result
    }

    /// Parse `Type`
    pub fn parse_type_(&mut self) -> Result<Type> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Type") {
            return Err(ParseError { message: "left-recursive entry into Type".into(), span: self.current_span() });
        }
        self.push_rule_context("Type", _entry_pos);
        let _result: Result<Type> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Type)?;
        self.parse_type_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Type {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Type");
        _result
    }

    /// Parse `TypePrefix`
    pub fn parse_type_prefix(&mut self) -> Result<TypePrefix> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypePrefix") {
            return Err(ParseError { message: "left-recursive entry into TypePrefix".into(), span: self.current_span() });
        }
        self.push_rule_context("TypePrefix", _entry_pos);
        let _result: Result<TypePrefix> = (|| {
        let start = self.current_span();
        let mut is_abstract = false;
        let mut owned_relationship = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Abstract)?;
            is_abstract = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(TypePrefix {
            span: start.merge(end),
            is_abstract,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypePrefix");
        _result
    }

    /// Parse `TypeDeclaration`
    pub fn parse_type_declaration(&mut self) -> Result<TypeDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeDeclaration") {
            return Err(ParseError { message: "left-recursive entry into TypeDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeDeclaration", _entry_pos);
        let _result: Result<TypeDeclaration> = (|| {
        let start = self.current_span();
        let mut is_sufficient = false;
        let mut owned_relationship = Vec::new();
        let mut type_relationship_part = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::All)?;
            is_sufficient = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_identification()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = self.parse_owned_multiplicity()?;
            owned_relationship.push(v);
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let mut _glr_stop_positions: Vec<usize> = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_specialization_part()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_conjugation_part()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        _glr_stop_positions.push(self.pos);
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let saved_alt = self.save();
                let mut best_alt_pos: Option<usize> = None;
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.parse_specialization_part()?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.parse_conjugation_part()?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                match best_alt_pos {
                    Some(pos) => self.pos = pos,
                    None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
                }
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_type_relationship_part()?;
                type_relationship_part.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            type_relationship_part.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(TypeDeclaration {
            span: start.merge(end),
            is_sufficient,
            owned_relationship,
            type_relationship_part,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeDeclaration");
        _result
    }

    /// Parse `SpecializationPart`
    pub fn parse_specialization_part(&mut self) -> Result<SpecializationPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SpecializationPart") {
            return Err(ParseError { message: "left-recursive entry into SpecializationPart".into(), span: self.current_span() });
        }
        self.push_rule_context("SpecializationPart", _entry_pos);
        let _result: Result<SpecializationPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Specializes)?;
            Ok(())
        })?;
        let v = self.parse_owned_specialization()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_specialization()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(SpecializationPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SpecializationPart");
        _result
    }

    /// Parse `ConjugationPart`
    pub fn parse_conjugation_part(&mut self) -> Result<ConjugationPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConjugationPart") {
            return Err(ParseError { message: "left-recursive entry into ConjugationPart".into(), span: self.current_span() });
        }
        self.push_rule_context("ConjugationPart", _entry_pos);
        let _result: Result<ConjugationPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Tilde)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Conjugates)?;
            Ok(())
        })?;
        let v = self.parse_owned_conjugation()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(ConjugationPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConjugationPart");
        _result
    }

    /// Parse `TypeRelationshipPart`
    pub fn parse_type_relationship_part(&mut self) -> Result<TypeRelationshipPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeRelationshipPart") {
            return Err(ParseError { message: "left-recursive entry into TypeRelationshipPart".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeRelationshipPart", _entry_pos);
        let _result: Result<TypeRelationshipPart> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(TypeRelationshipPart, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_disjoining_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((TypeRelationshipPart::DisjoiningPart(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_unioning_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((TypeRelationshipPart::UnioningPart(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_intersecting_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((TypeRelationshipPart::IntersectingPart(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_differencing_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((TypeRelationshipPart::DifferencingPart(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected TypeRelationshipPart".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeRelationshipPart");
        _result
    }

    /// Parse `DisjoiningPart`
    pub fn parse_disjoining_part(&mut self) -> Result<DisjoiningPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("DisjoiningPart") {
            return Err(ParseError { message: "left-recursive entry into DisjoiningPart".into(), span: self.current_span() });
        }
        self.push_rule_context("DisjoiningPart", _entry_pos);
        let _result: Result<DisjoiningPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Disjoint)?;
        self.expect(TokenKind::From)?;
        let v = self.parse_owned_disjoining()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_disjoining()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(DisjoiningPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "DisjoiningPart");
        _result
    }

    /// Parse `UnioningPart`
    pub fn parse_unioning_part(&mut self) -> Result<UnioningPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("UnioningPart") {
            return Err(ParseError { message: "left-recursive entry into UnioningPart".into(), span: self.current_span() });
        }
        self.push_rule_context("UnioningPart", _entry_pos);
        let _result: Result<UnioningPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Unions)?;
        let v = self.parse_unioning()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_unioning()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(UnioningPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "UnioningPart");
        _result
    }

    /// Parse `IntersectingPart`
    pub fn parse_intersecting_part(&mut self) -> Result<IntersectingPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("IntersectingPart") {
            return Err(ParseError { message: "left-recursive entry into IntersectingPart".into(), span: self.current_span() });
        }
        self.push_rule_context("IntersectingPart", _entry_pos);
        let _result: Result<IntersectingPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Intersects)?;
        let v = self.parse_intersecting()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_intersecting()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(IntersectingPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "IntersectingPart");
        _result
    }

    /// Parse `DifferencingPart`
    pub fn parse_differencing_part(&mut self) -> Result<DifferencingPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("DifferencingPart") {
            return Err(ParseError { message: "left-recursive entry into DifferencingPart".into(), span: self.current_span() });
        }
        self.push_rule_context("DifferencingPart", _entry_pos);
        let _result: Result<DifferencingPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Differences)?;
        let v = self.parse_differencing()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_differencing()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(DifferencingPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "DifferencingPart");
        _result
    }

    /// Parse `TypeBody`
    pub fn parse_type_body(&mut self) -> Result<TypeBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeBody") {
            return Err(ParseError { message: "left-recursive entry into TypeBody".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeBody", _entry_pos);
        let _result: Result<TypeBody> = (|| {
        let start = self.current_span();
        let mut type_body_element = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LBrace)?;
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_type_body_element()?;
                    type_body_element.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                type_body_element.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            self.expect(TokenKind::RBrace)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Semi)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(TypeBody {
            span: start.merge(end),
            type_body_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeBody");
        _result
    }

    /// Parse `TypeBodyElement`
    pub fn parse_type_body_element(&mut self) -> Result<TypeBodyElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeBodyElement") {
            return Err(ParseError { message: "left-recursive entry into TypeBodyElement".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeBodyElement", _entry_pos);
        let _result: Result<TypeBodyElement> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_non_feature_member()?;
            owned_relationship.push(TypeBodyElementOwnedRelationshipMember::NonFeatureMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_member()?;
            owned_relationship.push(TypeBodyElementOwnedRelationshipMember::FeatureMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_alias_member()?;
            owned_relationship.push(TypeBodyElementOwnedRelationshipMember::AliasMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_import()?;
            owned_relationship.push(TypeBodyElementOwnedRelationshipMember::Import(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(TypeBodyElement {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeBodyElement");
        _result
    }

    /// Parse `Specialization`
    pub fn parse_specialization(&mut self) -> Result<Specialization> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Specialization") {
            return Err(ParseError { message: "left-recursive entry into Specialization".into(), span: self.current_span() });
        }
        self.push_rule_context("Specialization", _entry_pos);
        let _result: Result<Specialization> = (|| {
        let start = self.current_span();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Specialization)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Subtype)?;
        self.parse_specific_type()?;
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Specializes)?;
            Ok(())
        })?;
        self.parse_general_type()?;
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Specialization {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Specialization");
        _result
    }

    /// Parse `OwnedSpecialization`
    pub fn parse_owned_specialization(&mut self) -> Result<OwnedSpecialization> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedSpecialization") {
            return Err(ParseError { message: "left-recursive entry into OwnedSpecialization".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedSpecialization", _entry_pos);
        let _result: Result<OwnedSpecialization> = (|| {
        let start = self.current_span();
        self.parse_general_type()?;

        let end = self.current_span();
        Ok(OwnedSpecialization {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedSpecialization");
        _result
    }

    /// Parse `SpecificType`
    pub fn parse_specific_type(&mut self) -> Result<SpecificType> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SpecificType") {
            return Err(ParseError { message: "left-recursive entry into SpecificType".into(), span: self.current_span() });
        }
        self.push_rule_context("SpecificType", _entry_pos);
        let _result: Result<SpecificType> = (|| {
        let start = self.current_span();
        let mut specific = Vec::new();
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            specific.push(SpecificTypeSpecificMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "SpecificType", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            specific.push(SpecificTypeSpecificMember::OwnedFeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "SpecificType", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(SpecificType {
            span: start.merge(end),
            specific,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SpecificType");
        _result
    }

    /// Parse `GeneralType`
    pub fn parse_general_type(&mut self) -> Result<GeneralType> {
        let _entry_pos = self.pos;
        if !self.enter_rule("GeneralType") {
            return Err(ParseError { message: "left-recursive entry into GeneralType".into(), span: self.current_span() });
        }
        self.push_rule_context("GeneralType", _entry_pos);
        let _result: Result<GeneralType> = (|| {
        let start = self.current_span();
        let mut general = Vec::new();
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            general.push(GeneralTypeGeneralMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "GeneralType", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            general.push(GeneralTypeGeneralMember::OwnedFeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "GeneralType", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(GeneralType {
            span: start.merge(end),
            general,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "GeneralType");
        _result
    }

    /// Parse `Conjugation`
    pub fn parse_conjugation(&mut self) -> Result<Conjugation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Conjugation") {
            return Err(ParseError { message: "left-recursive entry into Conjugation".into(), span: self.current_span() });
        }
        self.push_rule_context("Conjugation", _entry_pos);
        let _result: Result<Conjugation> = (|| {
        let start = self.current_span();
        let mut conjugated_type = None;
        let mut original_type = None;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Conjugation)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Conjugate)?;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            conjugated_type = Some(ConjugationConjugatedTypeMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Conjugation", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_chain()?;
            conjugated_type = Some(ConjugationConjugatedTypeMember::FeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Conjugation", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Tilde)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Conjugates)?;
            Ok(())
        })?;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            original_type = Some(ConjugationOriginalTypeMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Conjugation", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_chain()?;
            original_type = Some(ConjugationOriginalTypeMember::FeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Conjugation", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Conjugation {
            span: start.merge(end),
            conjugated_type,
            original_type,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Conjugation");
        _result
    }

    /// Parse `OwnedConjugation`
    pub fn parse_owned_conjugation(&mut self) -> Result<OwnedConjugation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedConjugation") {
            return Err(ParseError { message: "left-recursive entry into OwnedConjugation".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedConjugation", _entry_pos);
        let _result: Result<OwnedConjugation> = (|| {
        let start = self.current_span();
        let mut original_type = None;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            original_type = Some(OwnedConjugationOriginalTypeMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "OwnedConjugation", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_chain()?;
            original_type = Some(OwnedConjugationOriginalTypeMember::FeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "OwnedConjugation", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(OwnedConjugation {
            span: start.merge(end),
            original_type,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedConjugation");
        _result
    }

    /// Parse `Disjoining`
    pub fn parse_disjoining(&mut self) -> Result<Disjoining> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Disjoining") {
            return Err(ParseError { message: "left-recursive entry into Disjoining".into(), span: self.current_span() });
        }
        self.push_rule_context("Disjoining", _entry_pos);
        let _result: Result<Disjoining> = (|| {
        let start = self.current_span();
        let mut disjoining_type = None;
        let mut type_disjoined = None;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Disjoining)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Disjoint)?;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            type_disjoined = Some(DisjoiningTypeDisjoinedMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Disjoining", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_chain()?;
            type_disjoined = Some(DisjoiningTypeDisjoinedMember::FeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Disjoining", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.expect(TokenKind::From)?;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            disjoining_type = Some(DisjoiningDisjoiningTypeMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Disjoining", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_chain()?;
            disjoining_type = Some(DisjoiningDisjoiningTypeMember::FeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Disjoining", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Disjoining {
            span: start.merge(end),
            disjoining_type,
            type_disjoined,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Disjoining");
        _result
    }

    /// Parse `OwnedDisjoining`
    pub fn parse_owned_disjoining(&mut self) -> Result<OwnedDisjoining> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedDisjoining") {
            return Err(ParseError { message: "left-recursive entry into OwnedDisjoining".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedDisjoining", _entry_pos);
        let _result: Result<OwnedDisjoining> = (|| {
        let start = self.current_span();
        let mut disjoining_type = None;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            disjoining_type = Some(OwnedDisjoiningDisjoiningTypeMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "OwnedDisjoining", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_chain()?;
            disjoining_type = Some(OwnedDisjoiningDisjoiningTypeMember::FeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "OwnedDisjoining", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(OwnedDisjoining {
            span: start.merge(end),
            disjoining_type,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedDisjoining");
        _result
    }

    /// Parse `Unioning`
    pub fn parse_unioning(&mut self) -> Result<Unioning> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Unioning") {
            return Err(ParseError { message: "left-recursive entry into Unioning".into(), span: self.current_span() });
        }
        self.push_rule_context("Unioning", _entry_pos);
        let _result: Result<Unioning> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let mut unioning_type = None;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            unioning_type = Some(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Unioning", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            owned_related_element.push(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Unioning", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(Unioning {
            span: start.merge(end),
            owned_related_element,
            unioning_type,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Unioning");
        _result
    }

    /// Parse `Intersecting`
    pub fn parse_intersecting(&mut self) -> Result<Intersecting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Intersecting") {
            return Err(ParseError { message: "left-recursive entry into Intersecting".into(), span: self.current_span() });
        }
        self.push_rule_context("Intersecting", _entry_pos);
        let _result: Result<Intersecting> = (|| {
        let start = self.current_span();
        let mut intersecting_type = None;
        let mut owned_related_element = Vec::new();
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            intersecting_type = Some(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Intersecting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            owned_related_element.push(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Intersecting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(Intersecting {
            span: start.merge(end),
            intersecting_type,
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Intersecting");
        _result
    }

    /// Parse `Differencing`
    pub fn parse_differencing(&mut self) -> Result<Differencing> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Differencing") {
            return Err(ParseError { message: "left-recursive entry into Differencing".into(), span: self.current_span() });
        }
        self.push_rule_context("Differencing", _entry_pos);
        let _result: Result<Differencing> = (|| {
        let start = self.current_span();
        let mut differencing_type = None;
        let mut owned_related_element = Vec::new();
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            differencing_type = Some(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Differencing", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            owned_related_element.push(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "Differencing", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(Differencing {
            span: start.merge(end),
            differencing_type,
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Differencing");
        _result
    }

    /// Parse `FeatureMember`
    pub fn parse_feature_member(&mut self) -> Result<FeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureMember") {
            return Err(ParseError { message: "left-recursive entry into FeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureMember", _entry_pos);
        let _result: Result<FeatureMember> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(FeatureMember, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_type_feature_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureMember::TypeFeatureMember(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_owned_feature_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureMember::OwnedFeatureMember(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected FeatureMember".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureMember");
        _result
    }

    /// Parse `TypeFeatureMember`
    pub fn parse_type_feature_member(&mut self) -> Result<TypeFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into TypeFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeFeatureMember", _entry_pos);
        let _result: Result<TypeFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.parse_member_prefix()?;
        self.expect(TokenKind::Member)?;
        let v = self.parse_feature_element()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(TypeFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeFeatureMember");
        _result
    }

    /// Parse `OwnedFeatureMember`
    pub fn parse_owned_feature_member(&mut self) -> Result<OwnedFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into OwnedFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedFeatureMember", _entry_pos);
        let _result: Result<OwnedFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.parse_member_prefix()?;
        let v = self.parse_feature_element()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(OwnedFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedFeatureMember");
        _result
    }

    /// Parse `Classifier`
    pub fn parse_classifier(&mut self) -> Result<Classifier> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Classifier") {
            return Err(ParseError { message: "left-recursive entry into Classifier".into(), span: self.current_span() });
        }
        self.push_rule_context("Classifier", _entry_pos);
        let _result: Result<Classifier> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Classifier)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Classifier {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Classifier");
        _result
    }

    /// Parse `ClassifierDeclaration`
    pub fn parse_classifier_declaration(&mut self) -> Result<ClassifierDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ClassifierDeclaration") {
            return Err(ParseError { message: "left-recursive entry into ClassifierDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("ClassifierDeclaration", _entry_pos);
        let _result: Result<ClassifierDeclaration> = (|| {
        let start = self.current_span();
        let mut is_sufficient = false;
        let mut owned_relationship = Vec::new();
        let mut type_relationship_part = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::All)?;
            is_sufficient = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_identification()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = self.parse_owned_multiplicity()?;
            owned_relationship.push(v);
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.parse_superclassing_part()?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.parse_conjugation_part()?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_type_relationship_part()?;
                type_relationship_part.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            type_relationship_part.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(ClassifierDeclaration {
            span: start.merge(end),
            is_sufficient,
            owned_relationship,
            type_relationship_part,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ClassifierDeclaration");
        _result
    }

    /// Parse `SuperclassingPart`
    pub fn parse_superclassing_part(&mut self) -> Result<SuperclassingPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SuperclassingPart") {
            return Err(ParseError { message: "left-recursive entry into SuperclassingPart".into(), span: self.current_span() });
        }
        self.push_rule_context("SuperclassingPart", _entry_pos);
        let _result: Result<SuperclassingPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Specializes)?;
            Ok(())
        })?;
        let v = self.parse_owned_subclassification()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_subclassification()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(SuperclassingPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SuperclassingPart");
        _result
    }

    /// Parse `Subclassification`
    pub fn parse_subclassification(&mut self) -> Result<Subclassification> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Subclassification") {
            return Err(ParseError { message: "left-recursive entry into Subclassification".into(), span: self.current_span() });
        }
        self.push_rule_context("Subclassification", _entry_pos);
        let _result: Result<Subclassification> = (|| {
        let start = self.current_span();
        let mut subclassifier_opt: Option<_> = None;
        let mut superclassifier_opt: Option<_> = None;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Specialization)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Subclassifier)?;
        let v = self.parse_cross_ref()?;
        subclassifier_opt = Some(v);
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Specializes)?;
            Ok(())
        })?;
        let v = self.parse_cross_ref()?;
        superclassifier_opt = Some(v);
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Subclassification {
            span: start.merge(end),
            subclassifier: subclassifier_opt.ok_or_else(|| ParseError { message: "missing subclassifier".into(), span: start })?,
            superclassifier: superclassifier_opt.ok_or_else(|| ParseError { message: "missing superclassifier".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Subclassification");
        _result
    }

    /// Parse `OwnedSubclassification`
    pub fn parse_owned_subclassification(&mut self) -> Result<OwnedSubclassification> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedSubclassification") {
            return Err(ParseError { message: "left-recursive entry into OwnedSubclassification".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedSubclassification", _entry_pos);
        let _result: Result<OwnedSubclassification> = (|| {
        let start = self.current_span();
        let mut superclassifier_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        superclassifier_opt = Some(v);

        let end = self.current_span();
        Ok(OwnedSubclassification {
            span: start.merge(end),
            superclassifier: superclassifier_opt.ok_or_else(|| ParseError { message: "missing superclassifier".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedSubclassification");
        _result
    }

    /// Parse `Feature`
    pub fn parse_feature(&mut self) -> Result<Feature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Feature") {
            return Err(ParseError { message: "left-recursive entry into Feature".into(), span: self.current_span() });
        }
        self.push_rule_context("Feature", _entry_pos);
        let _result: Result<Feature> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_prefix()?;
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Feature)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(v);
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_feature_declaration()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.parse_end_feature_prefix()?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.parse_basic_feature_prefix()?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            self.parse_feature_declaration()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_value_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Feature {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Feature");
        _result
    }

    /// Parse `EndFeaturePrefix`
    pub fn parse_end_feature_prefix(&mut self) -> Result<EndFeaturePrefix> {
        let _entry_pos = self.pos;
        if !self.enter_rule("EndFeaturePrefix") {
            return Err(ParseError { message: "left-recursive entry into EndFeaturePrefix".into(), span: self.current_span() });
        }
        self.push_rule_context("EndFeaturePrefix", _entry_pos);
        let _result: Result<EndFeaturePrefix> = (|| {
        let start = self.current_span();
        let mut is_constant = false;
        let mut is_end = false;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Const)?;
            is_constant = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::End)?;
        is_end = true;

        let end = self.current_span();
        Ok(EndFeaturePrefix {
            span: start.merge(end),
            is_constant,
            is_end,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "EndFeaturePrefix");
        _result
    }

    /// Parse `BasicFeaturePrefix`
    pub fn parse_basic_feature_prefix(&mut self) -> Result<BasicFeaturePrefix> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BasicFeaturePrefix") {
            return Err(ParseError { message: "left-recursive entry into BasicFeaturePrefix".into(), span: self.current_span() });
        }
        self.push_rule_context("BasicFeaturePrefix", _entry_pos);
        let _result: Result<BasicFeaturePrefix> = (|| {
        let start = self.current_span();
        let mut direction = None;
        let mut is_abstract = false;
        let mut is_composite = false;
        let mut is_constant = false;
        let mut is_derived = false;
        let mut is_portion = false;
        let mut is_variable = false;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = self.parse_feature_direction()?;
            direction = Some(Box::new(v));
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Derived)?;
            is_derived = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Abstract)?;
            is_abstract = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Composite)?;
                is_composite = true;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Portion)?;
                is_portion = true;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Var)?;
                is_variable = true;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Const)?;
                is_constant = true;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });

        let end = self.current_span();
        Ok(BasicFeaturePrefix {
            span: start.merge(end),
            direction,
            is_abstract,
            is_composite,
            is_constant,
            is_derived,
            is_portion,
            is_variable,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BasicFeaturePrefix");
        _result
    }

    /// Parse `FeaturePrefix`
    pub fn parse_feature_prefix(&mut self) -> Result<FeaturePrefix> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeaturePrefix") {
            return Err(ParseError { message: "left-recursive entry into FeaturePrefix".into(), span: self.current_span() });
        }
        self.push_rule_context("FeaturePrefix", _entry_pos);
        let _result: Result<FeaturePrefix> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_end_feature_prefix()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_owned_cross_feature_member()?;
                owned_relationship.push(FeaturePrefixOwnedRelationshipMember::OwnedCrossFeatureMember(Box::new(v)));
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_basic_feature_prefix()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(FeaturePrefixOwnedRelationshipMember::PrefixMetadataMember(Box::new(v)));
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(FeaturePrefix {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeaturePrefix");
        _result
    }

    /// Parse `OwnedCrossFeatureMember`
    pub fn parse_owned_cross_feature_member(&mut self) -> Result<OwnedCrossFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedCrossFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into OwnedCrossFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedCrossFeatureMember", _entry_pos);
        let _result: Result<OwnedCrossFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_owned_cross_feature()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(OwnedCrossFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedCrossFeatureMember");
        _result
    }

    /// Parse `OwnedCrossFeature`
    pub fn parse_owned_cross_feature(&mut self) -> Result<OwnedCrossFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedCrossFeature") {
            return Err(ParseError { message: "left-recursive entry into OwnedCrossFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedCrossFeature", _entry_pos);
        let _result: Result<OwnedCrossFeature> = (|| {
        let start = self.current_span();
        self.parse_basic_feature_prefix()?;
        self.parse_feature_declaration()?;

        let end = self.current_span();
        Ok(OwnedCrossFeature {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedCrossFeature");
        _result
    }

    /// Parse `FeatureDirection`
    pub fn parse_feature_direction(&mut self) -> Result<FeatureDirection> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureDirection") {
            return Err(ParseError { message: "left-recursive entry into FeatureDirection".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureDirection", _entry_pos);
        let _result: Result<FeatureDirection> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::In)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Out)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Inout)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(FeatureDirection {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureDirection");
        _result
    }

    /// Parse `FeatureDeclaration`
    pub fn parse_feature_declaration(&mut self) -> Result<FeatureDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureDeclaration") {
            return Err(ParseError { message: "left-recursive entry into FeatureDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureDeclaration", _entry_pos);
        let _result: Result<FeatureDeclaration> = (|| {
        let start = self.current_span();
        let mut feature_relationship_part = Vec::new();
        let mut is_sufficient = false;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::All)?;
            is_sufficient = true;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_identification()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let saved_alt = self.save();
                let mut best_alt_pos: Option<usize> = None;
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.parse_feature_specialization_part()?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.parse_conjugation_part()?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                match best_alt_pos {
                    Some(pos) => self.pos = pos,
                    None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
                }
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_specialization_part()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_conjugation_part()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_feature_relationship_part()?;
                feature_relationship_part.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            feature_relationship_part.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(FeatureDeclaration {
            span: start.merge(end),
            feature_relationship_part,
            is_sufficient,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureDeclaration");
        _result
    }

    /// Parse `FeatureIdentification`
    pub fn parse_feature_identification(&mut self) -> Result<FeatureIdentification> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureIdentification") {
            return Err(ParseError { message: "left-recursive entry into FeatureIdentification".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureIdentification", _entry_pos);
        let _result: Result<FeatureIdentification> = (|| {
        let start = self.current_span();
        let mut declared_name = None;
        let mut declared_short_name = None;
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Lt)?;
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            declared_short_name = Some(v);
            self.expect(TokenKind::Gt)?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let v = match self.current() {
                    Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                    Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                    None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
                };
                declared_name = Some(v);
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            declared_name = Some(v);
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(FeatureIdentification {
            span: start.merge(end),
            declared_name,
            declared_short_name,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureIdentification");
        _result
    }

    /// Parse `FeatureRelationshipPart`
    pub fn parse_feature_relationship_part(&mut self) -> Result<FeatureRelationshipPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureRelationshipPart") {
            return Err(ParseError { message: "left-recursive entry into FeatureRelationshipPart".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureRelationshipPart", _entry_pos);
        let _result: Result<FeatureRelationshipPart> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(FeatureRelationshipPart, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_type_relationship_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureRelationshipPart::TypeRelationshipPart(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_chaining_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureRelationshipPart::ChainingPart(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_inverting_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureRelationshipPart::InvertingPart(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_type_featuring_part() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureRelationshipPart::TypeFeaturingPart(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected FeatureRelationshipPart".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureRelationshipPart");
        _result
    }

    /// Parse `ChainingPart`
    pub fn parse_chaining_part(&mut self) -> Result<ChainingPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ChainingPart") {
            return Err(ParseError { message: "left-recursive entry into ChainingPart".into(), span: self.current_span() });
        }
        self.push_rule_context("ChainingPart", _entry_pos);
        let _result: Result<ChainingPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Chains)?;
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chaining()?;
            owned_relationship.push(v);
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_chain()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(ChainingPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ChainingPart");
        _result
    }

    /// Parse `InvertingPart`
    pub fn parse_inverting_part(&mut self) -> Result<InvertingPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("InvertingPart") {
            return Err(ParseError { message: "left-recursive entry into InvertingPart".into(), span: self.current_span() });
        }
        self.push_rule_context("InvertingPart", _entry_pos);
        let _result: Result<InvertingPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Inverse)?;
        self.expect(TokenKind::Of)?;
        let v = self.parse_owned_feature_inverting()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(InvertingPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "InvertingPart");
        _result
    }

    /// Parse `TypeFeaturingPart`
    pub fn parse_type_featuring_part(&mut self) -> Result<TypeFeaturingPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeFeaturingPart") {
            return Err(ParseError { message: "left-recursive entry into TypeFeaturingPart".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeFeaturingPart", _entry_pos);
        let _result: Result<TypeFeaturingPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let mut owned_type_featuring = Vec::new();
        self.expect(TokenKind::Featured)?;
        self.expect(TokenKind::By)?;
        let v = self.parse_owned_type_featuring()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_type_featuring()?;
                owned_type_featuring.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(TypeFeaturingPart {
            span: start.merge(end),
            owned_relationship,
            owned_type_featuring,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeFeaturingPart");
        _result
    }

    /// Parse `FeatureSpecializationPart`
    pub fn parse_feature_specialization_part(&mut self) -> Result<FeatureSpecializationPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureSpecializationPart") {
            return Err(ParseError { message: "left-recursive entry into FeatureSpecializationPart".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureSpecializationPart", _entry_pos);
        let _result: Result<FeatureSpecializationPart> = (|| {
        let start = self.current_span();
        let mut feature_specialization = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_specialization()?;
            feature_specialization.push(v);
            loop {
                let saved = self.save();
                let body_ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_feature_specialization()?;
                    feature_specialization.push(v);
                    Ok(())
                })();
                if body_ok.is_err() {
                    self.restore(saved);
                    break; // Loop body failed, exit
                }
                if self.save() == saved { break; } // No progress, exit
                // Loop body succeeded - check if remainder can still parse
                let pos_after_body = self.save();
                let remainder_ok = (|| -> std::result::Result<(), ParseError> {
                    let saved = self.save();
                    let _: std::result::Result<(), ParseError> = (|| {
                        self.parse_multiplicity_part()?;
                        Ok(())
                    })().map_err(|e| { self.restore(saved); e });
                    loop {
                        let saved = self.save();
                        let ok: std::result::Result<(), ParseError> = (|| {
                            self.parse_feature_specialization()?;
                            Ok(())
                        })();
                        if ok.is_err() { self.restore(saved); break; }
                        if self.save() == saved { break; }
                    }
                    Ok(())
                })().is_ok();
                self.restore(pos_after_body);
                if !remainder_ok {
                    // Remainder can't parse after consuming this iteration
                    // Backtrack and leave input for remainder
                    feature_specialization.pop();
                    self.restore(saved);
                    break;
                }
                // Remainder can parse, keep going
            }
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_multiplicity_part()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_feature_specialization()?;
                    feature_specialization.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                feature_specialization.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_multiplicity_part()?;
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_feature_specialization()?;
                    feature_specialization.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                feature_specialization.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(FeatureSpecializationPart {
            span: start.merge(end),
            feature_specialization,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureSpecializationPart");
        _result
    }

    /// Parse `MultiplicityPart`
    pub fn parse_multiplicity_part(&mut self) -> Result<MultiplicityPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MultiplicityPart") {
            return Err(ParseError { message: "left-recursive entry into MultiplicityPart".into(), span: self.current_span() });
        }
        self.push_rule_context("MultiplicityPart", _entry_pos);
        let _result: Result<MultiplicityPart> = (|| {
        let start = self.current_span();
        let mut is_ordered = false;
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_owned_multiplicity()?;
                owned_relationship.push(v);
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Ordered)?;
                is_ordered = true;
                let saved = self.save();
                let _: std::result::Result<(), ParseError> = (|| {
                    self.expect(TokenKind::Nonunique)?;
                    Ok(())
                })().map_err(|e| { self.restore(saved); e });
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Nonunique)?;
                let saved = self.save();
                let _: std::result::Result<(), ParseError> = (|| {
                    self.expect(TokenKind::Ordered)?;
                    is_ordered = true;
                    Ok(())
                })().map_err(|e| { self.restore(saved); e });
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_multiplicity()?;
            owned_relationship.push(v);
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(MultiplicityPart {
            span: start.merge(end),
            is_ordered,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MultiplicityPart");
        _result
    }

    /// Parse `FeatureSpecialization`
    pub fn parse_feature_specialization(&mut self) -> Result<FeatureSpecialization> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureSpecialization") {
            return Err(ParseError { message: "left-recursive entry into FeatureSpecialization".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureSpecialization", _entry_pos);
        let _result: Result<FeatureSpecialization> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(FeatureSpecialization, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_typings() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureSpecialization::Typings(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_subsettings() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureSpecialization::Subsettings(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_references() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureSpecialization::References(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_crosses() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureSpecialization::Crosses(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_redefinitions() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureSpecialization::Redefinitions(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected FeatureSpecialization".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureSpecialization");
        _result
    }

    /// Parse `Typings`
    pub fn parse_typings(&mut self) -> Result<Typings> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Typings") {
            return Err(ParseError { message: "left-recursive entry into Typings".into(), span: self.current_span() });
        }
        self.push_rule_context("Typings", _entry_pos);
        let _result: Result<Typings> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.parse_typed_by()?;
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_feature_typing()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(Typings {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Typings");
        _result
    }

    /// Parse `TypedBy`
    pub fn parse_typed_by(&mut self) -> Result<TypedBy> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypedBy") {
            return Err(ParseError { message: "left-recursive entry into TypedBy".into(), span: self.current_span() });
        }
        self.push_rule_context("TypedBy", _entry_pos);
        let _result: Result<TypedBy> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Colon)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Typed)?;
            self.expect(TokenKind::By)?;
            Ok(())
        })?;
        let v = self.parse_owned_feature_typing()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(TypedBy {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypedBy");
        _result
    }

    /// Parse `Subsettings`
    pub fn parse_subsettings(&mut self) -> Result<Subsettings> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Subsettings") {
            return Err(ParseError { message: "left-recursive entry into Subsettings".into(), span: self.current_span() });
        }
        self.push_rule_context("Subsettings", _entry_pos);
        let _result: Result<Subsettings> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.parse_subsets()?;
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_subsetting()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(Subsettings {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Subsettings");
        _result
    }

    /// Parse `Subsets`
    pub fn parse_subsets(&mut self) -> Result<Subsets> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Subsets") {
            return Err(ParseError { message: "left-recursive entry into Subsets".into(), span: self.current_span() });
        }
        self.push_rule_context("Subsets", _entry_pos);
        let _result: Result<Subsets> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Subsets)?;
            Ok(())
        })?;
        let v = self.parse_owned_subsetting()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(Subsets {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Subsets");
        _result
    }

    /// Parse `References`
    pub fn parse_references(&mut self) -> Result<References> {
        let _entry_pos = self.pos;
        if !self.enter_rule("References") {
            return Err(ParseError { message: "left-recursive entry into References".into(), span: self.current_span() });
        }
        self.push_rule_context("References", _entry_pos);
        let _result: Result<References> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::References)?;
            Ok(())
        })?;
        let v = self.parse_owned_reference_subsetting()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(References {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "References");
        _result
    }

    /// Parse `Crosses`
    pub fn parse_crosses(&mut self) -> Result<Crosses> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Crosses") {
            return Err(ParseError { message: "left-recursive entry into Crosses".into(), span: self.current_span() });
        }
        self.push_rule_context("Crosses", _entry_pos);
        let _result: Result<Crosses> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::FatArrow)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Crosses)?;
            Ok(())
        })?;
        let v = self.parse_owned_cross_subsetting()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(Crosses {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Crosses");
        _result
    }

    /// Parse `Redefinitions`
    pub fn parse_redefinitions(&mut self) -> Result<Redefinitions> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Redefinitions") {
            return Err(ParseError { message: "left-recursive entry into Redefinitions".into(), span: self.current_span() });
        }
        self.push_rule_context("Redefinitions", _entry_pos);
        let _result: Result<Redefinitions> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.parse_redefines()?;
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_owned_redefinition()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(Redefinitions {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Redefinitions");
        _result
    }

    /// Parse `Redefines`
    pub fn parse_redefines(&mut self) -> Result<Redefines> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Redefines") {
            return Err(ParseError { message: "left-recursive entry into Redefines".into(), span: self.current_span() });
        }
        self.push_rule_context("Redefines", _entry_pos);
        let _result: Result<Redefines> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGtGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Redefines)?;
            Ok(())
        })?;
        let v = self.parse_owned_redefinition()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(Redefines {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Redefines");
        _result
    }

    /// Parse `FeatureTyping`
    pub fn parse_feature_typing(&mut self) -> Result<FeatureTyping> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureTyping") {
            return Err(ParseError { message: "left-recursive entry into FeatureTyping".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureTyping", _entry_pos);
        let _result: Result<FeatureTyping> = (|| {
        let start = self.current_span();
        let mut typed_feature_opt: Option<_> = None;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Specialization)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Typing)?;
        let v = self.parse_cross_ref()?;
        typed_feature_opt = Some(v);
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Colon)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Typed)?;
            self.expect(TokenKind::By)?;
            Ok(())
        })?;
        self.parse_general_type()?;
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(FeatureTyping {
            span: start.merge(end),
            typed_feature: typed_feature_opt.ok_or_else(|| ParseError { message: "missing typed_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureTyping");
        _result
    }

    /// Parse `OwnedFeatureTyping`
    pub fn parse_owned_feature_typing(&mut self) -> Result<OwnedFeatureTyping> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedFeatureTyping") {
            return Err(ParseError { message: "left-recursive entry into OwnedFeatureTyping".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedFeatureTyping", _entry_pos);
        let _result: Result<OwnedFeatureTyping> = (|| {
        let start = self.current_span();
        self.parse_general_type()?;

        let end = self.current_span();
        Ok(OwnedFeatureTyping {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedFeatureTyping");
        _result
    }

    /// Parse `Subsetting`
    pub fn parse_subsetting(&mut self) -> Result<Subsetting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Subsetting") {
            return Err(ParseError { message: "left-recursive entry into Subsetting".into(), span: self.current_span() });
        }
        self.push_rule_context("Subsetting", _entry_pos);
        let _result: Result<Subsetting> = (|| {
        let start = self.current_span();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Specialization)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Subset)?;
        self.parse_specific_type()?;
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Subsets)?;
            Ok(())
        })?;
        self.parse_general_type()?;
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Subsetting {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Subsetting");
        _result
    }

    /// Parse `OwnedSubsetting`
    pub fn parse_owned_subsetting(&mut self) -> Result<OwnedSubsetting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedSubsetting") {
            return Err(ParseError { message: "left-recursive entry into OwnedSubsetting".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedSubsetting", _entry_pos);
        let _result: Result<OwnedSubsetting> = (|| {
        let start = self.current_span();
        self.parse_general_type()?;

        let end = self.current_span();
        Ok(OwnedSubsetting {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedSubsetting");
        _result
    }

    /// Parse `OwnedReferenceSubsetting`
    pub fn parse_owned_reference_subsetting(&mut self) -> Result<OwnedReferenceSubsetting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedReferenceSubsetting") {
            return Err(ParseError { message: "left-recursive entry into OwnedReferenceSubsetting".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedReferenceSubsetting", _entry_pos);
        let _result: Result<OwnedReferenceSubsetting> = (|| {
        let start = self.current_span();
        self.parse_general_type()?;

        let end = self.current_span();
        Ok(OwnedReferenceSubsetting {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedReferenceSubsetting");
        _result
    }

    /// Parse `OwnedCrossSubsetting`
    pub fn parse_owned_cross_subsetting(&mut self) -> Result<OwnedCrossSubsetting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedCrossSubsetting") {
            return Err(ParseError { message: "left-recursive entry into OwnedCrossSubsetting".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedCrossSubsetting", _entry_pos);
        let _result: Result<OwnedCrossSubsetting> = (|| {
        let start = self.current_span();
        self.parse_general_type()?;

        let end = self.current_span();
        Ok(OwnedCrossSubsetting {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedCrossSubsetting");
        _result
    }

    /// Parse `Redefinition`
    pub fn parse_redefinition(&mut self) -> Result<Redefinition> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Redefinition") {
            return Err(ParseError { message: "left-recursive entry into Redefinition".into(), span: self.current_span() });
        }
        self.push_rule_context("Redefinition", _entry_pos);
        let _result: Result<Redefinition> = (|| {
        let start = self.current_span();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Specialization)?;
            self.parse_identification()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Redefinition)?;
        self.parse_specific_type()?;
        let saved_lex = self.save();
        (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::ColonGtGt)?;
            Ok(())
        })().or_else(|_: ParseError| {
            self.restore(saved_lex);
            self.expect(TokenKind::Redefines)?;
            Ok(())
        })?;
        self.parse_general_type()?;
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(Redefinition {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Redefinition");
        _result
    }

    /// Parse `OwnedRedefinition`
    pub fn parse_owned_redefinition(&mut self) -> Result<OwnedRedefinition> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedRedefinition") {
            return Err(ParseError { message: "left-recursive entry into OwnedRedefinition".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedRedefinition", _entry_pos);
        let _result: Result<OwnedRedefinition> = (|| {
        let start = self.current_span();
        self.parse_general_type()?;

        let end = self.current_span();
        Ok(OwnedRedefinition {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedRedefinition");
        _result
    }

    /// Parse `OwnedFeatureChain`
    pub fn parse_owned_feature_chain(&mut self) -> Result<OwnedFeatureChain> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedFeatureChain") {
            return Err(ParseError { message: "left-recursive entry into OwnedFeatureChain".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedFeatureChain", _entry_pos);
        let _result: Result<OwnedFeatureChain> = (|| {
        let start = self.current_span();
        self.parse_feature_chain()?;

        let end = self.current_span();
        Ok(OwnedFeatureChain {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedFeatureChain");
        _result
    }

    /// Parse `FeatureChain`
    pub fn parse_feature_chain(&mut self) -> Result<FeatureChain> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureChain") {
            return Err(ParseError { message: "left-recursive entry into FeatureChain".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureChain", _entry_pos);
        let _result: Result<FeatureChain> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_owned_feature_chaining()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = Vec::new();
        self.expect(TokenKind::Dot)?;
        let v = self.parse_owned_feature_chaining()?;
        owned_relationship.push(v);
        _glr_stop_positions.push(self.pos);
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Dot)?;
                let v = self.parse_owned_feature_chaining()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(FeatureChain {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureChain");
        _result
    }

    /// Parse `OwnedFeatureChaining`
    pub fn parse_owned_feature_chaining(&mut self) -> Result<OwnedFeatureChaining> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedFeatureChaining") {
            return Err(ParseError { message: "left-recursive entry into OwnedFeatureChaining".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedFeatureChaining", _entry_pos);
        let _result: Result<OwnedFeatureChaining> = (|| {
        let start = self.current_span();
        let mut chaining_feature_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        chaining_feature_opt = Some(v);

        let end = self.current_span();
        Ok(OwnedFeatureChaining {
            span: start.merge(end),
            chaining_feature: chaining_feature_opt.ok_or_else(|| ParseError { message: "missing chaining_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedFeatureChaining");
        _result
    }

    /// Parse `FeatureInverting`
    pub fn parse_feature_inverting(&mut self) -> Result<FeatureInverting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureInverting") {
            return Err(ParseError { message: "left-recursive entry into FeatureInverting".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureInverting", _entry_pos);
        let _result: Result<FeatureInverting> = (|| {
        let start = self.current_span();
        let mut feature_inverted = None;
        let mut inverting_feature = None;
        let mut owned_related_element = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Inverting)?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_identification()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::Inverse)?;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            feature_inverted = Some(FeatureInvertingFeatureInvertedMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "FeatureInverting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            feature_inverted = Some(FeatureInvertingFeatureInvertedMember::OwnedFeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "FeatureInverting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.expect(TokenKind::Of)?;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            inverting_feature = Some(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "FeatureInverting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            owned_related_element.push(v);
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "FeatureInverting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(FeatureInverting {
            span: start.merge(end),
            feature_inverted,
            inverting_feature,
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureInverting");
        _result
    }

    /// Parse `OwnedFeatureInverting`
    pub fn parse_owned_feature_inverting(&mut self) -> Result<OwnedFeatureInverting> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedFeatureInverting") {
            return Err(ParseError { message: "left-recursive entry into OwnedFeatureInverting".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedFeatureInverting", _entry_pos);
        let _result: Result<OwnedFeatureInverting> = (|| {
        let start = self.current_span();
        let mut inverting_feature = None;
        let saved_alt = self.save();
        let mut alt_results: Vec<usize> = Vec::new();
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cross_ref()?;
            inverting_feature = Some(OwnedFeatureInvertingInvertingFeatureMember::QualifiedNameRef(v));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "OwnedFeatureInverting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_chain()?;
            inverting_feature = Some(OwnedFeatureInvertingInvertingFeatureMember::OwnedFeatureChain(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end_pos = self.pos;
            if !self.is_parse_excluded(saved_alt, "OwnedFeatureInverting", end_pos) {
                alt_results.push(end_pos);
            }
        }
        alt_results.sort_by(|a, b| b.cmp(a));
        alt_results.dedup();
        match alt_results.first() {
            Some(&pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(OwnedFeatureInverting {
            span: start.merge(end),
            inverting_feature,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedFeatureInverting");
        _result
    }

    /// Parse `TypeFeaturing`
    pub fn parse_type_featuring(&mut self) -> Result<TypeFeaturing> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeFeaturing") {
            return Err(ParseError { message: "left-recursive entry into TypeFeaturing".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeFeaturing", _entry_pos);
        let _result: Result<TypeFeaturing> = (|| {
        let start = self.current_span();
        let mut feature_of_type_opt: Option<_> = None;
        let mut featuring_type_opt: Option<_> = None;
        self.expect(TokenKind::Featuring)?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_identification()?;
            self.expect(TokenKind::Of)?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.parse_cross_ref()?;
        feature_of_type_opt = Some(v);
        self.expect(TokenKind::By)?;
        let v = self.parse_cross_ref()?;
        featuring_type_opt = Some(v);
        self.parse_relationship_body()?;

        let end = self.current_span();
        Ok(TypeFeaturing {
            span: start.merge(end),
            feature_of_type: feature_of_type_opt.ok_or_else(|| ParseError { message: "missing feature_of_type".into(), span: start })?,
            featuring_type: featuring_type_opt.ok_or_else(|| ParseError { message: "missing featuring_type".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeFeaturing");
        _result
    }

    /// Parse `OwnedTypeFeaturing`
    pub fn parse_owned_type_featuring(&mut self) -> Result<OwnedTypeFeaturing> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedTypeFeaturing") {
            return Err(ParseError { message: "left-recursive entry into OwnedTypeFeaturing".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedTypeFeaturing", _entry_pos);
        let _result: Result<OwnedTypeFeaturing> = (|| {
        let start = self.current_span();
        let mut featuring_type_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        featuring_type_opt = Some(v);

        let end = self.current_span();
        Ok(OwnedTypeFeaturing {
            span: start.merge(end),
            featuring_type: featuring_type_opt.ok_or_else(|| ParseError { message: "missing featuring_type".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedTypeFeaturing");
        _result
    }

    /// Parse `DataType`
    pub fn parse_data_type(&mut self) -> Result<DataType> {
        let _entry_pos = self.pos;
        if !self.enter_rule("DataType") {
            return Err(ParseError { message: "left-recursive entry into DataType".into(), span: self.current_span() });
        }
        self.push_rule_context("DataType", _entry_pos);
        let _result: Result<DataType> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Datatype)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(DataType {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "DataType");
        _result
    }

    /// Parse `Class`
    pub fn parse_class(&mut self) -> Result<Class> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Class") {
            return Err(ParseError { message: "left-recursive entry into Class".into(), span: self.current_span() });
        }
        self.push_rule_context("Class", _entry_pos);
        let _result: Result<Class> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Class)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Class {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Class");
        _result
    }

    /// Parse `Structure`
    pub fn parse_structure(&mut self) -> Result<Structure> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Structure") {
            return Err(ParseError { message: "left-recursive entry into Structure".into(), span: self.current_span() });
        }
        self.push_rule_context("Structure", _entry_pos);
        let _result: Result<Structure> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Struct)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Structure {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Structure");
        _result
    }

    /// Parse `Association`
    pub fn parse_association(&mut self) -> Result<Association> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Association") {
            return Err(ParseError { message: "left-recursive entry into Association".into(), span: self.current_span() });
        }
        self.push_rule_context("Association", _entry_pos);
        let _result: Result<Association> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Assoc)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Association {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Association");
        _result
    }

    /// Parse `AssociationStructure`
    pub fn parse_association_structure(&mut self) -> Result<AssociationStructure> {
        let _entry_pos = self.pos;
        if !self.enter_rule("AssociationStructure") {
            return Err(ParseError { message: "left-recursive entry into AssociationStructure".into(), span: self.current_span() });
        }
        self.push_rule_context("AssociationStructure", _entry_pos);
        let _result: Result<AssociationStructure> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Assoc)?;
        self.expect(TokenKind::Struct)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(AssociationStructure {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "AssociationStructure");
        _result
    }

    /// Parse `Connector`
    pub fn parse_connector(&mut self) -> Result<Connector> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Connector") {
            return Err(ParseError { message: "left-recursive entry into Connector".into(), span: self.current_span() });
        }
        self.push_rule_context("Connector", _entry_pos);
        let _result: Result<Connector> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Connector)?;
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_connector_declaration()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_feature_declaration()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_value_part()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Connector {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Connector");
        _result
    }

    /// Parse `ConnectorDeclaration`
    pub fn parse_connector_declaration(&mut self) -> Result<ConnectorDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConnectorDeclaration") {
            return Err(ParseError { message: "left-recursive entry into ConnectorDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("ConnectorDeclaration", _entry_pos);
        let _result: Result<ConnectorDeclaration> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(ConnectorDeclaration, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_binary_connector_declaration() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((ConnectorDeclaration::BinaryConnectorDeclaration(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_nary_connector_declaration() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((ConnectorDeclaration::NaryConnectorDeclaration(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected ConnectorDeclaration".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConnectorDeclaration");
        _result
    }

    /// Parse `BinaryConnectorDeclaration`
    pub fn parse_binary_connector_declaration(&mut self) -> Result<BinaryConnectorDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BinaryConnectorDeclaration") {
            return Err(ParseError { message: "left-recursive entry into BinaryConnectorDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("BinaryConnectorDeclaration", _entry_pos);
        let _result: Result<BinaryConnectorDeclaration> = (|| {
        let start = self.current_span();
        let mut is_sufficient = false;
        let mut owned_relationship = Vec::new();
        let saved_opt = self.save();
        let mut opt_succeeded = false;
        let mut glr_attempts = 0;
        const MAX_GLR_ATTEMPTS: usize = 10;
        let mut last_pos_before_opt = saved_opt;
        loop {
            if glr_attempts >= MAX_GLR_ATTEMPTS { break; }
            glr_attempts += 1;
            self.restore(saved_opt);
            let pre_opt_pos = self.pos;
            let opt_ok: std::result::Result<(), ParseError> = (|| {
                let saved_alt = self.save();
                let mut best_alt_pos: Option<usize> = None;
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    let saved = self.save();
                    let _: std::result::Result<(), ParseError> = (|| {
                        self.parse_feature_declaration()?;
                        Ok(())
                    })().map_err(|e| { self.restore(saved); e });
                    self.expect(TokenKind::From)?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.expect(TokenKind::All)?;
                    is_sufficient = true;
                    let saved = self.save();
                    let _: std::result::Result<(), ParseError> = (|| {
                        self.expect(TokenKind::From)?;
                        Ok(())
                    })().map_err(|e| { self.restore(saved); e });
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                match best_alt_pos {
                    Some(pos) => self.pos = pos,
                    None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
                }
                Ok(())
            })();
            let post_opt_pos = self.pos;
            if opt_ok.is_err() {
                // Optional parsing failed — if we progressed, exclude and retry
                if post_opt_pos > pre_opt_pos && post_opt_pos != last_pos_before_opt {
                    last_pos_before_opt = post_opt_pos;
                    self.exclude_parse(saved_opt, "GeneralType", post_opt_pos);
                    self.exclude_parse(saved_opt, "OwnedConjugation", post_opt_pos);
                    self.exclude_parse(saved_opt, "OwnedDisjoining", post_opt_pos);
                    self.exclude_parse(saved_opt, "Unioning", post_opt_pos);
                    self.exclude_parse(saved_opt, "Intersecting", post_opt_pos);
                    self.exclude_parse(saved_opt, "Differencing", post_opt_pos);
                    self.exclude_parse(saved_opt, "OwnedFeatureInverting", post_opt_pos);
                    continue; // Retry with shorter alternative
                }
                self.restore(saved_opt);
                break; // No more alternatives
            }
            let opt_end = self.pos;
            // Probe if remainder can parse from here
            let rem_ok: std::result::Result<(), ParseError> = (|| {
                self.parse_connector_end_member()?;
                self.expect(TokenKind::To)?;
                self.parse_connector_end_member()?;
                Ok(())
            })();
            self.pos = opt_end; // Restore after probe
            if rem_ok.is_ok() {
                opt_succeeded = true;
                break;
            }
            // Remainder failed — exclude this parse result and retry
            self.exclude_parse(saved_opt, "GeneralType", opt_end);
            self.exclude_parse(saved_opt, "OwnedConjugation", opt_end);
            self.exclude_parse(saved_opt, "OwnedDisjoining", opt_end);
            self.exclude_parse(saved_opt, "Unioning", opt_end);
            self.exclude_parse(saved_opt, "Intersecting", opt_end);
            self.exclude_parse(saved_opt, "Differencing", opt_end);
            self.exclude_parse(saved_opt, "OwnedFeatureInverting", opt_end);
        }
        if !opt_succeeded {
            self.restore(saved_opt);
        }
        let v = self.parse_connector_end_member()?;
        owned_relationship.push(v);
        self.expect(TokenKind::To)?;
        let v = self.parse_connector_end_member()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(BinaryConnectorDeclaration {
            span: start.merge(end),
            is_sufficient,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BinaryConnectorDeclaration");
        _result
    }

    /// Parse `NaryConnectorDeclaration`
    pub fn parse_nary_connector_declaration(&mut self) -> Result<NaryConnectorDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NaryConnectorDeclaration") {
            return Err(ParseError { message: "left-recursive entry into NaryConnectorDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("NaryConnectorDeclaration", _entry_pos);
        let _result: Result<NaryConnectorDeclaration> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_feature_declaration()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::LParen)?;
        let v = self.parse_connector_end_member()?;
        owned_relationship.push(v);
        self.expect(TokenKind::Comma)?;
        let v = self.parse_connector_end_member()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.expect(TokenKind::RParen)?;

        let end = self.current_span();
        Ok(NaryConnectorDeclaration {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NaryConnectorDeclaration");
        _result
    }

    /// Parse `ConnectorEndMember`
    pub fn parse_connector_end_member(&mut self) -> Result<ConnectorEndMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConnectorEndMember") {
            return Err(ParseError { message: "left-recursive entry into ConnectorEndMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ConnectorEndMember", _entry_pos);
        let _result: Result<ConnectorEndMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_connector_end()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(ConnectorEndMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConnectorEndMember");
        _result
    }

    /// Parse `ConnectorEnd`
    pub fn parse_connector_end(&mut self) -> Result<ConnectorEnd> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConnectorEnd") {
            return Err(ParseError { message: "left-recursive entry into ConnectorEnd".into(), span: self.current_span() });
        }
        self.push_rule_context("ConnectorEnd", _entry_pos);
        let _result: Result<ConnectorEnd> = (|| {
        let start = self.current_span();
        let mut declared_name = None;
        let mut owned_relationship = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = self.parse_owned_cross_multiplicity_member()?;
            owned_relationship.push(ConnectorEndOwnedRelationshipMember::OwnedCrossMultiplicityMember(Box::new(v)));
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = match self.current() {
                Some(t) if t.kind.is_name_token() => { let text = t.text.clone(); self.pos += 1; text }
                Some(t) => return Err(ParseError { message: format!("expected name, got {:?}", t.kind), span: t.span }),
                None => return Err(ParseError { message: "expected name, got EOF".into(), span: Span::default() }),
            };
            declared_name = Some(v);
            let saved_lex = self.save();
            (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::ColonColonGt)?;
                Ok(())
            })().or_else(|_: ParseError| {
                self.restore(saved_lex);
                self.expect(TokenKind::References)?;
                Ok(())
            })?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.parse_owned_reference_subsetting()?;
        owned_relationship.push(ConnectorEndOwnedRelationshipMember::OwnedReferenceSubsetting(Box::new(v)));

        let end = self.current_span();
        Ok(ConnectorEnd {
            span: start.merge(end),
            declared_name,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConnectorEnd");
        _result
    }

    /// Parse `OwnedCrossMultiplicityMember`
    pub fn parse_owned_cross_multiplicity_member(&mut self) -> Result<OwnedCrossMultiplicityMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedCrossMultiplicityMember") {
            return Err(ParseError { message: "left-recursive entry into OwnedCrossMultiplicityMember".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedCrossMultiplicityMember", _entry_pos);
        let _result: Result<OwnedCrossMultiplicityMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_owned_cross_multiplicity()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(OwnedCrossMultiplicityMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedCrossMultiplicityMember");
        _result
    }

    /// Parse `OwnedCrossMultiplicity`
    pub fn parse_owned_cross_multiplicity(&mut self) -> Result<OwnedCrossMultiplicity> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedCrossMultiplicity") {
            return Err(ParseError { message: "left-recursive entry into OwnedCrossMultiplicity".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedCrossMultiplicity", _entry_pos);
        let _result: Result<OwnedCrossMultiplicity> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_owned_multiplicity()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(OwnedCrossMultiplicity {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedCrossMultiplicity");
        _result
    }

    /// Parse `BindingConnector`
    pub fn parse_binding_connector(&mut self) -> Result<BindingConnector> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BindingConnector") {
            return Err(ParseError { message: "left-recursive entry into BindingConnector".into(), span: self.current_span() });
        }
        self.push_rule_context("BindingConnector", _entry_pos);
        let _result: Result<BindingConnector> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Binding)?;
        self.parse_binding_connector_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(BindingConnector {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BindingConnector");
        _result
    }

    /// Parse `BindingConnectorDeclaration`
    pub fn parse_binding_connector_declaration(&mut self) -> Result<BindingConnectorDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BindingConnectorDeclaration") {
            return Err(ParseError { message: "left-recursive entry into BindingConnectorDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("BindingConnectorDeclaration", _entry_pos);
        let _result: Result<BindingConnectorDeclaration> = (|| {
        let start = self.current_span();
        let mut is_sufficient = false;
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_declaration()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Of)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                self.expect(TokenKind::Eq)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::All)?;
                is_sufficient = true;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let saved = self.save();
                let _: std::result::Result<(), ParseError> = (|| {
                    self.expect(TokenKind::Of)?;
                    Ok(())
                })().map_err(|e| { self.restore(saved); e });
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                self.expect(TokenKind::Eq)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(BindingConnectorDeclaration {
            span: start.merge(end),
            is_sufficient,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BindingConnectorDeclaration");
        _result
    }

    /// Parse `Succession`
    pub fn parse_succession(&mut self) -> Result<Succession> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Succession") {
            return Err(ParseError { message: "left-recursive entry into Succession".into(), span: self.current_span() });
        }
        self.push_rule_context("Succession", _entry_pos);
        let _result: Result<Succession> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Succession)?;
        self.parse_succession_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Succession {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Succession");
        _result
    }

    /// Parse `SuccessionDeclaration`
    pub fn parse_succession_declaration(&mut self) -> Result<SuccessionDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SuccessionDeclaration") {
            return Err(ParseError { message: "left-recursive entry into SuccessionDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("SuccessionDeclaration", _entry_pos);
        let _result: Result<SuccessionDeclaration> = (|| {
        let start = self.current_span();
        let mut is_sufficient = false;
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_declaration()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::First)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                self.expect(TokenKind::Then)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::All)?;
                is_sufficient = true;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let saved = self.save();
                let _: std::result::Result<(), ParseError> = (|| {
                    self.expect(TokenKind::First)?;
                    Ok(())
                })().map_err(|e| { self.restore(saved); e });
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                self.expect(TokenKind::Then)?;
                let v = self.parse_connector_end_member()?;
                owned_relationship.push(v);
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(SuccessionDeclaration {
            span: start.merge(end),
            is_sufficient,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SuccessionDeclaration");
        _result
    }

    /// Parse `Behavior`
    pub fn parse_behavior(&mut self) -> Result<Behavior> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Behavior") {
            return Err(ParseError { message: "left-recursive entry into Behavior".into(), span: self.current_span() });
        }
        self.push_rule_context("Behavior", _entry_pos);
        let _result: Result<Behavior> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Behavior)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Behavior {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Behavior");
        _result
    }

    /// Parse `Step`
    pub fn parse_step(&mut self) -> Result<Step> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Step") {
            return Err(ParseError { message: "left-recursive entry into Step".into(), span: self.current_span() });
        }
        self.push_rule_context("Step", _entry_pos);
        let _result: Result<Step> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Step)?;
        self.parse_feature_declaration()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_value_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Step {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Step");
        _result
    }

    /// Parse `Function`
    pub fn parse_function(&mut self) -> Result<Function> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Function") {
            return Err(ParseError { message: "left-recursive entry into Function".into(), span: self.current_span() });
        }
        self.push_rule_context("Function", _entry_pos);
        let _result: Result<Function> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Function)?;
        self.parse_classifier_declaration()?;
        self.parse_function_body()?;

        let end = self.current_span();
        Ok(Function {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Function");
        _result
    }

    /// Parse `FunctionBody`
    pub fn parse_function_body(&mut self) -> Result<FunctionBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionBody") {
            return Err(ParseError { message: "left-recursive entry into FunctionBody".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionBody", _entry_pos);
        let _result: Result<FunctionBody> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LBrace)?;
            self.parse_function_body_part()?;
            self.expect(TokenKind::RBrace)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Semi)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(FunctionBody {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionBody");
        _result
    }

    /// Parse `FunctionBodyPart`
    pub fn parse_function_body_part(&mut self) -> Result<FunctionBodyPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionBodyPart") {
            return Err(ParseError { message: "left-recursive entry into FunctionBodyPart".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionBodyPart", _entry_pos);
        let _result: Result<FunctionBodyPart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let saved_alt = self.save();
                let mut best_alt_pos: Option<usize> = None;
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.parse_type_body_element()?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    let v = self.parse_return_feature_member()?;
                    owned_relationship.push(FunctionBodyPartOwnedRelationshipMember::ReturnFeatureMember(Box::new(v)));
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                match best_alt_pos {
                    Some(pos) => self.pos = pos,
                    None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
                }
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let v = self.parse_result_expression_member()?;
            owned_relationship.push(FunctionBodyPartOwnedRelationshipMember::ResultExpressionMember(Box::new(v)));
            Ok(())
        })().map_err(|e| { self.restore(saved); e });

        let end = self.current_span();
        Ok(FunctionBodyPart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionBodyPart");
        _result
    }

    /// Parse `ReturnFeatureMember`
    pub fn parse_return_feature_member(&mut self) -> Result<ReturnFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ReturnFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into ReturnFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ReturnFeatureMember", _entry_pos);
        let _result: Result<ReturnFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.parse_member_prefix()?;
        self.expect(TokenKind::Return)?;
        let v = self.parse_feature_element()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(ReturnFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ReturnFeatureMember");
        _result
    }

    /// Parse `ResultExpressionMember`
    pub fn parse_result_expression_member(&mut self) -> Result<ResultExpressionMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ResultExpressionMember") {
            return Err(ParseError { message: "left-recursive entry into ResultExpressionMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ResultExpressionMember", _entry_pos);
        let _result: Result<ResultExpressionMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.parse_member_prefix()?;
        let v = self.parse_owned_expression()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(ResultExpressionMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ResultExpressionMember");
        _result
    }

    /// Parse `Expression`
    pub fn parse_expression(&mut self) -> Result<Expression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Expression") {
            return Err(ParseError { message: "left-recursive entry into Expression".into(), span: self.current_span() });
        }
        self.push_rule_context("Expression", _entry_pos);
        let _result: Result<Expression> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Expr)?;
        self.parse_feature_declaration()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_value_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_function_body()?;

        let end = self.current_span();
        Ok(Expression {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Expression");
        _result
    }

    /// Parse `Predicate`
    pub fn parse_predicate(&mut self) -> Result<Predicate> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Predicate") {
            return Err(ParseError { message: "left-recursive entry into Predicate".into(), span: self.current_span() });
        }
        self.push_rule_context("Predicate", _entry_pos);
        let _result: Result<Predicate> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Predicate)?;
        self.parse_classifier_declaration()?;
        self.parse_function_body()?;

        let end = self.current_span();
        Ok(Predicate {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Predicate");
        _result
    }

    /// Parse `BooleanExpression`
    pub fn parse_boolean_expression(&mut self) -> Result<BooleanExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BooleanExpression") {
            return Err(ParseError { message: "left-recursive entry into BooleanExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("BooleanExpression", _entry_pos);
        let _result: Result<BooleanExpression> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Bool)?;
        self.parse_feature_declaration()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_value_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_function_body()?;

        let end = self.current_span();
        Ok(BooleanExpression {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BooleanExpression");
        _result
    }

    /// Parse `Invariant`
    pub fn parse_invariant(&mut self) -> Result<Invariant> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Invariant") {
            return Err(ParseError { message: "left-recursive entry into Invariant".into(), span: self.current_span() });
        }
        self.push_rule_context("Invariant", _entry_pos);
        let _result: Result<Invariant> = (|| {
        let start = self.current_span();
        let mut is_negated = false;
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Inv)?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::True)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::False)?;
                is_negated = true;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_feature_declaration()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_value_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_function_body()?;

        let end = self.current_span();
        Ok(Invariant {
            span: start.merge(end),
            is_negated,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Invariant");
        _result
    }

    /// Parse `OwnedExpressionReferenceMember`
    /// Entry point
    pub fn parse_owned_expression_reference_member(&mut self) -> Result<OwnedExpressionReferenceMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedExpressionReferenceMember") {
            return Err(ParseError { message: "left-recursive entry into OwnedExpressionReferenceMember".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedExpressionReferenceMember", _entry_pos);
        let _result: Result<OwnedExpressionReferenceMember> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_owned_expression_reference()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(OwnedExpressionReferenceMember {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedExpressionReferenceMember");
        _result
    }

    /// Parse `OwnedExpressionReference`
    pub fn parse_owned_expression_reference(&mut self) -> Result<OwnedExpressionReference> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedExpressionReference") {
            return Err(ParseError { message: "left-recursive entry into OwnedExpressionReference".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedExpressionReference", _entry_pos);
        let _result: Result<OwnedExpressionReference> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_owned_expression_member()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(OwnedExpressionReference {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedExpressionReference");
        _result
    }

    /// Parse `OwnedExpressionMember`
    pub fn parse_owned_expression_member(&mut self) -> Result<OwnedExpressionMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedExpressionMember") {
            return Err(ParseError { message: "left-recursive entry into OwnedExpressionMember".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedExpressionMember", _entry_pos);
        let _result: Result<OwnedExpressionMember> = (|| {
        let start = self.current_span();
        let mut owned_feature_member_opt: Option<_> = None;
        let v = self.parse_owned_expression()?;
        owned_feature_member_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(OwnedExpressionMember {
            span: start.merge(end),
            owned_feature_member: owned_feature_member_opt.ok_or_else(|| ParseError { message: "missing owned_feature_member".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedExpressionMember");
        _result
    }

    /// Parse body of `OwnedExpression` (left-recursive helper)
    fn parse_owned_expression_body(&mut self) -> Result<OwnedExpression> {
        let alt_saved = self.save();
        let mut best: Option<(OwnedExpression, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_conditional_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::ConditionalExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_conditional_binary_operator_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::ConditionalBinaryOperatorExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_binary_operator_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::BinaryOperatorExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_unary_operator_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::UnaryOperatorExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_classification_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::ClassificationExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_metaclassification_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::MetaclassificationExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_extent_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::ExtentExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_primary_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((OwnedExpression::PrimaryExpression(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected OwnedExpression".into(), span: self.current_span() })
        }
    }

    /// Parse `OwnedExpression` (left-recursive, seed-grow)
    pub fn parse_owned_expression(&mut self) -> Result<OwnedExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "OwnedExpression")) {
            if let Some((start, end, ref result)) = self.lr_owned_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into OwnedExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "OwnedExpression"));
        self.push_rule_context("OwnedExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_owned_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_owned_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_owned_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_owned_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "OwnedExpression"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "OwnedExpression"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_owned_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "OwnedExpression"));
            result
        }
    }

    /// Parse `ConditionalExpression`
    pub fn parse_conditional_expression(&mut self) -> Result<ConditionalExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConditionalExpression") {
            return Err(ParseError { message: "left-recursive entry into ConditionalExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("ConditionalExpression", _entry_pos);
        let _result: Result<ConditionalExpression> = (|| {
        let start = self.current_span();
        let mut operator = false;
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::If)?;
        let v = self.parse_argument_member()?;
        owned_relationship.push(ConditionalExpressionOwnedRelationshipMember::ArgumentMember(Box::new(v)));
        self.expect(TokenKind::Question)?;
        let v = self.parse_argument_expression_member()?;
        owned_relationship.push(ConditionalExpressionOwnedRelationshipMember::ArgumentExpressionMember(Box::new(v)));
        self.expect(TokenKind::Else)?;
        let v = self.parse_argument_expression_member()?;
        owned_relationship.push(ConditionalExpressionOwnedRelationshipMember::ArgumentExpressionMember(Box::new(v)));
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(ConditionalExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(ConditionalExpression {
            span: start.merge(end),
            operator,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConditionalExpression");
        _result
    }

    /// Parse body of `ConditionalBinaryOperatorExpression` (left-recursive helper)
    fn parse_conditional_binary_operator_expression_body(&mut self) -> Result<ConditionalBinaryOperatorExpression> {
        let start = self.current_span();
        let mut operator_opt: Option<_> = None;
        let mut owned_relationship = Vec::new();
        let v = self.parse_argument_member()?;
        owned_relationship.push(ConditionalBinaryOperatorExpressionOwnedRelationshipMember::ArgumentMember(Box::new(v)));
        let v = self.parse_conditional_binary_operator()?;
        operator_opt = Some(Box::new(v));
        let v = self.parse_argument_expression_member()?;
        owned_relationship.push(ConditionalBinaryOperatorExpressionOwnedRelationshipMember::ArgumentExpressionMember(Box::new(v)));
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(ConditionalBinaryOperatorExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(ConditionalBinaryOperatorExpression {
            span: start.merge(end),
            operator: operator_opt.ok_or_else(|| ParseError { message: "missing operator".into(), span: start })?,
            owned_relationship,
        })
    }

    /// Parse `ConditionalBinaryOperatorExpression` (left-recursive, seed-grow)
    pub fn parse_conditional_binary_operator_expression(&mut self) -> Result<ConditionalBinaryOperatorExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "ConditionalBinaryOperatorExpression")) {
            if let Some((start, end, ref result)) = self.lr_conditional_binary_operator_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into ConditionalBinaryOperatorExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "ConditionalBinaryOperatorExpression"));
        self.push_rule_context("ConditionalBinaryOperatorExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_conditional_binary_operator_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_conditional_binary_operator_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_conditional_binary_operator_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_conditional_binary_operator_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ConditionalBinaryOperatorExpression"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ConditionalBinaryOperatorExpression"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_conditional_binary_operator_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "ConditionalBinaryOperatorExpression"));
            result
        }
    }

    /// Parse `ConditionalBinaryOperator`
    pub fn parse_conditional_binary_operator(&mut self) -> Result<ConditionalBinaryOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConditionalBinaryOperator") {
            return Err(ParseError { message: "left-recursive entry into ConditionalBinaryOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("ConditionalBinaryOperator", _entry_pos);
        let _result: Result<ConditionalBinaryOperator> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::QuestionQuestion)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Or)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::And)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Implies)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(ConditionalBinaryOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConditionalBinaryOperator");
        _result
    }

    /// Parse body of `BinaryOperatorExpression` (left-recursive helper)
    fn parse_binary_operator_expression_body(&mut self) -> Result<BinaryOperatorExpression> {
        let start = self.current_span();
        let mut operator_opt: Option<_> = None;
        let mut owned_relationship = Vec::new();
        let v = self.parse_argument_member()?;
        owned_relationship.push(BinaryOperatorExpressionOwnedRelationshipMember::ArgumentMember(Box::new(v)));
        let v = self.parse_binary_operator()?;
        operator_opt = Some(Box::new(v));
        let v = self.parse_argument_member()?;
        owned_relationship.push(BinaryOperatorExpressionOwnedRelationshipMember::ArgumentMember(Box::new(v)));
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(BinaryOperatorExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(BinaryOperatorExpression {
            span: start.merge(end),
            operator: operator_opt.ok_or_else(|| ParseError { message: "missing operator".into(), span: start })?,
            owned_relationship,
        })
    }

    /// Parse `BinaryOperatorExpression` (left-recursive, seed-grow)
    pub fn parse_binary_operator_expression(&mut self) -> Result<BinaryOperatorExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "BinaryOperatorExpression")) {
            if let Some((start, end, ref result)) = self.lr_binary_operator_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into BinaryOperatorExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "BinaryOperatorExpression"));
        self.push_rule_context("BinaryOperatorExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_binary_operator_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_binary_operator_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_binary_operator_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_binary_operator_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "BinaryOperatorExpression"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "BinaryOperatorExpression"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_binary_operator_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "BinaryOperatorExpression"));
            result
        }
    }

    /// Parse `BinaryOperator`
    pub fn parse_binary_operator(&mut self) -> Result<BinaryOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BinaryOperator") {
            return Err(ParseError { message: "left-recursive entry into BinaryOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("BinaryOperator", _entry_pos);
        let _result: Result<BinaryOperator> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Pipe)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Amp)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Xor)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::DotDot)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::EqEq)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::BangEq)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Punct3D3D3D)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Punct213D3D)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Lt)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Gt)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LtEq)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::GtEq)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Plus)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Minus)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Star)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Slash)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Percent)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Caret)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::StarStar)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(BinaryOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BinaryOperator");
        _result
    }

    /// Parse `UnaryOperatorExpression`
    pub fn parse_unary_operator_expression(&mut self) -> Result<UnaryOperatorExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("UnaryOperatorExpression") {
            return Err(ParseError { message: "left-recursive entry into UnaryOperatorExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("UnaryOperatorExpression", _entry_pos);
        let _result: Result<UnaryOperatorExpression> = (|| {
        let start = self.current_span();
        let mut operator_opt: Option<_> = None;
        let mut owned_relationship = Vec::new();
        let v = self.parse_unary_operator()?;
        operator_opt = Some(Box::new(v));
        let v = self.parse_argument_member()?;
        owned_relationship.push(UnaryOperatorExpressionOwnedRelationshipMember::ArgumentMember(Box::new(v)));
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(UnaryOperatorExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(UnaryOperatorExpression {
            span: start.merge(end),
            operator: operator_opt.ok_or_else(|| ParseError { message: "missing operator".into(), span: start })?,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "UnaryOperatorExpression");
        _result
    }

    /// Parse `UnaryOperator`
    pub fn parse_unary_operator(&mut self) -> Result<UnaryOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("UnaryOperator") {
            return Err(ParseError { message: "left-recursive entry into UnaryOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("UnaryOperator", _entry_pos);
        let _result: Result<UnaryOperator> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Plus)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Minus)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Tilde)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Not)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(UnaryOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "UnaryOperator");
        _result
    }

    /// Parse body of `ClassificationExpression` (left-recursive helper)
    fn parse_classification_expression_body(&mut self) -> Result<ClassificationExpression> {
        let start = self.current_span();
        let mut operator = None;
        let mut owned_relationship = Vec::new();
        let saved_opt = self.save();
        let mut opt_succeeded = false;
        let mut glr_attempts = 0;
        const MAX_GLR_ATTEMPTS: usize = 10;
        let mut last_pos_before_opt = saved_opt;
        loop {
            if glr_attempts >= MAX_GLR_ATTEMPTS { break; }
            glr_attempts += 1;
            self.restore(saved_opt);
            let pre_opt_pos = self.pos;
            let opt_ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_argument_member()?;
                owned_relationship.push(ClassificationExpressionOwnedRelationshipMember::ArgumentMember(Box::new(v)));
                Ok(())
            })();
            let post_opt_pos = self.pos;
            if opt_ok.is_err() {
                // Optional parsing failed — if we progressed, exclude and retry
                if post_opt_pos > pre_opt_pos && post_opt_pos != last_pos_before_opt {
                    last_pos_before_opt = post_opt_pos;
                    self.exclude_parse(saved_opt, "GeneralType", post_opt_pos);
                    self.exclude_parse(saved_opt, "OwnedConjugation", post_opt_pos);
                    self.exclude_parse(saved_opt, "OwnedDisjoining", post_opt_pos);
                    self.exclude_parse(saved_opt, "Unioning", post_opt_pos);
                    self.exclude_parse(saved_opt, "Intersecting", post_opt_pos);
                    self.exclude_parse(saved_opt, "Differencing", post_opt_pos);
                    self.exclude_parse(saved_opt, "OwnedFeatureInverting", post_opt_pos);
                    self.exclude_parse(saved_opt, "SpecificType", post_opt_pos);
                    continue; // Retry with shorter alternative
                }
                self.restore(saved_opt);
                break; // No more alternatives
            }
            let opt_end = self.pos;
            // Probe if remainder can parse from here
            let rem_ok: std::result::Result<(), ParseError> = (|| {
                let saved_probe = self.save();
                (|| -> std::result::Result<(), ParseError> {
                    self.parse_classification_test_operator()?;
                    self.parse_type_reference_member()?;
                    Ok(())
                })().or_else(|_: ParseError| {
                    self.restore(saved_probe);
                    self.parse_cast_operator()?;
                    self.parse_type_result_member()?;
                    Ok(())
                })?;
                self.parse_empty_result_member()?;
                Ok(())
            })();
            self.pos = opt_end; // Restore after probe
            if rem_ok.is_ok() {
                opt_succeeded = true;
                break;
            }
            // Remainder failed — exclude this parse result and retry
            self.exclude_parse(saved_opt, "GeneralType", opt_end);
            self.exclude_parse(saved_opt, "OwnedConjugation", opt_end);
            self.exclude_parse(saved_opt, "OwnedDisjoining", opt_end);
            self.exclude_parse(saved_opt, "Unioning", opt_end);
            self.exclude_parse(saved_opt, "Intersecting", opt_end);
            self.exclude_parse(saved_opt, "Differencing", opt_end);
            self.exclude_parse(saved_opt, "OwnedFeatureInverting", opt_end);
            self.exclude_parse(saved_opt, "SpecificType", opt_end);
        }
        if !opt_succeeded {
            self.restore(saved_opt);
        }
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_classification_test_operator()?;
            operator = Some(ClassificationExpressionOperatorMember::ClassificationTestOperator(Box::new(v)));
            let v = self.parse_type_reference_member()?;
            owned_relationship.push(ClassificationExpressionOwnedRelationshipMember::TypeReferenceMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_cast_operator()?;
            operator = Some(ClassificationExpressionOperatorMember::CastOperator(Box::new(v)));
            let v = self.parse_type_result_member()?;
            owned_relationship.push(ClassificationExpressionOwnedRelationshipMember::TypeResultMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(ClassificationExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(ClassificationExpression {
            span: start.merge(end),
            operator,
            owned_relationship,
        })
    }

    /// Parse `ClassificationExpression` (left-recursive, seed-grow)
    pub fn parse_classification_expression(&mut self) -> Result<ClassificationExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "ClassificationExpression")) {
            if let Some((start, end, ref result)) = self.lr_classification_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into ClassificationExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "ClassificationExpression"));
        self.push_rule_context("ClassificationExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_classification_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_classification_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_classification_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_classification_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ClassificationExpression"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ClassificationExpression"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_classification_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "ClassificationExpression"));
            result
        }
    }

    /// Parse `ClassificationTestOperator`
    pub fn parse_classification_test_operator(&mut self) -> Result<ClassificationTestOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ClassificationTestOperator") {
            return Err(ParseError { message: "left-recursive entry into ClassificationTestOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("ClassificationTestOperator", _entry_pos);
        let _result: Result<ClassificationTestOperator> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Istype)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Hastype)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::AtSign)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(ClassificationTestOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ClassificationTestOperator");
        _result
    }

    /// Parse `CastOperator`
    pub fn parse_cast_operator(&mut self) -> Result<CastOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("CastOperator") {
            return Err(ParseError { message: "left-recursive entry into CastOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("CastOperator", _entry_pos);
        let _result: Result<CastOperator> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::As)?;

        let end = self.current_span();
        Ok(CastOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "CastOperator");
        _result
    }

    /// Parse `MetaclassificationExpression`
    pub fn parse_metaclassification_expression(&mut self) -> Result<MetaclassificationExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetaclassificationExpression") {
            return Err(ParseError { message: "left-recursive entry into MetaclassificationExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("MetaclassificationExpression", _entry_pos);
        let _result: Result<MetaclassificationExpression> = (|| {
        let start = self.current_span();
        let mut operator = None;
        let mut owned_relationship = Vec::new();
        let v = self.parse_metadata_argument_member()?;
        owned_relationship.push(MetaclassificationExpressionOwnedRelationshipMember::MetadataArgumentMember(Box::new(v)));
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_classification_test_operator()?;
            operator = Some(MetaclassificationExpressionOperatorMember::ClassificationTestOperator(Box::new(v)));
            let v = self.parse_type_reference_member()?;
            owned_relationship.push(MetaclassificationExpressionOwnedRelationshipMember::TypeReferenceMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_meta_cast_operator()?;
            operator = Some(MetaclassificationExpressionOperatorMember::MetaCastOperator(Box::new(v)));
            let v = self.parse_type_result_member()?;
            owned_relationship.push(MetaclassificationExpressionOwnedRelationshipMember::TypeResultMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(MetaclassificationExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(MetaclassificationExpression {
            span: start.merge(end),
            operator,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetaclassificationExpression");
        _result
    }

    /// Parse body of `ArgumentMember` (left-recursive helper)
    fn parse_argument_member_body(&mut self) -> Result<ArgumentMember> {
        let start = self.current_span();
        let mut owned_member_parameter_opt: Option<_> = None;
        let v = self.parse_argument()?;
        owned_member_parameter_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(ArgumentMember {
            span: start.merge(end),
            owned_member_parameter: owned_member_parameter_opt.ok_or_else(|| ParseError { message: "missing owned_member_parameter".into(), span: start })?,
        })
    }

    /// Parse `ArgumentMember` (left-recursive, seed-grow)
    pub fn parse_argument_member(&mut self) -> Result<ArgumentMember> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "ArgumentMember")) {
            if let Some((start, end, ref result)) = self.lr_argument_member {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into ArgumentMember".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "ArgumentMember"));
        self.push_rule_context("ArgumentMember", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_argument_member_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_argument_member = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_argument_member_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_argument_member = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ArgumentMember"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ArgumentMember"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_argument_member_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "ArgumentMember"));
            result
        }
    }

    /// Parse body of `Argument` (left-recursive helper)
    fn parse_argument_body(&mut self) -> Result<Argument> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_argument_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(Argument {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `Argument` (left-recursive, seed-grow)
    pub fn parse_argument(&mut self) -> Result<Argument> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "Argument")) {
            if let Some((start, end, ref result)) = self.lr_argument {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into Argument".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "Argument"));
        self.push_rule_context("Argument", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_argument_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_argument = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_argument_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_argument = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "Argument"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "Argument"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_argument_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "Argument"));
            result
        }
    }

    /// Parse body of `ArgumentValue` (left-recursive helper)
    fn parse_argument_value_body(&mut self) -> Result<ArgumentValue> {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_owned_expression()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(ArgumentValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
    }

    /// Parse `ArgumentValue` (left-recursive, seed-grow)
    pub fn parse_argument_value(&mut self) -> Result<ArgumentValue> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "ArgumentValue")) {
            if let Some((start, end, ref result)) = self.lr_argument_value {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into ArgumentValue".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_1 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "ArgumentValue"));
        self.push_rule_context("ArgumentValue", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_1;
            self.lr_head_1 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_argument_value_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_argument_value = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_argument_value_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_argument_value = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ArgumentValue"));
                self.lr_head_1 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "ArgumentValue"));
                self.lr_head_1 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_argument_value_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "ArgumentValue"));
            result
        }
    }

    /// Parse `ArgumentExpressionMember`
    pub fn parse_argument_expression_member(&mut self) -> Result<ArgumentExpressionMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ArgumentExpressionMember") {
            return Err(ParseError { message: "left-recursive entry into ArgumentExpressionMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ArgumentExpressionMember", _entry_pos);
        let _result: Result<ArgumentExpressionMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_argument_expression()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(ArgumentExpressionMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ArgumentExpressionMember");
        _result
    }

    /// Parse `ArgumentExpression`
    pub fn parse_argument_expression(&mut self) -> Result<ArgumentExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ArgumentExpression") {
            return Err(ParseError { message: "left-recursive entry into ArgumentExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("ArgumentExpression", _entry_pos);
        let _result: Result<ArgumentExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_argument_expression_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(ArgumentExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ArgumentExpression");
        _result
    }

    /// Parse `ArgumentExpressionValue`
    pub fn parse_argument_expression_value(&mut self) -> Result<ArgumentExpressionValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ArgumentExpressionValue") {
            return Err(ParseError { message: "left-recursive entry into ArgumentExpressionValue".into(), span: self.current_span() });
        }
        self.push_rule_context("ArgumentExpressionValue", _entry_pos);
        let _result: Result<ArgumentExpressionValue> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_owned_expression_reference()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(ArgumentExpressionValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ArgumentExpressionValue");
        _result
    }

    /// Parse `MetadataArgumentMember`
    pub fn parse_metadata_argument_member(&mut self) -> Result<MetadataArgumentMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataArgumentMember") {
            return Err(ParseError { message: "left-recursive entry into MetadataArgumentMember".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataArgumentMember", _entry_pos);
        let _result: Result<MetadataArgumentMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_metadata_argument()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(MetadataArgumentMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataArgumentMember");
        _result
    }

    /// Parse `MetadataArgument`
    pub fn parse_metadata_argument(&mut self) -> Result<MetadataArgument> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataArgument") {
            return Err(ParseError { message: "left-recursive entry into MetadataArgument".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataArgument", _entry_pos);
        let _result: Result<MetadataArgument> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_metadata_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(MetadataArgument {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataArgument");
        _result
    }

    /// Parse `MetadataValue`
    pub fn parse_metadata_value(&mut self) -> Result<MetadataValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataValue") {
            return Err(ParseError { message: "left-recursive entry into MetadataValue".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataValue", _entry_pos);
        let _result: Result<MetadataValue> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_metadata_reference()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(MetadataValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataValue");
        _result
    }

    /// Parse `MetadataReference`
    pub fn parse_metadata_reference(&mut self) -> Result<MetadataReference> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataReference") {
            return Err(ParseError { message: "left-recursive entry into MetadataReference".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataReference", _entry_pos);
        let _result: Result<MetadataReference> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_element_reference_member()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(MetadataReference {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataReference");
        _result
    }

    /// Parse `MetaclassificationTestOperator`
    /// Entry point
    pub fn parse_metaclassification_test_operator(&mut self) -> Result<MetaclassificationTestOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetaclassificationTestOperator") {
            return Err(ParseError { message: "left-recursive entry into MetaclassificationTestOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("MetaclassificationTestOperator", _entry_pos);
        let _result: Result<MetaclassificationTestOperator> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Punct4040)?;

        let end = self.current_span();
        Ok(MetaclassificationTestOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetaclassificationTestOperator");
        _result
    }

    /// Parse `MetaCastOperator`
    pub fn parse_meta_cast_operator(&mut self) -> Result<MetaCastOperator> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetaCastOperator") {
            return Err(ParseError { message: "left-recursive entry into MetaCastOperator".into(), span: self.current_span() });
        }
        self.push_rule_context("MetaCastOperator", _entry_pos);
        let _result: Result<MetaCastOperator> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Meta)?;

        let end = self.current_span();
        Ok(MetaCastOperator {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetaCastOperator");
        _result
    }

    /// Parse `ExtentExpression`
    pub fn parse_extent_expression(&mut self) -> Result<ExtentExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ExtentExpression") {
            return Err(ParseError { message: "left-recursive entry into ExtentExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("ExtentExpression", _entry_pos);
        let _result: Result<ExtentExpression> = (|| {
        let start = self.current_span();
        let mut operator = false;
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::All)?;
        let v = self.parse_type_reference_member()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(ExtentExpression {
            span: start.merge(end),
            operator,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ExtentExpression");
        _result
    }

    /// Parse `TypeReferenceMember`
    pub fn parse_type_reference_member(&mut self) -> Result<TypeReferenceMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeReferenceMember") {
            return Err(ParseError { message: "left-recursive entry into TypeReferenceMember".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeReferenceMember", _entry_pos);
        let _result: Result<TypeReferenceMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_type_reference()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(TypeReferenceMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeReferenceMember");
        _result
    }

    /// Parse `TypeResultMember`
    pub fn parse_type_result_member(&mut self) -> Result<TypeResultMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeResultMember") {
            return Err(ParseError { message: "left-recursive entry into TypeResultMember".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeResultMember", _entry_pos);
        let _result: Result<TypeResultMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_type_reference()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(TypeResultMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeResultMember");
        _result
    }

    /// Parse `TypeReference`
    pub fn parse_type_reference(&mut self) -> Result<TypeReference> {
        let _entry_pos = self.pos;
        if !self.enter_rule("TypeReference") {
            return Err(ParseError { message: "left-recursive entry into TypeReference".into(), span: self.current_span() });
        }
        self.push_rule_context("TypeReference", _entry_pos);
        let _result: Result<TypeReference> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_reference_typing()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(TypeReference {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "TypeReference");
        _result
    }

    /// Parse `ReferenceTyping`
    pub fn parse_reference_typing(&mut self) -> Result<ReferenceTyping> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ReferenceTyping") {
            return Err(ParseError { message: "left-recursive entry into ReferenceTyping".into(), span: self.current_span() });
        }
        self.push_rule_context("ReferenceTyping", _entry_pos);
        let _result: Result<ReferenceTyping> = (|| {
        let start = self.current_span();
        let mut type__opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        type__opt = Some(v);

        let end = self.current_span();
        Ok(ReferenceTyping {
            span: start.merge(end),
            type_: type__opt.ok_or_else(|| ParseError { message: "missing type_".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ReferenceTyping");
        _result
    }

    /// Parse `EmptyResultMember`
    pub fn parse_empty_result_member(&mut self) -> Result<EmptyResultMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("EmptyResultMember") {
            return Err(ParseError { message: "left-recursive entry into EmptyResultMember".into(), span: self.current_span() });
        }
        self.push_rule_context("EmptyResultMember", _entry_pos);
        let _result: Result<EmptyResultMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_empty_feature()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(EmptyResultMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "EmptyResultMember");
        _result
    }

    /// Parse `EmptyFeature`
    pub fn parse_empty_feature(&mut self) -> Result<EmptyFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("EmptyFeature") {
            return Err(ParseError { message: "left-recursive entry into EmptyFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("EmptyFeature", _entry_pos);
        let _result: Result<EmptyFeature> = (|| {
        let start = self.current_span();

        let end = self.current_span();
        Ok(EmptyFeature {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "EmptyFeature");
        _result
    }

    /// Parse body of `PrimaryExpression` (left-recursive helper)
    fn parse_primary_expression_body(&mut self) -> Result<PrimaryExpression> {
        let alt_saved = self.save();
        let mut best: Option<(PrimaryExpression, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature_chain_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((PrimaryExpression::FeatureChainExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_non_feature_chain_primary_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((PrimaryExpression::NonFeatureChainPrimaryExpression(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected PrimaryExpression".into(), span: self.current_span() })
        }
    }

    /// Parse `PrimaryExpression` (left-recursive, seed-grow)
    pub fn parse_primary_expression(&mut self) -> Result<PrimaryExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "PrimaryExpression")) {
            if let Some((start, end, ref result)) = self.lr_primary_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into PrimaryExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "PrimaryExpression"));
        self.push_rule_context("PrimaryExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_primary_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_primary_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_primary_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_primary_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_primary_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "PrimaryExpression"));
            result
        }
    }

    /// Parse body of `PrimaryArgumentValue` (left-recursive helper)
    fn parse_primary_argument_value_body(&mut self) -> Result<PrimaryArgumentValue> {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_primary_expression()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(PrimaryArgumentValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
    }

    /// Parse `PrimaryArgumentValue` (left-recursive, seed-grow)
    pub fn parse_primary_argument_value(&mut self) -> Result<PrimaryArgumentValue> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "PrimaryArgumentValue")) {
            if let Some((start, end, ref result)) = self.lr_primary_argument_value {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into PrimaryArgumentValue".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "PrimaryArgumentValue"));
        self.push_rule_context("PrimaryArgumentValue", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_primary_argument_value_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_primary_argument_value = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_primary_argument_value_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_primary_argument_value = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryArgumentValue"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryArgumentValue"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_primary_argument_value_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "PrimaryArgumentValue"));
            result
        }
    }

    /// Parse body of `PrimaryArgument` (left-recursive helper)
    fn parse_primary_argument_body(&mut self) -> Result<PrimaryArgument> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_primary_argument_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(PrimaryArgument {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `PrimaryArgument` (left-recursive, seed-grow)
    pub fn parse_primary_argument(&mut self) -> Result<PrimaryArgument> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "PrimaryArgument")) {
            if let Some((start, end, ref result)) = self.lr_primary_argument {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into PrimaryArgument".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "PrimaryArgument"));
        self.push_rule_context("PrimaryArgument", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_primary_argument_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_primary_argument = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_primary_argument_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_primary_argument = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryArgument"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryArgument"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_primary_argument_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "PrimaryArgument"));
            result
        }
    }

    /// Parse body of `PrimaryArgumentMember` (left-recursive helper)
    fn parse_primary_argument_member_body(&mut self) -> Result<PrimaryArgumentMember> {
        let start = self.current_span();
        let mut owned_member_parameter_opt: Option<_> = None;
        let v = self.parse_primary_argument()?;
        owned_member_parameter_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(PrimaryArgumentMember {
            span: start.merge(end),
            owned_member_parameter: owned_member_parameter_opt.ok_or_else(|| ParseError { message: "missing owned_member_parameter".into(), span: start })?,
        })
    }

    /// Parse `PrimaryArgumentMember` (left-recursive, seed-grow)
    pub fn parse_primary_argument_member(&mut self) -> Result<PrimaryArgumentMember> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "PrimaryArgumentMember")) {
            if let Some((start, end, ref result)) = self.lr_primary_argument_member {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into PrimaryArgumentMember".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "PrimaryArgumentMember"));
        self.push_rule_context("PrimaryArgumentMember", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_primary_argument_member_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_primary_argument_member = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_primary_argument_member_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_primary_argument_member = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryArgumentMember"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "PrimaryArgumentMember"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_primary_argument_member_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "PrimaryArgumentMember"));
            result
        }
    }

    /// Parse body of `NonFeatureChainPrimaryExpression` (left-recursive helper)
    fn parse_non_feature_chain_primary_expression_body(&mut self) -> Result<NonFeatureChainPrimaryExpression> {
        let alt_saved = self.save();
        let mut best: Option<(NonFeatureChainPrimaryExpression, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_bracket_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::BracketExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_index_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::IndexExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_sequence_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::SequenceExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_select_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::SelectExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_collect_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::CollectExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_function_operation_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::FunctionOperationExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_base_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((NonFeatureChainPrimaryExpression::BaseExpression(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected NonFeatureChainPrimaryExpression".into(), span: self.current_span() })
        }
    }

    /// Parse `NonFeatureChainPrimaryExpression` (left-recursive, seed-grow)
    pub fn parse_non_feature_chain_primary_expression(&mut self) -> Result<NonFeatureChainPrimaryExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "NonFeatureChainPrimaryExpression")) {
            if let Some((start, end, ref result)) = self.lr_non_feature_chain_primary_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into NonFeatureChainPrimaryExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "NonFeatureChainPrimaryExpression"));
        self.push_rule_context("NonFeatureChainPrimaryExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_non_feature_chain_primary_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_non_feature_chain_primary_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_non_feature_chain_primary_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_non_feature_chain_primary_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "NonFeatureChainPrimaryExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "NonFeatureChainPrimaryExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_non_feature_chain_primary_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "NonFeatureChainPrimaryExpression"));
            result
        }
    }

    /// Parse `NonFeatureChainPrimaryArgumentValue`
    pub fn parse_non_feature_chain_primary_argument_value(&mut self) -> Result<NonFeatureChainPrimaryArgumentValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NonFeatureChainPrimaryArgumentValue") {
            return Err(ParseError { message: "left-recursive entry into NonFeatureChainPrimaryArgumentValue".into(), span: self.current_span() });
        }
        self.push_rule_context("NonFeatureChainPrimaryArgumentValue", _entry_pos);
        let _result: Result<NonFeatureChainPrimaryArgumentValue> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_non_feature_chain_primary_expression()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(NonFeatureChainPrimaryArgumentValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NonFeatureChainPrimaryArgumentValue");
        _result
    }

    /// Parse `NonFeatureChainPrimaryArgument`
    /// Entry point
    pub fn parse_non_feature_chain_primary_argument(&mut self) -> Result<NonFeatureChainPrimaryArgument> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NonFeatureChainPrimaryArgument") {
            return Err(ParseError { message: "left-recursive entry into NonFeatureChainPrimaryArgument".into(), span: self.current_span() });
        }
        self.push_rule_context("NonFeatureChainPrimaryArgument", _entry_pos);
        let _result: Result<NonFeatureChainPrimaryArgument> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_non_feature_chain_primary_argument_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(NonFeatureChainPrimaryArgument {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NonFeatureChainPrimaryArgument");
        _result
    }

    /// Parse body of `NonFeatureChainPrimaryArgumentMember` (left-recursive helper)
    fn parse_non_feature_chain_primary_argument_member_body(&mut self) -> Result<NonFeatureChainPrimaryArgumentMember> {
        let start = self.current_span();
        let mut owned_member_parameter_opt: Option<_> = None;
        let v = self.parse_primary_argument()?;
        owned_member_parameter_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(NonFeatureChainPrimaryArgumentMember {
            span: start.merge(end),
            owned_member_parameter: owned_member_parameter_opt.ok_or_else(|| ParseError { message: "missing owned_member_parameter".into(), span: start })?,
        })
    }

    /// Parse `NonFeatureChainPrimaryArgumentMember` (left-recursive, seed-grow)
    pub fn parse_non_feature_chain_primary_argument_member(&mut self) -> Result<NonFeatureChainPrimaryArgumentMember> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "NonFeatureChainPrimaryArgumentMember")) {
            if let Some((start, end, ref result)) = self.lr_non_feature_chain_primary_argument_member {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into NonFeatureChainPrimaryArgumentMember".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "NonFeatureChainPrimaryArgumentMember"));
        self.push_rule_context("NonFeatureChainPrimaryArgumentMember", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_non_feature_chain_primary_argument_member_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_non_feature_chain_primary_argument_member = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_non_feature_chain_primary_argument_member_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_non_feature_chain_primary_argument_member = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "NonFeatureChainPrimaryArgumentMember"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "NonFeatureChainPrimaryArgumentMember"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_non_feature_chain_primary_argument_member_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "NonFeatureChainPrimaryArgumentMember"));
            result
        }
    }

    /// Parse body of `BracketExpression` (left-recursive helper)
    fn parse_bracket_expression_body(&mut self) -> Result<BracketExpression> {
        let start = self.current_span();
        let mut operator = false;
        let mut owned_relationship = Vec::new();
        let v = self.parse_primary_argument_member()?;
        owned_relationship.push(BracketExpressionOwnedRelationshipMember::PrimaryArgumentMember(Box::new(v)));
        self.expect(TokenKind::LBracket)?;
        let v = self.parse_sequence_expression_list_member()?;
        owned_relationship.push(BracketExpressionOwnedRelationshipMember::SequenceExpressionListMember(Box::new(v)));
        self.expect(TokenKind::RBracket)?;

        let end = self.current_span();
        Ok(BracketExpression {
            span: start.merge(end),
            operator,
            owned_relationship,
        })
    }

    /// Parse `BracketExpression` (left-recursive, seed-grow)
    pub fn parse_bracket_expression(&mut self) -> Result<BracketExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "BracketExpression")) {
            if let Some((start, end, ref result)) = self.lr_bracket_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into BracketExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "BracketExpression"));
        self.push_rule_context("BracketExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_bracket_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_bracket_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_bracket_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_bracket_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "BracketExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "BracketExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_bracket_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "BracketExpression"));
            result
        }
    }

    /// Parse body of `IndexExpression` (left-recursive helper)
    fn parse_index_expression_body(&mut self) -> Result<IndexExpression> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_primary_argument_member()?;
        owned_relationship.push(IndexExpressionOwnedRelationshipMember::PrimaryArgumentMember(Box::new(v)));
        self.expect(TokenKind::Hash)?;
        self.expect(TokenKind::LParen)?;
        let v = self.parse_sequence_expression_list_member()?;
        owned_relationship.push(IndexExpressionOwnedRelationshipMember::SequenceExpressionListMember(Box::new(v)));
        self.expect(TokenKind::RParen)?;

        let end = self.current_span();
        Ok(IndexExpression {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `IndexExpression` (left-recursive, seed-grow)
    pub fn parse_index_expression(&mut self) -> Result<IndexExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "IndexExpression")) {
            if let Some((start, end, ref result)) = self.lr_index_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into IndexExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "IndexExpression"));
        self.push_rule_context("IndexExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_index_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_index_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_index_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_index_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "IndexExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "IndexExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_index_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "IndexExpression"));
            result
        }
    }

    /// Parse `SequenceExpression`
    pub fn parse_sequence_expression(&mut self) -> Result<SequenceExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SequenceExpression") {
            return Err(ParseError { message: "left-recursive entry into SequenceExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("SequenceExpression", _entry_pos);
        let _result: Result<SequenceExpression> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::LParen)?;
        self.parse_sequence_expression_list()?;
        self.expect(TokenKind::RParen)?;

        let end = self.current_span();
        Ok(SequenceExpression {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SequenceExpression");
        _result
    }

    /// Parse `SequenceExpressionList`
    pub fn parse_sequence_expression_list(&mut self) -> Result<SequenceExpressionList> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SequenceExpressionList") {
            return Err(ParseError { message: "left-recursive entry into SequenceExpressionList".into(), span: self.current_span() });
        }
        self.push_rule_context("SequenceExpressionList", _entry_pos);
        let _result: Result<SequenceExpressionList> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_owned_expression()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_sequence_operator_expression()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(SequenceExpressionList {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SequenceExpressionList");
        _result
    }

    /// Parse `SequenceOperatorExpression`
    pub fn parse_sequence_operator_expression(&mut self) -> Result<SequenceOperatorExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SequenceOperatorExpression") {
            return Err(ParseError { message: "left-recursive entry into SequenceOperatorExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("SequenceOperatorExpression", _entry_pos);
        let _result: Result<SequenceOperatorExpression> = (|| {
        let start = self.current_span();
        let mut operator = false;
        let mut owned_relationship = Vec::new();
        let v = self.parse_owned_expression_member()?;
        owned_relationship.push(SequenceOperatorExpressionOwnedRelationshipMember::OwnedExpressionMember(Box::new(v)));
        self.expect(TokenKind::Comma)?;
        let v = self.parse_sequence_expression_list_member()?;
        owned_relationship.push(SequenceOperatorExpressionOwnedRelationshipMember::SequenceExpressionListMember(Box::new(v)));

        let end = self.current_span();
        Ok(SequenceOperatorExpression {
            span: start.merge(end),
            operator,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SequenceOperatorExpression");
        _result
    }

    /// Parse `SequenceExpressionListMember`
    pub fn parse_sequence_expression_list_member(&mut self) -> Result<SequenceExpressionListMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SequenceExpressionListMember") {
            return Err(ParseError { message: "left-recursive entry into SequenceExpressionListMember".into(), span: self.current_span() });
        }
        self.push_rule_context("SequenceExpressionListMember", _entry_pos);
        let _result: Result<SequenceExpressionListMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_sequence_expression_list()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(SequenceExpressionListMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SequenceExpressionListMember");
        _result
    }

    /// Parse body of `FeatureChainExpression` (left-recursive helper)
    fn parse_feature_chain_expression_body(&mut self) -> Result<FeatureChainExpression> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_non_feature_chain_primary_argument_member()?;
        owned_relationship.push(FeatureChainExpressionOwnedRelationshipMember::NonFeatureChainPrimaryArgumentMember(Box::new(v)));
        self.expect(TokenKind::Dot)?;
        let v = self.parse_feature_chain_member()?;
        owned_relationship.push(FeatureChainExpressionOwnedRelationshipMember::FeatureChainMember(Box::new(v)));

        let end = self.current_span();
        Ok(FeatureChainExpression {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `FeatureChainExpression` (left-recursive, seed-grow)
    pub fn parse_feature_chain_expression(&mut self) -> Result<FeatureChainExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "FeatureChainExpression")) {
            if let Some((start, end, ref result)) = self.lr_feature_chain_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into FeatureChainExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "FeatureChainExpression"));
        self.push_rule_context("FeatureChainExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_feature_chain_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_feature_chain_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_feature_chain_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_feature_chain_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "FeatureChainExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "FeatureChainExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_feature_chain_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "FeatureChainExpression"));
            result
        }
    }

    /// Parse body of `CollectExpression` (left-recursive helper)
    fn parse_collect_expression_body(&mut self) -> Result<CollectExpression> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_primary_argument_member()?;
        owned_relationship.push(CollectExpressionOwnedRelationshipMember::PrimaryArgumentMember(Box::new(v)));
        self.expect(TokenKind::Dot)?;
        let v = self.parse_body_argument_member()?;
        owned_relationship.push(CollectExpressionOwnedRelationshipMember::BodyArgumentMember(Box::new(v)));

        let end = self.current_span();
        Ok(CollectExpression {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `CollectExpression` (left-recursive, seed-grow)
    pub fn parse_collect_expression(&mut self) -> Result<CollectExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "CollectExpression")) {
            if let Some((start, end, ref result)) = self.lr_collect_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into CollectExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "CollectExpression"));
        self.push_rule_context("CollectExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_collect_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_collect_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_collect_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_collect_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "CollectExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "CollectExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_collect_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "CollectExpression"));
            result
        }
    }

    /// Parse body of `SelectExpression` (left-recursive helper)
    fn parse_select_expression_body(&mut self) -> Result<SelectExpression> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_primary_argument_member()?;
        owned_relationship.push(SelectExpressionOwnedRelationshipMember::PrimaryArgumentMember(Box::new(v)));
        self.expect(TokenKind::DotQuestion)?;
        let v = self.parse_body_argument_member()?;
        owned_relationship.push(SelectExpressionOwnedRelationshipMember::BodyArgumentMember(Box::new(v)));

        let end = self.current_span();
        Ok(SelectExpression {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `SelectExpression` (left-recursive, seed-grow)
    pub fn parse_select_expression(&mut self) -> Result<SelectExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "SelectExpression")) {
            if let Some((start, end, ref result)) = self.lr_select_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into SelectExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "SelectExpression"));
        self.push_rule_context("SelectExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_select_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_select_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_select_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_select_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "SelectExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "SelectExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_select_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "SelectExpression"));
            result
        }
    }

    /// Parse body of `FunctionOperationExpression` (left-recursive helper)
    fn parse_function_operation_expression_body(&mut self) -> Result<FunctionOperationExpression> {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_primary_argument_member()?;
        owned_relationship.push(FunctionOperationExpressionOwnedRelationshipMember::PrimaryArgumentMember(Box::new(v)));
        self.expect(TokenKind::Arrow)?;
        let v = self.parse_invocation_type_member()?;
        owned_relationship.push(FunctionOperationExpressionOwnedRelationshipMember::InvocationTypeMember(Box::new(v)));
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_body_argument_member()?;
            owned_relationship.push(FunctionOperationExpressionOwnedRelationshipMember::BodyArgumentMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_function_reference_argument_member()?;
            owned_relationship.push(FunctionOperationExpressionOwnedRelationshipMember::FunctionReferenceArgumentMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_argument_list()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(FunctionOperationExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(FunctionOperationExpression {
            span: start.merge(end),
            owned_relationship,
        })
    }

    /// Parse `FunctionOperationExpression` (left-recursive, seed-grow)
    pub fn parse_function_operation_expression(&mut self) -> Result<FunctionOperationExpression> {
        let _entry_pos = self.pos;

        // Left-recursive re-entry: return memo if available
        if self.visiting.contains(&(self.pos, "FunctionOperationExpression")) {
            if let Some((start, end, ref result)) = self.lr_function_operation_expression {
                if start == _entry_pos {
                    self.pos = end;
                    return Ok(result.clone());
                }
            }
            return Err(ParseError { message: "left-recursive entry into FunctionOperationExpression".into(), span: self.current_span() });
        }

        let is_head = self.lr_head_0 != Some(_entry_pos);
        self.visiting.insert((_entry_pos, "FunctionOperationExpression"));
        self.push_rule_context("FunctionOperationExpression", _entry_pos);

        if is_head {
            // We are the LR head at this position
            let prev_head_pos = self.lr_head_0;
            self.lr_head_0 = Some(_entry_pos);

            // Seed phase
            let seed = self.parse_function_operation_expression_body();

            if let Ok(seed_val) = seed {
                let mut best = seed_val;
                let mut best_pos = self.pos;

                // Grow loop
                loop {
                    self.lr_function_operation_expression = Some((_entry_pos, best_pos, best.clone()));
                    self.pos = _entry_pos;
                    match self.parse_function_operation_expression_body() {
                        Ok(grown) if self.pos > best_pos => {
                            best = grown;
                            best_pos = self.pos;
                        }
                        _ => {
                            self.pos = best_pos;
                            break;
                        }
                    }
                }

                self.lr_function_operation_expression = None;
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "FunctionOperationExpression"));
                self.lr_head_0 = prev_head_pos;
                Ok(best)
            } else {
                self.pop_rule_context();
                self.visiting.remove(&(_entry_pos, "FunctionOperationExpression"));
                self.lr_head_0 = prev_head_pos;
                seed
            }
        } else {
            // Another LR rule is head at this position — just parse (no grow loop)
            let result = self.parse_function_operation_expression_body();
            self.pop_rule_context();
            self.visiting.remove(&(_entry_pos, "FunctionOperationExpression"));
            result
        }
    }

    /// Parse `BodyArgumentMember`
    pub fn parse_body_argument_member(&mut self) -> Result<BodyArgumentMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BodyArgumentMember") {
            return Err(ParseError { message: "left-recursive entry into BodyArgumentMember".into(), span: self.current_span() });
        }
        self.push_rule_context("BodyArgumentMember", _entry_pos);
        let _result: Result<BodyArgumentMember> = (|| {
        let start = self.current_span();
        let mut owned_member_parameter_opt: Option<_> = None;
        let v = self.parse_body_argument()?;
        owned_member_parameter_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(BodyArgumentMember {
            span: start.merge(end),
            owned_member_parameter: owned_member_parameter_opt.ok_or_else(|| ParseError { message: "missing owned_member_parameter".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BodyArgumentMember");
        _result
    }

    /// Parse `BodyArgument`
    pub fn parse_body_argument(&mut self) -> Result<BodyArgument> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BodyArgument") {
            return Err(ParseError { message: "left-recursive entry into BodyArgument".into(), span: self.current_span() });
        }
        self.push_rule_context("BodyArgument", _entry_pos);
        let _result: Result<BodyArgument> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_body_argument_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(BodyArgument {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BodyArgument");
        _result
    }

    /// Parse `BodyArgumentValue`
    pub fn parse_body_argument_value(&mut self) -> Result<BodyArgumentValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BodyArgumentValue") {
            return Err(ParseError { message: "left-recursive entry into BodyArgumentValue".into(), span: self.current_span() });
        }
        self.push_rule_context("BodyArgumentValue", _entry_pos);
        let _result: Result<BodyArgumentValue> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_body_expression()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(BodyArgumentValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BodyArgumentValue");
        _result
    }

    /// Parse `FunctionReferenceArgumentMember`
    pub fn parse_function_reference_argument_member(&mut self) -> Result<FunctionReferenceArgumentMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionReferenceArgumentMember") {
            return Err(ParseError { message: "left-recursive entry into FunctionReferenceArgumentMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionReferenceArgumentMember", _entry_pos);
        let _result: Result<FunctionReferenceArgumentMember> = (|| {
        let start = self.current_span();
        let mut owned_member_parameter_opt: Option<_> = None;
        let v = self.parse_function_reference_argument()?;
        owned_member_parameter_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(FunctionReferenceArgumentMember {
            span: start.merge(end),
            owned_member_parameter: owned_member_parameter_opt.ok_or_else(|| ParseError { message: "missing owned_member_parameter".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionReferenceArgumentMember");
        _result
    }

    /// Parse `FunctionReferenceArgument`
    pub fn parse_function_reference_argument(&mut self) -> Result<FunctionReferenceArgument> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionReferenceArgument") {
            return Err(ParseError { message: "left-recursive entry into FunctionReferenceArgument".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionReferenceArgument", _entry_pos);
        let _result: Result<FunctionReferenceArgument> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_function_reference_argument_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(FunctionReferenceArgument {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionReferenceArgument");
        _result
    }

    /// Parse `FunctionReferenceArgumentValue`
    pub fn parse_function_reference_argument_value(&mut self) -> Result<FunctionReferenceArgumentValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionReferenceArgumentValue") {
            return Err(ParseError { message: "left-recursive entry into FunctionReferenceArgumentValue".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionReferenceArgumentValue", _entry_pos);
        let _result: Result<FunctionReferenceArgumentValue> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_function_reference_expression()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(FunctionReferenceArgumentValue {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionReferenceArgumentValue");
        _result
    }

    /// Parse `FunctionReferenceExpression`
    pub fn parse_function_reference_expression(&mut self) -> Result<FunctionReferenceExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionReferenceExpression") {
            return Err(ParseError { message: "left-recursive entry into FunctionReferenceExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionReferenceExpression", _entry_pos);
        let _result: Result<FunctionReferenceExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_function_reference_member()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(FunctionReferenceExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionReferenceExpression");
        _result
    }

    /// Parse `FunctionReferenceMember`
    pub fn parse_function_reference_member(&mut self) -> Result<FunctionReferenceMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionReferenceMember") {
            return Err(ParseError { message: "left-recursive entry into FunctionReferenceMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionReferenceMember", _entry_pos);
        let _result: Result<FunctionReferenceMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_function_reference()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(FunctionReferenceMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionReferenceMember");
        _result
    }

    /// Parse `FunctionReference`
    pub fn parse_function_reference(&mut self) -> Result<FunctionReference> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FunctionReference") {
            return Err(ParseError { message: "left-recursive entry into FunctionReference".into(), span: self.current_span() });
        }
        self.push_rule_context("FunctionReference", _entry_pos);
        let _result: Result<FunctionReference> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_reference_typing()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(FunctionReference {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FunctionReference");
        _result
    }

    /// Parse `FeatureChainMember`
    pub fn parse_feature_chain_member(&mut self) -> Result<FeatureChainMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureChainMember") {
            return Err(ParseError { message: "left-recursive entry into FeatureChainMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureChainMember", _entry_pos);
        let _result: Result<FeatureChainMember> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(FeatureChainMember, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature_reference_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureChainMember::FeatureReferenceMember(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_owned_feature_chain_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((FeatureChainMember::OwnedFeatureChainMember(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected FeatureChainMember".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureChainMember");
        _result
    }

    /// Parse `OwnedFeatureChainMember`
    pub fn parse_owned_feature_chain_member(&mut self) -> Result<OwnedFeatureChainMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedFeatureChainMember") {
            return Err(ParseError { message: "left-recursive entry into OwnedFeatureChainMember".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedFeatureChainMember", _entry_pos);
        let _result: Result<OwnedFeatureChainMember> = (|| {
        let start = self.current_span();
        let mut owned_member_element_opt: Option<_> = None;
        let v = self.parse_feature_chain()?;
        owned_member_element_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(OwnedFeatureChainMember {
            span: start.merge(end),
            owned_member_element: owned_member_element_opt.ok_or_else(|| ParseError { message: "missing owned_member_element".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedFeatureChainMember");
        _result
    }

    /// Parse `BaseExpression`
    pub fn parse_base_expression(&mut self) -> Result<BaseExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BaseExpression") {
            return Err(ParseError { message: "left-recursive entry into BaseExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("BaseExpression", _entry_pos);
        let _result: Result<BaseExpression> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(BaseExpression, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_null_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::NullExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_literal_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::LiteralExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_feature_reference_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::FeatureReferenceExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_metadata_access_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::MetadataAccessExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_invocation_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::InvocationExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_constructor_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::ConstructorExpression(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_body_expression() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((BaseExpression::BodyExpression(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected BaseExpression".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BaseExpression");
        _result
    }

    /// Parse `NullExpression`
    pub fn parse_null_expression(&mut self) -> Result<NullExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NullExpression") {
            return Err(ParseError { message: "left-recursive entry into NullExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("NullExpression", _entry_pos);
        let _result: Result<NullExpression> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LParen)?;
            self.expect(TokenKind::RParen)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Null)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(NullExpression {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NullExpression");
        _result
    }

    /// Parse `FeatureReferenceExpression`
    pub fn parse_feature_reference_expression(&mut self) -> Result<FeatureReferenceExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureReferenceExpression") {
            return Err(ParseError { message: "left-recursive entry into FeatureReferenceExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureReferenceExpression", _entry_pos);
        let _result: Result<FeatureReferenceExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_feature_reference_member()?;
        owned_relationship.push(FeatureReferenceExpressionOwnedRelationshipMember::FeatureReferenceMember(Box::new(v)));
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(FeatureReferenceExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(FeatureReferenceExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureReferenceExpression");
        _result
    }

    /// Parse `FeatureReferenceMember`
    pub fn parse_feature_reference_member(&mut self) -> Result<FeatureReferenceMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureReferenceMember") {
            return Err(ParseError { message: "left-recursive entry into FeatureReferenceMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureReferenceMember", _entry_pos);
        let _result: Result<FeatureReferenceMember> = (|| {
        let start = self.current_span();
        let mut member_element_opt: Option<_> = None;
        let v = self.parse_feature_reference()?;
        member_element_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(FeatureReferenceMember {
            span: start.merge(end),
            member_element: member_element_opt.ok_or_else(|| ParseError { message: "missing member_element".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureReferenceMember");
        _result
    }

    /// Parse `FeatureReference`
    pub fn parse_feature_reference(&mut self) -> Result<FeatureReference> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureReference") {
            return Err(ParseError { message: "left-recursive entry into FeatureReference".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureReference", _entry_pos);
        let _result: Result<FeatureReference> = (|| {
        let start = self.current_span();
        self.parse_cross_ref()?;

        let end = self.current_span();
        Ok(FeatureReference {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureReference");
        _result
    }

    /// Parse `MetadataAccessExpression`
    pub fn parse_metadata_access_expression(&mut self) -> Result<MetadataAccessExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataAccessExpression") {
            return Err(ParseError { message: "left-recursive entry into MetadataAccessExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataAccessExpression", _entry_pos);
        let _result: Result<MetadataAccessExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_element_reference_member()?;
        owned_relationship.push(v);
        self.expect(TokenKind::Dot)?;
        self.expect(TokenKind::Metadata)?;

        let end = self.current_span();
        Ok(MetadataAccessExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataAccessExpression");
        _result
    }

    /// Parse `ElementReferenceMember`
    pub fn parse_element_reference_member(&mut self) -> Result<ElementReferenceMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ElementReferenceMember") {
            return Err(ParseError { message: "left-recursive entry into ElementReferenceMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ElementReferenceMember", _entry_pos);
        let _result: Result<ElementReferenceMember> = (|| {
        let start = self.current_span();
        let mut member_element_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        member_element_opt = Some(v);

        let end = self.current_span();
        Ok(ElementReferenceMember {
            span: start.merge(end),
            member_element: member_element_opt.ok_or_else(|| ParseError { message: "missing member_element".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ElementReferenceMember");
        _result
    }

    /// Parse `InvocationExpression`
    pub fn parse_invocation_expression(&mut self) -> Result<InvocationExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("InvocationExpression") {
            return Err(ParseError { message: "left-recursive entry into InvocationExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("InvocationExpression", _entry_pos);
        let _result: Result<InvocationExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_instantiated_type_member()?;
        owned_relationship.push(InvocationExpressionOwnedRelationshipMember::InstantiatedTypeMember(Box::new(v)));
        self.parse_argument_list()?;
        let v = self.parse_empty_result_member()?;
        owned_relationship.push(InvocationExpressionOwnedRelationshipMember::EmptyResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(InvocationExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "InvocationExpression");
        _result
    }

    /// Parse `ConstructorExpression`
    pub fn parse_constructor_expression(&mut self) -> Result<ConstructorExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConstructorExpression") {
            return Err(ParseError { message: "left-recursive entry into ConstructorExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("ConstructorExpression", _entry_pos);
        let _result: Result<ConstructorExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::New)?;
        let v = self.parse_instantiated_type_member()?;
        owned_relationship.push(ConstructorExpressionOwnedRelationshipMember::InstantiatedTypeMember(Box::new(v)));
        let v = self.parse_constructor_result_member()?;
        owned_relationship.push(ConstructorExpressionOwnedRelationshipMember::ConstructorResultMember(Box::new(v)));

        let end = self.current_span();
        Ok(ConstructorExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConstructorExpression");
        _result
    }

    /// Parse `ConstructorResultMember`
    pub fn parse_constructor_result_member(&mut self) -> Result<ConstructorResultMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConstructorResultMember") {
            return Err(ParseError { message: "left-recursive entry into ConstructorResultMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ConstructorResultMember", _entry_pos);
        let _result: Result<ConstructorResultMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_constructor_result()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(ConstructorResultMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConstructorResultMember");
        _result
    }

    /// Parse `ConstructorResult`
    pub fn parse_constructor_result(&mut self) -> Result<ConstructorResult> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ConstructorResult") {
            return Err(ParseError { message: "left-recursive entry into ConstructorResult".into(), span: self.current_span() });
        }
        self.push_rule_context("ConstructorResult", _entry_pos);
        let _result: Result<ConstructorResult> = (|| {
        let start = self.current_span();
        self.parse_argument_list()?;

        let end = self.current_span();
        Ok(ConstructorResult {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ConstructorResult");
        _result
    }

    /// Parse `InstantiatedTypeMember`
    pub fn parse_instantiated_type_member(&mut self) -> Result<InstantiatedTypeMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("InstantiatedTypeMember") {
            return Err(ParseError { message: "left-recursive entry into InstantiatedTypeMember".into(), span: self.current_span() });
        }
        self.push_rule_context("InstantiatedTypeMember", _entry_pos);
        let _result: Result<InstantiatedTypeMember> = (|| {
        let start = self.current_span();
        let mut member_element = None;
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_instantiated_type_reference()?;
            member_element = Some(Box::new(v));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_owned_feature_chain_member()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(InstantiatedTypeMember {
            span: start.merge(end),
            member_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "InstantiatedTypeMember");
        _result
    }

    /// Parse `InstantiatedTypeReference`
    pub fn parse_instantiated_type_reference(&mut self) -> Result<InstantiatedTypeReference> {
        let _entry_pos = self.pos;
        if !self.enter_rule("InstantiatedTypeReference") {
            return Err(ParseError { message: "left-recursive entry into InstantiatedTypeReference".into(), span: self.current_span() });
        }
        self.push_rule_context("InstantiatedTypeReference", _entry_pos);
        let _result: Result<InstantiatedTypeReference> = (|| {
        let start = self.current_span();
        self.parse_cross_ref()?;

        let end = self.current_span();
        Ok(InstantiatedTypeReference {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "InstantiatedTypeReference");
        _result
    }

    /// Parse `ArgumentList`
    pub fn parse_argument_list(&mut self) -> Result<ArgumentList> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ArgumentList") {
            return Err(ParseError { message: "left-recursive entry into ArgumentList".into(), span: self.current_span() });
        }
        self.push_rule_context("ArgumentList", _entry_pos);
        let _result: Result<ArgumentList> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::LParen)?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.parse_positional_argument_list()?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.parse_named_argument_list()?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.expect(TokenKind::RParen)?;

        let end = self.current_span();
        Ok(ArgumentList {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ArgumentList");
        _result
    }

    /// Parse `PositionalArgumentList`
    pub fn parse_positional_argument_list(&mut self) -> Result<PositionalArgumentList> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PositionalArgumentList") {
            return Err(ParseError { message: "left-recursive entry into PositionalArgumentList".into(), span: self.current_span() });
        }
        self.push_rule_context("PositionalArgumentList", _entry_pos);
        let _result: Result<PositionalArgumentList> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_argument_member()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_argument_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(PositionalArgumentList {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PositionalArgumentList");
        _result
    }

    /// Parse `NamedArgumentList`
    pub fn parse_named_argument_list(&mut self) -> Result<NamedArgumentList> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamedArgumentList") {
            return Err(ParseError { message: "left-recursive entry into NamedArgumentList".into(), span: self.current_span() });
        }
        self.push_rule_context("NamedArgumentList", _entry_pos);
        let _result: Result<NamedArgumentList> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_named_argument_member()?;
        owned_relationship.push(v);
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Comma)?;
                let v = self.parse_named_argument_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }

        let end = self.current_span();
        Ok(NamedArgumentList {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamedArgumentList");
        _result
    }

    /// Parse `NamedArgumentMember`
    pub fn parse_named_argument_member(&mut self) -> Result<NamedArgumentMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamedArgumentMember") {
            return Err(ParseError { message: "left-recursive entry into NamedArgumentMember".into(), span: self.current_span() });
        }
        self.push_rule_context("NamedArgumentMember", _entry_pos);
        let _result: Result<NamedArgumentMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_named_argument()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(NamedArgumentMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamedArgumentMember");
        _result
    }

    /// Parse `NamedArgument`
    pub fn parse_named_argument(&mut self) -> Result<NamedArgument> {
        let _entry_pos = self.pos;
        if !self.enter_rule("NamedArgument") {
            return Err(ParseError { message: "left-recursive entry into NamedArgument".into(), span: self.current_span() });
        }
        self.push_rule_context("NamedArgument", _entry_pos);
        let _result: Result<NamedArgument> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_parameter_redefinition()?;
        owned_relationship.push(NamedArgumentOwnedRelationshipMember::ParameterRedefinition(Box::new(v)));
        self.expect(TokenKind::Eq)?;
        let v = self.parse_argument_value()?;
        owned_relationship.push(NamedArgumentOwnedRelationshipMember::ArgumentValue(Box::new(v)));

        let end = self.current_span();
        Ok(NamedArgument {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "NamedArgument");
        _result
    }

    /// Parse `ParameterRedefinition`
    pub fn parse_parameter_redefinition(&mut self) -> Result<ParameterRedefinition> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ParameterRedefinition") {
            return Err(ParseError { message: "left-recursive entry into ParameterRedefinition".into(), span: self.current_span() });
        }
        self.push_rule_context("ParameterRedefinition", _entry_pos);
        let _result: Result<ParameterRedefinition> = (|| {
        let start = self.current_span();
        let mut redefined_feature_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        redefined_feature_opt = Some(v);

        let end = self.current_span();
        Ok(ParameterRedefinition {
            span: start.merge(end),
            redefined_feature: redefined_feature_opt.ok_or_else(|| ParseError { message: "missing redefined_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ParameterRedefinition");
        _result
    }

    /// Parse `BodyExpression`
    pub fn parse_body_expression(&mut self) -> Result<BodyExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BodyExpression") {
            return Err(ParseError { message: "left-recursive entry into BodyExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("BodyExpression", _entry_pos);
        let _result: Result<BodyExpression> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_expression_body_member()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(BodyExpression {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BodyExpression");
        _result
    }

    /// Parse `ExpressionBodyMember`
    pub fn parse_expression_body_member(&mut self) -> Result<ExpressionBodyMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ExpressionBodyMember") {
            return Err(ParseError { message: "left-recursive entry into ExpressionBodyMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ExpressionBodyMember", _entry_pos);
        let _result: Result<ExpressionBodyMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_expression_body()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(ExpressionBodyMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ExpressionBodyMember");
        _result
    }

    /// Parse `ExpressionBody`
    pub fn parse_expression_body(&mut self) -> Result<ExpressionBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ExpressionBody") {
            return Err(ParseError { message: "left-recursive entry into ExpressionBody".into(), span: self.current_span() });
        }
        self.push_rule_context("ExpressionBody", _entry_pos);
        let _result: Result<ExpressionBody> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::LBrace)?;
        self.parse_function_body_part()?;
        self.expect(TokenKind::RBrace)?;

        let end = self.current_span();
        Ok(ExpressionBody {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ExpressionBody");
        _result
    }

    /// Parse `LiteralExpression`
    pub fn parse_literal_expression(&mut self) -> Result<LiteralExpression> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LiteralExpression") {
            return Err(ParseError { message: "left-recursive entry into LiteralExpression".into(), span: self.current_span() });
        }
        self.push_rule_context("LiteralExpression", _entry_pos);
        let _result: Result<LiteralExpression> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(LiteralExpression, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_literal_boolean() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((LiteralExpression::LiteralBoolean(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_literal_string() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((LiteralExpression::LiteralString(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_literal_integer() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((LiteralExpression::LiteralInteger(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_literal_real() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((LiteralExpression::LiteralReal(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_literal_infinity() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((LiteralExpression::LiteralInfinity(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected LiteralExpression".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LiteralExpression");
        _result
    }

    /// Parse `LiteralBoolean`
    pub fn parse_literal_boolean(&mut self) -> Result<LiteralBoolean> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LiteralBoolean") {
            return Err(ParseError { message: "left-recursive entry into LiteralBoolean".into(), span: self.current_span() });
        }
        self.push_rule_context("LiteralBoolean", _entry_pos);
        let _result: Result<LiteralBoolean> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_boolean_value()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(LiteralBoolean {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LiteralBoolean");
        _result
    }

    /// Parse `BooleanValue`
    pub fn parse_boolean_value(&mut self) -> Result<BooleanValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("BooleanValue") {
            return Err(ParseError { message: "left-recursive entry into BooleanValue".into(), span: self.current_span() });
        }
        self.push_rule_context("BooleanValue", _entry_pos);
        let _result: Result<BooleanValue> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::True)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::False)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(BooleanValue {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "BooleanValue");
        _result
    }

    /// Parse `LiteralString`
    pub fn parse_literal_string(&mut self) -> Result<LiteralString> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LiteralString") {
            return Err(ParseError { message: "left-recursive entry into LiteralString".into(), span: self.current_span() });
        }
        self.push_rule_context("LiteralString", _entry_pos);
        let _result: Result<LiteralString> = (|| {
        let start = self.current_span();
        let mut value = String::new();
        let v = self.expect(TokenKind::String)?.text.clone();
        value = v;

        let end = self.current_span();
        Ok(LiteralString {
            span: start.merge(end),
            value,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LiteralString");
        _result
    }

    /// Parse `LiteralInteger`
    pub fn parse_literal_integer(&mut self) -> Result<LiteralInteger> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LiteralInteger") {
            return Err(ParseError { message: "left-recursive entry into LiteralInteger".into(), span: self.current_span() });
        }
        self.push_rule_context("LiteralInteger", _entry_pos);
        let _result: Result<LiteralInteger> = (|| {
        let start = self.current_span();
        let mut value = String::new();
        let v = self.expect(TokenKind::Integer)?.text.clone();
        value = v;

        let end = self.current_span();
        Ok(LiteralInteger {
            span: start.merge(end),
            value,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LiteralInteger");
        _result
    }

    /// Parse `LiteralReal`
    pub fn parse_literal_real(&mut self) -> Result<LiteralReal> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LiteralReal") {
            return Err(ParseError { message: "left-recursive entry into LiteralReal".into(), span: self.current_span() });
        }
        self.push_rule_context("LiteralReal", _entry_pos);
        let _result: Result<LiteralReal> = (|| {
        let start = self.current_span();
        let mut value_opt: Option<_> = None;
        let v = self.parse_real_value()?;
        value_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(LiteralReal {
            span: start.merge(end),
            value: value_opt.ok_or_else(|| ParseError { message: "missing value".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LiteralReal");
        _result
    }

    /// Parse `RealValue`
    pub fn parse_real_value(&mut self) -> Result<RealValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("RealValue") {
            return Err(ParseError { message: "left-recursive entry into RealValue".into(), span: self.current_span() });
        }
        self.push_rule_context("RealValue", _entry_pos);
        let _result: Result<RealValue> = (|| {
        let start = self.current_span();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Integer)?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            self.expect(TokenKind::Dot)?;
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Integer)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Real)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Real)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(RealValue {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "RealValue");
        _result
    }

    /// Parse `LiteralInfinity`
    pub fn parse_literal_infinity(&mut self) -> Result<LiteralInfinity> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LiteralInfinity") {
            return Err(ParseError { message: "left-recursive entry into LiteralInfinity".into(), span: self.current_span() });
        }
        self.push_rule_context("LiteralInfinity", _entry_pos);
        let _result: Result<LiteralInfinity> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Star)?;

        let end = self.current_span();
        Ok(LiteralInfinity {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LiteralInfinity");
        _result
    }

    /// Parse `Interaction`
    pub fn parse_interaction(&mut self) -> Result<Interaction> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Interaction") {
            return Err(ParseError { message: "left-recursive entry into Interaction".into(), span: self.current_span() });
        }
        self.push_rule_context("Interaction", _entry_pos);
        let _result: Result<Interaction> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Interaction)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Interaction {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Interaction");
        _result
    }

    /// Parse `Flow`
    pub fn parse_flow(&mut self) -> Result<Flow> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Flow") {
            return Err(ParseError { message: "left-recursive entry into Flow".into(), span: self.current_span() });
        }
        self.push_rule_context("Flow", _entry_pos);
        let _result: Result<Flow> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Flow)?;
        self.parse_flow_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Flow {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Flow");
        _result
    }

    /// Parse `SuccessionFlow`
    pub fn parse_succession_flow(&mut self) -> Result<SuccessionFlow> {
        let _entry_pos = self.pos;
        if !self.enter_rule("SuccessionFlow") {
            return Err(ParseError { message: "left-recursive entry into SuccessionFlow".into(), span: self.current_span() });
        }
        self.push_rule_context("SuccessionFlow", _entry_pos);
        let _result: Result<SuccessionFlow> = (|| {
        let start = self.current_span();
        self.parse_feature_prefix()?;
        self.expect(TokenKind::Succession)?;
        self.expect(TokenKind::Flow)?;
        self.parse_flow_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(SuccessionFlow {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "SuccessionFlow");
        _result
    }

    /// Parse `FlowDeclaration`
    pub fn parse_flow_declaration(&mut self) -> Result<FlowDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FlowDeclaration") {
            return Err(ParseError { message: "left-recursive entry into FlowDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("FlowDeclaration", _entry_pos);
        let _result: Result<FlowDeclaration> = (|| {
        let start = self.current_span();
        let mut is_sufficient = false;
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::All)?;
                is_sufficient = true;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let v = self.parse_flow_end_member()?;
            owned_relationship.push(FlowDeclarationOwnedRelationshipMember::FlowEndMember(Box::new(v)));
            self.expect(TokenKind::To)?;
            let v = self.parse_flow_end_member()?;
            owned_relationship.push(FlowDeclarationOwnedRelationshipMember::FlowEndMember(Box::new(v)));
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_declaration()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_value_part()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::Of)?;
                let v = self.parse_payload_feature_member()?;
                owned_relationship.push(FlowDeclarationOwnedRelationshipMember::PayloadFeatureMember(Box::new(v)));
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.expect(TokenKind::From)?;
                let v = self.parse_flow_end_member()?;
                owned_relationship.push(FlowDeclarationOwnedRelationshipMember::FlowEndMember(Box::new(v)));
                self.expect(TokenKind::To)?;
                let v = self.parse_flow_end_member()?;
                owned_relationship.push(FlowDeclarationOwnedRelationshipMember::FlowEndMember(Box::new(v)));
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(FlowDeclaration {
            span: start.merge(end),
            is_sufficient,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FlowDeclaration");
        _result
    }

    /// Parse `PayloadFeatureMember`
    pub fn parse_payload_feature_member(&mut self) -> Result<PayloadFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PayloadFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into PayloadFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("PayloadFeatureMember", _entry_pos);
        let _result: Result<PayloadFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element_opt: Option<_> = None;
        let v = self.parse_payload_feature()?;
        owned_related_element_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(PayloadFeatureMember {
            span: start.merge(end),
            owned_related_element: owned_related_element_opt.ok_or_else(|| ParseError { message: "missing owned_related_element".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PayloadFeatureMember");
        _result
    }

    /// Parse `PayloadFeature`
    pub fn parse_payload_feature(&mut self) -> Result<PayloadFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PayloadFeature") {
            return Err(ParseError { message: "left-recursive entry into PayloadFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("PayloadFeature", _entry_pos);
        let _result: Result<PayloadFeature> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_identification()?;
            self.parse_payload_feature_specialization_part()?;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_value_part()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_identification()?;
            self.parse_value_part()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_feature_typing()?;
            owned_relationship.push(PayloadFeatureOwnedRelationshipMember::OwnedFeatureTyping(Box::new(v)));
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_owned_multiplicity()?;
                owned_relationship.push(PayloadFeatureOwnedRelationshipMember::OwnedMultiplicity(Box::new(v)));
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_owned_multiplicity()?;
            owned_relationship.push(PayloadFeatureOwnedRelationshipMember::OwnedMultiplicity(Box::new(v)));
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_owned_feature_typing()?;
                owned_relationship.push(PayloadFeatureOwnedRelationshipMember::OwnedFeatureTyping(Box::new(v)));
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(PayloadFeature {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PayloadFeature");
        _result
    }

    /// Parse `PayloadFeatureSpecializationPart`
    pub fn parse_payload_feature_specialization_part(&mut self) -> Result<PayloadFeatureSpecializationPart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PayloadFeatureSpecializationPart") {
            return Err(ParseError { message: "left-recursive entry into PayloadFeatureSpecializationPart".into(), span: self.current_span() });
        }
        self.push_rule_context("PayloadFeatureSpecializationPart", _entry_pos);
        let _result: Result<PayloadFeatureSpecializationPart> = (|| {
        let start = self.current_span();
        let mut feature_specialization = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_multiplicity_part()?;
            let mut _glr_stop_positions: Vec<usize> = Vec::new();
            let v = self.parse_feature_specialization()?;
            feature_specialization.push(v);
            _glr_stop_positions.push(self.pos);
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_feature_specialization()?;
                    feature_specialization.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                feature_specialization.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            let v = self.parse_feature_specialization()?;
            feature_specialization.push(v);
            loop {
                let saved = self.save();
                let body_ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_feature_specialization()?;
                    feature_specialization.push(v);
                    Ok(())
                })();
                if body_ok.is_err() {
                    self.restore(saved);
                    break; // Loop body failed, exit
                }
                if self.save() == saved { break; } // No progress, exit
                // Loop body succeeded - check if remainder can still parse
                let pos_after_body = self.save();
                let remainder_ok = (|| -> std::result::Result<(), ParseError> {
                    let saved = self.save();
                    let _: std::result::Result<(), ParseError> = (|| {
                        self.parse_multiplicity_part()?;
                        Ok(())
                    })().map_err(|e| { self.restore(saved); e });
                    loop {
                        let saved = self.save();
                        let ok: std::result::Result<(), ParseError> = (|| {
                            self.parse_feature_specialization()?;
                            Ok(())
                        })();
                        if ok.is_err() { self.restore(saved); break; }
                        if self.save() == saved { break; }
                    }
                    Ok(())
                })().is_ok();
                self.restore(pos_after_body);
                if !remainder_ok {
                    // Remainder can't parse after consuming this iteration
                    // Backtrack and leave input for remainder
                    feature_specialization.pop();
                    self.restore(saved);
                    break;
                }
                // Remainder can parse, keep going
            }
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                self.parse_multiplicity_part()?;
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_feature_specialization()?;
                    feature_specialization.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                feature_specialization.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(PayloadFeatureSpecializationPart {
            span: start.merge(end),
            feature_specialization,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PayloadFeatureSpecializationPart");
        _result
    }

    /// Parse `FlowEndMember`
    pub fn parse_flow_end_member(&mut self) -> Result<FlowEndMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FlowEndMember") {
            return Err(ParseError { message: "left-recursive entry into FlowEndMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FlowEndMember", _entry_pos);
        let _result: Result<FlowEndMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_flow_end()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(FlowEndMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FlowEndMember");
        _result
    }

    /// Parse `FlowEnd`
    pub fn parse_flow_end(&mut self) -> Result<FlowEnd> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FlowEnd") {
            return Err(ParseError { message: "left-recursive entry into FlowEnd".into(), span: self.current_span() });
        }
        self.push_rule_context("FlowEnd", _entry_pos);
        let _result: Result<FlowEnd> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_opt = self.save();
        let mut opt_succeeded = false;
        let mut glr_attempts = 0;
        const MAX_GLR_ATTEMPTS: usize = 10;
        let mut last_pos_before_opt = saved_opt;
        loop {
            if glr_attempts >= MAX_GLR_ATTEMPTS { break; }
            glr_attempts += 1;
            self.restore(saved_opt);
            let pre_opt_pos = self.pos;
            let opt_ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_owned_reference_subsetting()?;
                owned_relationship.push(FlowEndOwnedRelationshipMember::OwnedReferenceSubsetting(Box::new(v)));
                self.expect(TokenKind::Dot)?;
                Ok(())
            })();
            let post_opt_pos = self.pos;
            if opt_ok.is_err() {
                // Optional parsing failed — if we progressed, exclude and retry
                if post_opt_pos > pre_opt_pos && post_opt_pos != last_pos_before_opt {
                    last_pos_before_opt = post_opt_pos;
                    self.exclude_parse(saved_opt, "GeneralType", post_opt_pos);
                    continue; // Retry with shorter alternative
                }
                self.restore(saved_opt);
                break; // No more alternatives
            }
            let opt_end = self.pos;
            // Probe if remainder can parse from here
            let rem_ok: std::result::Result<(), ParseError> = (|| {
                self.parse_flow_feature_member()?;
                Ok(())
            })();
            self.pos = opt_end; // Restore after probe
            if rem_ok.is_ok() {
                opt_succeeded = true;
                break;
            }
            // Remainder failed — exclude this parse result and retry
            self.exclude_parse(saved_opt, "GeneralType", opt_end);
        }
        if !opt_succeeded {
            self.restore(saved_opt);
        }
        let v = self.parse_flow_feature_member()?;
        owned_relationship.push(FlowEndOwnedRelationshipMember::FlowFeatureMember(Box::new(v)));

        let end = self.current_span();
        Ok(FlowEnd {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FlowEnd");
        _result
    }

    /// Parse `FlowFeatureMember`
    pub fn parse_flow_feature_member(&mut self) -> Result<FlowFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FlowFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into FlowFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("FlowFeatureMember", _entry_pos);
        let _result: Result<FlowFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_flow_feature()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(FlowFeatureMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FlowFeatureMember");
        _result
    }

    /// Parse `FlowFeature`
    pub fn parse_flow_feature(&mut self) -> Result<FlowFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FlowFeature") {
            return Err(ParseError { message: "left-recursive entry into FlowFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("FlowFeature", _entry_pos);
        let _result: Result<FlowFeature> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_flow_feature_redefinition()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(FlowFeature {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FlowFeature");
        _result
    }

    /// Parse `FlowFeatureRedefinition`
    pub fn parse_flow_feature_redefinition(&mut self) -> Result<FlowFeatureRedefinition> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FlowFeatureRedefinition") {
            return Err(ParseError { message: "left-recursive entry into FlowFeatureRedefinition".into(), span: self.current_span() });
        }
        self.push_rule_context("FlowFeatureRedefinition", _entry_pos);
        let _result: Result<FlowFeatureRedefinition> = (|| {
        let start = self.current_span();
        let mut redefined_feature_opt: Option<_> = None;
        let v = self.parse_cross_ref()?;
        redefined_feature_opt = Some(v);

        let end = self.current_span();
        Ok(FlowFeatureRedefinition {
            span: start.merge(end),
            redefined_feature: redefined_feature_opt.ok_or_else(|| ParseError { message: "missing redefined_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FlowFeatureRedefinition");
        _result
    }

    /// Parse `ValuePart`
    pub fn parse_value_part(&mut self) -> Result<ValuePart> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ValuePart") {
            return Err(ParseError { message: "left-recursive entry into ValuePart".into(), span: self.current_span() });
        }
        self.push_rule_context("ValuePart", _entry_pos);
        let _result: Result<ValuePart> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_feature_value()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(ValuePart {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ValuePart");
        _result
    }

    /// Parse `FeatureValue`
    pub fn parse_feature_value(&mut self) -> Result<FeatureValue> {
        let _entry_pos = self.pos;
        if !self.enter_rule("FeatureValue") {
            return Err(ParseError { message: "left-recursive entry into FeatureValue".into(), span: self.current_span() });
        }
        self.push_rule_context("FeatureValue", _entry_pos);
        let _result: Result<FeatureValue> = (|| {
        let start = self.current_span();
        let mut is_default = false;
        let mut is_initial = false;
        let mut owned_related_element = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Eq)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Punct3A3D)?;
            is_initial = true;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Default)?;
            is_default = true;
            let saved = self.save();
            let _: std::result::Result<(), ParseError> = (|| {
                let saved_alt = self.save();
                let mut best_alt_pos: Option<usize> = None;
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.expect(TokenKind::Eq)?;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                self.restore(saved_alt);
                if (|| -> std::result::Result<(), ParseError> {
                    self.expect(TokenKind::Punct3A3D)?;
                    is_initial = true;
                    Ok(())
                })().is_ok() {
                    let end = self.save();
                    if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                }
                match best_alt_pos {
                    Some(pos) => self.pos = pos,
                    None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
                }
                Ok(())
            })().map_err(|e| { self.restore(saved); e });
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        let v = self.parse_owned_expression()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(FeatureValue {
            span: start.merge(end),
            is_default,
            is_initial,
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "FeatureValue");
        _result
    }

    /// Parse `Multiplicity`
    pub fn parse_multiplicity(&mut self) -> Result<Multiplicity> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Multiplicity") {
            return Err(ParseError { message: "left-recursive entry into Multiplicity".into(), span: self.current_span() });
        }
        self.push_rule_context("Multiplicity", _entry_pos);
        let _result: Result<Multiplicity> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(Multiplicity, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_multiplicity_subset() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((Multiplicity::MultiplicitySubset(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_multiplicity_range() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((Multiplicity::MultiplicityRange(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected Multiplicity".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Multiplicity");
        _result
    }

    /// Parse `MultiplicitySubset`
    pub fn parse_multiplicity_subset(&mut self) -> Result<MultiplicitySubset> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MultiplicitySubset") {
            return Err(ParseError { message: "left-recursive entry into MultiplicitySubset".into(), span: self.current_span() });
        }
        self.push_rule_context("MultiplicitySubset", _entry_pos);
        let _result: Result<MultiplicitySubset> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Multiplicity)?;
        self.parse_identification()?;
        self.parse_subsets()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(MultiplicitySubset {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MultiplicitySubset");
        _result
    }

    /// Parse `MultiplicityRange`
    pub fn parse_multiplicity_range(&mut self) -> Result<MultiplicityRange> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MultiplicityRange") {
            return Err(ParseError { message: "left-recursive entry into MultiplicityRange".into(), span: self.current_span() });
        }
        self.push_rule_context("MultiplicityRange", _entry_pos);
        let _result: Result<MultiplicityRange> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Multiplicity)?;
        self.parse_identification()?;
        self.parse_multiplicity_bounds()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(MultiplicityRange {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MultiplicityRange");
        _result
    }

    /// Parse `OwnedMultiplicity`
    pub fn parse_owned_multiplicity(&mut self) -> Result<OwnedMultiplicity> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedMultiplicity") {
            return Err(ParseError { message: "left-recursive entry into OwnedMultiplicity".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedMultiplicity", _entry_pos);
        let _result: Result<OwnedMultiplicity> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let v = self.parse_owned_multiplicity_range()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(OwnedMultiplicity {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedMultiplicity");
        _result
    }

    /// Parse `OwnedMultiplicityRange`
    pub fn parse_owned_multiplicity_range(&mut self) -> Result<OwnedMultiplicityRange> {
        let _entry_pos = self.pos;
        if !self.enter_rule("OwnedMultiplicityRange") {
            return Err(ParseError { message: "left-recursive entry into OwnedMultiplicityRange".into(), span: self.current_span() });
        }
        self.push_rule_context("OwnedMultiplicityRange", _entry_pos);
        let _result: Result<OwnedMultiplicityRange> = (|| {
        let start = self.current_span();
        self.parse_multiplicity_bounds()?;

        let end = self.current_span();
        Ok(OwnedMultiplicityRange {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "OwnedMultiplicityRange");
        _result
    }

    /// Parse `MultiplicityBounds`
    pub fn parse_multiplicity_bounds(&mut self) -> Result<MultiplicityBounds> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MultiplicityBounds") {
            return Err(ParseError { message: "left-recursive entry into MultiplicityBounds".into(), span: self.current_span() });
        }
        self.push_rule_context("MultiplicityBounds", _entry_pos);
        let _result: Result<MultiplicityBounds> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::LBracket)?;
        let saved_opt = self.save();
        let mut opt_succeeded = false;
        let mut glr_attempts = 0;
        const MAX_GLR_ATTEMPTS: usize = 10;
        let mut last_pos_before_opt = saved_opt;
        loop {
            if glr_attempts >= MAX_GLR_ATTEMPTS { break; }
            glr_attempts += 1;
            self.restore(saved_opt);
            let pre_opt_pos = self.pos;
            let opt_ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_multiplicity_expression_member()?;
                owned_relationship.push(v);
                self.expect(TokenKind::DotDot)?;
                Ok(())
            })();
            let post_opt_pos = self.pos;
            if opt_ok.is_err() {
                // Optional parsing failed — if we progressed, exclude and retry
                if post_opt_pos > pre_opt_pos && post_opt_pos != last_pos_before_opt {
                    last_pos_before_opt = post_opt_pos;
                    continue; // Retry with shorter alternative
                }
                self.restore(saved_opt);
                break; // No more alternatives
            }
            let opt_end = self.pos;
            // Probe if remainder can parse from here
            let rem_ok: std::result::Result<(), ParseError> = (|| {
                self.parse_multiplicity_expression_member()?;
                self.expect(TokenKind::RBracket)?;
                Ok(())
            })();
            self.pos = opt_end; // Restore after probe
            if rem_ok.is_ok() {
                opt_succeeded = true;
                break;
            }
            // Remainder failed — exclude this parse result and retry
        }
        if !opt_succeeded {
            self.restore(saved_opt);
        }
        let v = self.parse_multiplicity_expression_member()?;
        owned_relationship.push(v);
        self.expect(TokenKind::RBracket)?;

        let end = self.current_span();
        Ok(MultiplicityBounds {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MultiplicityBounds");
        _result
    }

    /// Parse `MultiplicityExpressionMember`
    pub fn parse_multiplicity_expression_member(&mut self) -> Result<MultiplicityExpressionMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MultiplicityExpressionMember") {
            return Err(ParseError { message: "left-recursive entry into MultiplicityExpressionMember".into(), span: self.current_span() });
        }
        self.push_rule_context("MultiplicityExpressionMember", _entry_pos);
        let _result: Result<MultiplicityExpressionMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_literal_expression()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.parse_feature_reference_expression()?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(MultiplicityExpressionMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MultiplicityExpressionMember");
        _result
    }

    /// Parse `Metaclass`
    pub fn parse_metaclass(&mut self) -> Result<Metaclass> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Metaclass") {
            return Err(ParseError { message: "left-recursive entry into Metaclass".into(), span: self.current_span() });
        }
        self.push_rule_context("Metaclass", _entry_pos);
        let _result: Result<Metaclass> = (|| {
        let start = self.current_span();
        self.parse_type_prefix()?;
        self.expect(TokenKind::Metaclass)?;
        self.parse_classifier_declaration()?;
        self.parse_type_body()?;

        let end = self.current_span();
        Ok(Metaclass {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Metaclass");
        _result
    }

    /// Parse `PrefixMetadataAnnotation`
    pub fn parse_prefix_metadata_annotation(&mut self) -> Result<PrefixMetadataAnnotation> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PrefixMetadataAnnotation") {
            return Err(ParseError { message: "left-recursive entry into PrefixMetadataAnnotation".into(), span: self.current_span() });
        }
        self.push_rule_context("PrefixMetadataAnnotation", _entry_pos);
        let _result: Result<PrefixMetadataAnnotation> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.expect(TokenKind::Hash)?;
        let v = self.parse_prefix_metadata_feature()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(PrefixMetadataAnnotation {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PrefixMetadataAnnotation");
        _result
    }

    /// Parse `PrefixMetadataMember`
    pub fn parse_prefix_metadata_member(&mut self) -> Result<PrefixMetadataMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PrefixMetadataMember") {
            return Err(ParseError { message: "left-recursive entry into PrefixMetadataMember".into(), span: self.current_span() });
        }
        self.push_rule_context("PrefixMetadataMember", _entry_pos);
        let _result: Result<PrefixMetadataMember> = (|| {
        let start = self.current_span();
        let mut owned_related_element = Vec::new();
        self.expect(TokenKind::Hash)?;
        let v = self.parse_prefix_metadata_feature()?;
        owned_related_element.push(v);

        let end = self.current_span();
        Ok(PrefixMetadataMember {
            span: start.merge(end),
            owned_related_element,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PrefixMetadataMember");
        _result
    }

    /// Parse `PrefixMetadataFeature`
    pub fn parse_prefix_metadata_feature(&mut self) -> Result<PrefixMetadataFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PrefixMetadataFeature") {
            return Err(ParseError { message: "left-recursive entry into PrefixMetadataFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("PrefixMetadataFeature", _entry_pos);
        let _result: Result<PrefixMetadataFeature> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let v = self.parse_owned_feature_typing()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(PrefixMetadataFeature {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PrefixMetadataFeature");
        _result
    }

    /// Parse `MetadataFeature`
    pub fn parse_metadata_feature(&mut self) -> Result<MetadataFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataFeature") {
            return Err(ParseError { message: "left-recursive entry into MetadataFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataFeature", _entry_pos);
        let _result: Result<MetadataFeature> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(MetadataFeatureOwnedRelationshipMember::PrefixMetadataMember(Box::new(v)));
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::AtSign)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Metadata)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }
        self.parse_metadata_feature_declaration()?;
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::About)?;
            let v = self.parse_annotation()?;
            owned_relationship.push(MetadataFeatureOwnedRelationshipMember::Annotation(Box::new(v)));
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    self.expect(TokenKind::Comma)?;
                    let v = self.parse_annotation()?;
                    owned_relationship.push(MetadataFeatureOwnedRelationshipMember::Annotation(Box::new(v)));
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_metadata_body()?;

        let end = self.current_span();
        Ok(MetadataFeature {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataFeature");
        _result
    }

    /// Parse `MetadataFeatureDeclaration`
    pub fn parse_metadata_feature_declaration(&mut self) -> Result<MetadataFeatureDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataFeatureDeclaration") {
            return Err(ParseError { message: "left-recursive entry into MetadataFeatureDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataFeatureDeclaration", _entry_pos);
        let _result: Result<MetadataFeatureDeclaration> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_identification()?;
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Typed)?;
                self.expect(TokenKind::By)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Colon)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.parse_owned_feature_typing()?;
        owned_relationship.push(v);

        let end = self.current_span();
        Ok(MetadataFeatureDeclaration {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataFeatureDeclaration");
        _result
    }

    /// Parse `MetadataBody`
    pub fn parse_metadata_body(&mut self) -> Result<MetadataBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataBody") {
            return Err(ParseError { message: "left-recursive entry into MetadataBody".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataBody", _entry_pos);
        let _result: Result<MetadataBody> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LBrace)?;
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let v = self.parse_metadata_body_element()?;
                    owned_relationship.push(v);
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            self.expect(TokenKind::RBrace)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Semi)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(MetadataBody {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataBody");
        _result
    }

    /// Parse `MetadataBodyElement`
    pub fn parse_metadata_body_element(&mut self) -> Result<MetadataBodyElement> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataBodyElement") {
            return Err(ParseError { message: "left-recursive entry into MetadataBodyElement".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataBodyElement", _entry_pos);
        let _result: Result<MetadataBodyElement> = (|| {
        let alt_saved = self.save();
        let mut best: Option<(MetadataBodyElement, usize)> = None;

        self.restore(alt_saved);
        if let Ok(v) = self.parse_non_feature_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((MetadataBodyElement::NonFeatureMember(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_metadata_body_feature_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((MetadataBodyElement::MetadataBodyFeatureMember(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_alias_member() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((MetadataBodyElement::AliasMember(Box::new(v)), end));
            }
        }

        self.restore(alt_saved);
        if let Ok(v) = self.parse_import() {
            let end = self.pos;
            if best.as_ref().map_or(true, |(_, best_end)| end > *best_end) {
                best = Some((MetadataBodyElement::Import(Box::new(v)), end));
            }
        }

        match best {
            Some((result, end_pos)) => {
                self.pos = end_pos;
                Ok(result)
            }
            None => Err(ParseError { message: "expected MetadataBodyElement".into(), span: self.current_span() })
        }
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataBodyElement");
        _result
    }

    /// Parse `MetadataBodyFeatureMember`
    pub fn parse_metadata_body_feature_member(&mut self) -> Result<MetadataBodyFeatureMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataBodyFeatureMember") {
            return Err(ParseError { message: "left-recursive entry into MetadataBodyFeatureMember".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataBodyFeatureMember", _entry_pos);
        let _result: Result<MetadataBodyFeatureMember> = (|| {
        let start = self.current_span();
        let mut owned_member_feature_opt: Option<_> = None;
        let v = self.parse_metadata_body_feature()?;
        owned_member_feature_opt = Some(Box::new(v));

        let end = self.current_span();
        Ok(MetadataBodyFeatureMember {
            span: start.merge(end),
            owned_member_feature: owned_member_feature_opt.ok_or_else(|| ParseError { message: "missing owned_member_feature".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataBodyFeatureMember");
        _result
    }

    /// Parse `MetadataBodyFeature`
    pub fn parse_metadata_body_feature(&mut self) -> Result<MetadataBodyFeature> {
        let _entry_pos = self.pos;
        if !self.enter_rule("MetadataBodyFeature") {
            return Err(ParseError { message: "left-recursive entry into MetadataBodyFeature".into(), span: self.current_span() });
        }
        self.push_rule_context("MetadataBodyFeature", _entry_pos);
        let _result: Result<MetadataBodyFeature> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.expect(TokenKind::Feature)?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            let saved_alt = self.save();
            let mut best_alt_pos: Option<usize> = None;
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::ColonGtGt)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            self.restore(saved_alt);
            if (|| -> std::result::Result<(), ParseError> {
                self.expect(TokenKind::Redefines)?;
                Ok(())
            })().is_ok() {
                let end = self.save();
                if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
            }
            match best_alt_pos {
                Some(pos) => self.pos = pos,
                None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
            }
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let v = self.parse_owned_redefinition()?;
        owned_relationship.push(v);
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_feature_specialization_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        let saved = self.save();
        let _: std::result::Result<(), ParseError> = (|| {
            self.parse_value_part()?;
            Ok(())
        })().map_err(|e| { self.restore(saved); e });
        self.parse_metadata_body()?;

        let end = self.current_span();
        Ok(MetadataBodyFeature {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "MetadataBodyFeature");
        _result
    }

    /// Parse `Package`
    pub fn parse_package(&mut self) -> Result<Package> {
        let _entry_pos = self.pos;
        if !self.enter_rule("Package") {
            return Err(ParseError { message: "left-recursive entry into Package".into(), span: self.current_span() });
        }
        self.push_rule_context("Package", _entry_pos);
        let _result: Result<Package> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.parse_package_declaration()?;
        self.parse_package_body()?;

        let end = self.current_span();
        Ok(Package {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "Package");
        _result
    }

    /// Parse `LibraryPackage`
    pub fn parse_library_package(&mut self) -> Result<LibraryPackage> {
        let _entry_pos = self.pos;
        if !self.enter_rule("LibraryPackage") {
            return Err(ParseError { message: "left-recursive entry into LibraryPackage".into(), span: self.current_span() });
        }
        self.push_rule_context("LibraryPackage", _entry_pos);
        let _result: Result<LibraryPackage> = (|| {
        let start = self.current_span();
        let mut is_standard = false;
        let mut owned_relationship = Vec::new();
        self.expect(TokenKind::Standard)?;
        is_standard = true;
        self.expect(TokenKind::Library)?;
        let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
        loop {
            let saved = self.save();
            let ok: std::result::Result<(), ParseError> = (|| {
                let v = self.parse_prefix_metadata_member()?;
                owned_relationship.push(v);
                Ok(())
            })();
            if ok.is_err() { self.restore(saved); break; }
            if self.save() == saved { break; } // no progress
            _glr_stop_positions.push(self.pos);
        }
        while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
            _glr_stop_positions.pop();
            self.pos = *_glr_stop_positions.last().unwrap();
        }
        self.parse_package_declaration()?;
        self.parse_package_body()?;

        let end = self.current_span();
        Ok(LibraryPackage {
            span: start.merge(end),
            is_standard,
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "LibraryPackage");
        _result
    }

    /// Parse `PackageDeclaration`
    pub fn parse_package_declaration(&mut self) -> Result<PackageDeclaration> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PackageDeclaration") {
            return Err(ParseError { message: "left-recursive entry into PackageDeclaration".into(), span: self.current_span() });
        }
        self.push_rule_context("PackageDeclaration", _entry_pos);
        let _result: Result<PackageDeclaration> = (|| {
        let start = self.current_span();
        self.expect(TokenKind::Package)?;
        self.parse_identification()?;

        let end = self.current_span();
        Ok(PackageDeclaration {
            span: start.merge(end),
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PackageDeclaration");
        _result
    }

    /// Parse `PackageBody`
    pub fn parse_package_body(&mut self) -> Result<PackageBody> {
        let _entry_pos = self.pos;
        if !self.enter_rule("PackageBody") {
            return Err(ParseError { message: "left-recursive entry into PackageBody".into(), span: self.current_span() });
        }
        self.push_rule_context("PackageBody", _entry_pos);
        let _result: Result<PackageBody> = (|| {
        let start = self.current_span();
        let mut owned_relationship = Vec::new();
        let saved_alt = self.save();
        let mut best_alt_pos: Option<usize> = None;
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::LBrace)?;
            let mut _glr_stop_positions: Vec<usize> = vec![self.pos];
            loop {
                let saved = self.save();
                let ok: std::result::Result<(), ParseError> = (|| {
                    let saved_alt = self.save();
                    let mut best_alt_pos: Option<usize> = None;
                    self.restore(saved_alt);
                    if (|| -> std::result::Result<(), ParseError> {
                        self.parse_namespace_body_element()?;
                        Ok(())
                    })().is_ok() {
                        let end = self.save();
                        if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                    }
                    self.restore(saved_alt);
                    if (|| -> std::result::Result<(), ParseError> {
                        let v = self.parse_element_filter_member()?;
                        owned_relationship.push(v);
                        Ok(())
                    })().is_ok() {
                        let end = self.save();
                        if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
                    }
                    match best_alt_pos {
                        Some(pos) => self.pos = pos,
                        None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
                    }
                    Ok(())
                })();
                if ok.is_err() { self.restore(saved); break; }
                if self.save() == saved { break; } // no progress
                _glr_stop_positions.push(self.pos);
            }
            while _glr_stop_positions.len() > 1 && self.is_current_pos_excluded() {
                _glr_stop_positions.pop();
                self.pos = *_glr_stop_positions.last().unwrap();
            }
            self.expect(TokenKind::RBrace)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        self.restore(saved_alt);
        if (|| -> std::result::Result<(), ParseError> {
            self.expect(TokenKind::Semi)?;
            Ok(())
        })().is_ok() {
            let end = self.save();
            if best_alt_pos.map_or(true, |b| end > b) { best_alt_pos = Some(end); }
        }
        match best_alt_pos {
            Some(pos) => self.pos = pos,
            None => return Err(ParseError { message: "no alternative matched".into(), span: self.current_span() }),
        }

        let end = self.current_span();
        Ok(PackageBody {
            span: start.merge(end),
            owned_relationship,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "PackageBody");
        _result
    }

    /// Parse `ElementFilterMember`
    pub fn parse_element_filter_member(&mut self) -> Result<ElementFilterMember> {
        let _entry_pos = self.pos;
        if !self.enter_rule("ElementFilterMember") {
            return Err(ParseError { message: "left-recursive entry into ElementFilterMember".into(), span: self.current_span() });
        }
        self.push_rule_context("ElementFilterMember", _entry_pos);
        let _result: Result<ElementFilterMember> = (|| {
        let start = self.current_span();
        let mut condition_opt: Option<_> = None;
        self.parse_member_prefix()?;
        self.expect(TokenKind::Filter)?;
        let v = self.parse_owned_expression()?;
        condition_opt = Some(Box::new(v));
        self.expect(TokenKind::Semi)?;

        let end = self.current_span();
        Ok(ElementFilterMember {
            span: start.merge(end),
            condition: condition_opt.ok_or_else(|| ParseError { message: "missing condition".into(), span: start })?,
        })
        })();
        self.pop_rule_context();
        self.leave_rule(_entry_pos, "ElementFilterMember");
        _result
    }

    /// Stub for undefined rule `InvocationTypeMember`
    fn parse_invocation_type_member(&mut self) -> Result<InvocationTypeMember> {
        Err(ParseError { message: "rule InvocationTypeMember is not defined".into(), span: self.current_span() })
    }

    /// Dispatch to a parse function by rule name (snake_case).
    /// Returns the parser position after successful parsing.
    pub fn try_parse_rule(&mut self, rule: &str) -> Result<usize> {
        match rule {
            "identification" => self.parse_identification().map(|_| self.pos()),
            "relationship_body" => self.parse_relationship_body().map(|_| self.pos()),
            "relationship_owned_element" => self.parse_relationship_owned_element().map(|_| self.pos()),
            "owned_related_element" => self.parse_owned_related_element().map(|_| self.pos()),
            "dependency" => self.parse_dependency().map(|_| self.pos()),
            "annotation" => self.parse_annotation().map(|_| self.pos()),
            "owned_annotation" => self.parse_owned_annotation().map(|_| self.pos()),
            "annotating_element" => self.parse_annotating_element().map(|_| self.pos()),
            "comment" => self.parse_comment().map(|_| self.pos()),
            "documentation" => self.parse_documentation().map(|_| self.pos()),
            "textual_representation" => self.parse_textual_representation().map(|_| self.pos()),
            "root_namespace" => self.parse_root_namespace().map(|_| self.pos()),
            "namespace" => self.parse_namespace().map(|_| self.pos()),
            "namespace_declaration" => self.parse_namespace_declaration().map(|_| self.pos()),
            "namespace_body" => self.parse_namespace_body().map(|_| self.pos()),
            "namespace_body_element" => self.parse_namespace_body_element().map(|_| self.pos()),
            "member_prefix" => self.parse_member_prefix().map(|_| self.pos()),
            "visibility_indicator" => self.parse_visibility_indicator().map(|_| self.pos()),
            "namespace_member" => self.parse_namespace_member().map(|_| self.pos()),
            "non_feature_member" => self.parse_non_feature_member().map(|_| self.pos()),
            "namespace_feature_member" => self.parse_namespace_feature_member().map(|_| self.pos()),
            "alias_member" => self.parse_alias_member().map(|_| self.pos()),
            "qualified_name" => self.parse_qualified_name().map(|_| self.pos()),
            "import" => self.parse_import().map(|_| self.pos()),
            "import_declaration" => self.parse_import_declaration().map(|_| self.pos()),
            "membership_import" => self.parse_membership_import().map(|_| self.pos()),
            "namespace_import" => self.parse_namespace_import().map(|_| self.pos()),
            "filter_package" => self.parse_filter_package().map(|_| self.pos()),
            "filter_package_member" => self.parse_filter_package_member().map(|_| self.pos()),
            "member_element" => self.parse_member_element().map(|_| self.pos()),
            "non_feature_element" => self.parse_non_feature_element().map(|_| self.pos()),
            "feature_element" => self.parse_feature_element().map(|_| self.pos()),
            "type_" => self.parse_type_().map(|_| self.pos()),
            "type_prefix" => self.parse_type_prefix().map(|_| self.pos()),
            "type_declaration" => self.parse_type_declaration().map(|_| self.pos()),
            "specialization_part" => self.parse_specialization_part().map(|_| self.pos()),
            "conjugation_part" => self.parse_conjugation_part().map(|_| self.pos()),
            "type_relationship_part" => self.parse_type_relationship_part().map(|_| self.pos()),
            "disjoining_part" => self.parse_disjoining_part().map(|_| self.pos()),
            "unioning_part" => self.parse_unioning_part().map(|_| self.pos()),
            "intersecting_part" => self.parse_intersecting_part().map(|_| self.pos()),
            "differencing_part" => self.parse_differencing_part().map(|_| self.pos()),
            "type_body" => self.parse_type_body().map(|_| self.pos()),
            "type_body_element" => self.parse_type_body_element().map(|_| self.pos()),
            "specialization" => self.parse_specialization().map(|_| self.pos()),
            "owned_specialization" => self.parse_owned_specialization().map(|_| self.pos()),
            "specific_type" => self.parse_specific_type().map(|_| self.pos()),
            "general_type" => self.parse_general_type().map(|_| self.pos()),
            "conjugation" => self.parse_conjugation().map(|_| self.pos()),
            "owned_conjugation" => self.parse_owned_conjugation().map(|_| self.pos()),
            "disjoining" => self.parse_disjoining().map(|_| self.pos()),
            "owned_disjoining" => self.parse_owned_disjoining().map(|_| self.pos()),
            "unioning" => self.parse_unioning().map(|_| self.pos()),
            "intersecting" => self.parse_intersecting().map(|_| self.pos()),
            "differencing" => self.parse_differencing().map(|_| self.pos()),
            "feature_member" => self.parse_feature_member().map(|_| self.pos()),
            "type_feature_member" => self.parse_type_feature_member().map(|_| self.pos()),
            "owned_feature_member" => self.parse_owned_feature_member().map(|_| self.pos()),
            "classifier" => self.parse_classifier().map(|_| self.pos()),
            "classifier_declaration" => self.parse_classifier_declaration().map(|_| self.pos()),
            "superclassing_part" => self.parse_superclassing_part().map(|_| self.pos()),
            "subclassification" => self.parse_subclassification().map(|_| self.pos()),
            "owned_subclassification" => self.parse_owned_subclassification().map(|_| self.pos()),
            "feature" => self.parse_feature().map(|_| self.pos()),
            "end_feature_prefix" => self.parse_end_feature_prefix().map(|_| self.pos()),
            "basic_feature_prefix" => self.parse_basic_feature_prefix().map(|_| self.pos()),
            "feature_prefix" => self.parse_feature_prefix().map(|_| self.pos()),
            "owned_cross_feature_member" => self.parse_owned_cross_feature_member().map(|_| self.pos()),
            "owned_cross_feature" => self.parse_owned_cross_feature().map(|_| self.pos()),
            "feature_direction" => self.parse_feature_direction().map(|_| self.pos()),
            "feature_declaration" => self.parse_feature_declaration().map(|_| self.pos()),
            "feature_identification" => self.parse_feature_identification().map(|_| self.pos()),
            "feature_relationship_part" => self.parse_feature_relationship_part().map(|_| self.pos()),
            "chaining_part" => self.parse_chaining_part().map(|_| self.pos()),
            "inverting_part" => self.parse_inverting_part().map(|_| self.pos()),
            "type_featuring_part" => self.parse_type_featuring_part().map(|_| self.pos()),
            "feature_specialization_part" => self.parse_feature_specialization_part().map(|_| self.pos()),
            "multiplicity_part" => self.parse_multiplicity_part().map(|_| self.pos()),
            "feature_specialization" => self.parse_feature_specialization().map(|_| self.pos()),
            "typings" => self.parse_typings().map(|_| self.pos()),
            "typed_by" => self.parse_typed_by().map(|_| self.pos()),
            "subsettings" => self.parse_subsettings().map(|_| self.pos()),
            "subsets" => self.parse_subsets().map(|_| self.pos()),
            "references" => self.parse_references().map(|_| self.pos()),
            "crosses" => self.parse_crosses().map(|_| self.pos()),
            "redefinitions" => self.parse_redefinitions().map(|_| self.pos()),
            "redefines" => self.parse_redefines().map(|_| self.pos()),
            "feature_typing" => self.parse_feature_typing().map(|_| self.pos()),
            "owned_feature_typing" => self.parse_owned_feature_typing().map(|_| self.pos()),
            "subsetting" => self.parse_subsetting().map(|_| self.pos()),
            "owned_subsetting" => self.parse_owned_subsetting().map(|_| self.pos()),
            "owned_reference_subsetting" => self.parse_owned_reference_subsetting().map(|_| self.pos()),
            "owned_cross_subsetting" => self.parse_owned_cross_subsetting().map(|_| self.pos()),
            "redefinition" => self.parse_redefinition().map(|_| self.pos()),
            "owned_redefinition" => self.parse_owned_redefinition().map(|_| self.pos()),
            "owned_feature_chain" => self.parse_owned_feature_chain().map(|_| self.pos()),
            "feature_chain" => self.parse_feature_chain().map(|_| self.pos()),
            "owned_feature_chaining" => self.parse_owned_feature_chaining().map(|_| self.pos()),
            "feature_inverting" => self.parse_feature_inverting().map(|_| self.pos()),
            "owned_feature_inverting" => self.parse_owned_feature_inverting().map(|_| self.pos()),
            "type_featuring" => self.parse_type_featuring().map(|_| self.pos()),
            "owned_type_featuring" => self.parse_owned_type_featuring().map(|_| self.pos()),
            "data_type" => self.parse_data_type().map(|_| self.pos()),
            "class" => self.parse_class().map(|_| self.pos()),
            "structure" => self.parse_structure().map(|_| self.pos()),
            "association" => self.parse_association().map(|_| self.pos()),
            "association_structure" => self.parse_association_structure().map(|_| self.pos()),
            "connector" => self.parse_connector().map(|_| self.pos()),
            "connector_declaration" => self.parse_connector_declaration().map(|_| self.pos()),
            "binary_connector_declaration" => self.parse_binary_connector_declaration().map(|_| self.pos()),
            "nary_connector_declaration" => self.parse_nary_connector_declaration().map(|_| self.pos()),
            "connector_end_member" => self.parse_connector_end_member().map(|_| self.pos()),
            "connector_end" => self.parse_connector_end().map(|_| self.pos()),
            "owned_cross_multiplicity_member" => self.parse_owned_cross_multiplicity_member().map(|_| self.pos()),
            "owned_cross_multiplicity" => self.parse_owned_cross_multiplicity().map(|_| self.pos()),
            "binding_connector" => self.parse_binding_connector().map(|_| self.pos()),
            "binding_connector_declaration" => self.parse_binding_connector_declaration().map(|_| self.pos()),
            "succession" => self.parse_succession().map(|_| self.pos()),
            "succession_declaration" => self.parse_succession_declaration().map(|_| self.pos()),
            "behavior" => self.parse_behavior().map(|_| self.pos()),
            "step" => self.parse_step().map(|_| self.pos()),
            "function" => self.parse_function().map(|_| self.pos()),
            "function_body" => self.parse_function_body().map(|_| self.pos()),
            "function_body_part" => self.parse_function_body_part().map(|_| self.pos()),
            "return_feature_member" => self.parse_return_feature_member().map(|_| self.pos()),
            "result_expression_member" => self.parse_result_expression_member().map(|_| self.pos()),
            "expression" => self.parse_expression().map(|_| self.pos()),
            "predicate" => self.parse_predicate().map(|_| self.pos()),
            "boolean_expression" => self.parse_boolean_expression().map(|_| self.pos()),
            "invariant" => self.parse_invariant().map(|_| self.pos()),
            "owned_expression_reference_member" => self.parse_owned_expression_reference_member().map(|_| self.pos()),
            "owned_expression_reference" => self.parse_owned_expression_reference().map(|_| self.pos()),
            "owned_expression_member" => self.parse_owned_expression_member().map(|_| self.pos()),
            "owned_expression" => self.parse_owned_expression().map(|_| self.pos()),
            "conditional_expression" => self.parse_conditional_expression().map(|_| self.pos()),
            "conditional_binary_operator_expression" => self.parse_conditional_binary_operator_expression().map(|_| self.pos()),
            "conditional_binary_operator" => self.parse_conditional_binary_operator().map(|_| self.pos()),
            "binary_operator_expression" => self.parse_binary_operator_expression().map(|_| self.pos()),
            "binary_operator" => self.parse_binary_operator().map(|_| self.pos()),
            "unary_operator_expression" => self.parse_unary_operator_expression().map(|_| self.pos()),
            "unary_operator" => self.parse_unary_operator().map(|_| self.pos()),
            "classification_expression" => self.parse_classification_expression().map(|_| self.pos()),
            "classification_test_operator" => self.parse_classification_test_operator().map(|_| self.pos()),
            "cast_operator" => self.parse_cast_operator().map(|_| self.pos()),
            "metaclassification_expression" => self.parse_metaclassification_expression().map(|_| self.pos()),
            "argument_member" => self.parse_argument_member().map(|_| self.pos()),
            "argument" => self.parse_argument().map(|_| self.pos()),
            "argument_value" => self.parse_argument_value().map(|_| self.pos()),
            "argument_expression_member" => self.parse_argument_expression_member().map(|_| self.pos()),
            "argument_expression" => self.parse_argument_expression().map(|_| self.pos()),
            "argument_expression_value" => self.parse_argument_expression_value().map(|_| self.pos()),
            "metadata_argument_member" => self.parse_metadata_argument_member().map(|_| self.pos()),
            "metadata_argument" => self.parse_metadata_argument().map(|_| self.pos()),
            "metadata_value" => self.parse_metadata_value().map(|_| self.pos()),
            "metadata_reference" => self.parse_metadata_reference().map(|_| self.pos()),
            "metaclassification_test_operator" => self.parse_metaclassification_test_operator().map(|_| self.pos()),
            "meta_cast_operator" => self.parse_meta_cast_operator().map(|_| self.pos()),
            "extent_expression" => self.parse_extent_expression().map(|_| self.pos()),
            "type_reference_member" => self.parse_type_reference_member().map(|_| self.pos()),
            "type_result_member" => self.parse_type_result_member().map(|_| self.pos()),
            "type_reference" => self.parse_type_reference().map(|_| self.pos()),
            "reference_typing" => self.parse_reference_typing().map(|_| self.pos()),
            "empty_result_member" => self.parse_empty_result_member().map(|_| self.pos()),
            "empty_feature" => self.parse_empty_feature().map(|_| self.pos()),
            "primary_expression" => self.parse_primary_expression().map(|_| self.pos()),
            "primary_argument_value" => self.parse_primary_argument_value().map(|_| self.pos()),
            "primary_argument" => self.parse_primary_argument().map(|_| self.pos()),
            "primary_argument_member" => self.parse_primary_argument_member().map(|_| self.pos()),
            "non_feature_chain_primary_expression" => self.parse_non_feature_chain_primary_expression().map(|_| self.pos()),
            "non_feature_chain_primary_argument_value" => self.parse_non_feature_chain_primary_argument_value().map(|_| self.pos()),
            "non_feature_chain_primary_argument" => self.parse_non_feature_chain_primary_argument().map(|_| self.pos()),
            "non_feature_chain_primary_argument_member" => self.parse_non_feature_chain_primary_argument_member().map(|_| self.pos()),
            "bracket_expression" => self.parse_bracket_expression().map(|_| self.pos()),
            "index_expression" => self.parse_index_expression().map(|_| self.pos()),
            "sequence_expression" => self.parse_sequence_expression().map(|_| self.pos()),
            "sequence_expression_list" => self.parse_sequence_expression_list().map(|_| self.pos()),
            "sequence_operator_expression" => self.parse_sequence_operator_expression().map(|_| self.pos()),
            "sequence_expression_list_member" => self.parse_sequence_expression_list_member().map(|_| self.pos()),
            "feature_chain_expression" => self.parse_feature_chain_expression().map(|_| self.pos()),
            "collect_expression" => self.parse_collect_expression().map(|_| self.pos()),
            "select_expression" => self.parse_select_expression().map(|_| self.pos()),
            "function_operation_expression" => self.parse_function_operation_expression().map(|_| self.pos()),
            "body_argument_member" => self.parse_body_argument_member().map(|_| self.pos()),
            "body_argument" => self.parse_body_argument().map(|_| self.pos()),
            "body_argument_value" => self.parse_body_argument_value().map(|_| self.pos()),
            "function_reference_argument_member" => self.parse_function_reference_argument_member().map(|_| self.pos()),
            "function_reference_argument" => self.parse_function_reference_argument().map(|_| self.pos()),
            "function_reference_argument_value" => self.parse_function_reference_argument_value().map(|_| self.pos()),
            "function_reference_expression" => self.parse_function_reference_expression().map(|_| self.pos()),
            "function_reference_member" => self.parse_function_reference_member().map(|_| self.pos()),
            "function_reference" => self.parse_function_reference().map(|_| self.pos()),
            "feature_chain_member" => self.parse_feature_chain_member().map(|_| self.pos()),
            "owned_feature_chain_member" => self.parse_owned_feature_chain_member().map(|_| self.pos()),
            "base_expression" => self.parse_base_expression().map(|_| self.pos()),
            "null_expression" => self.parse_null_expression().map(|_| self.pos()),
            "feature_reference_expression" => self.parse_feature_reference_expression().map(|_| self.pos()),
            "feature_reference_member" => self.parse_feature_reference_member().map(|_| self.pos()),
            "feature_reference" => self.parse_feature_reference().map(|_| self.pos()),
            "metadata_access_expression" => self.parse_metadata_access_expression().map(|_| self.pos()),
            "element_reference_member" => self.parse_element_reference_member().map(|_| self.pos()),
            "invocation_expression" => self.parse_invocation_expression().map(|_| self.pos()),
            "constructor_expression" => self.parse_constructor_expression().map(|_| self.pos()),
            "constructor_result_member" => self.parse_constructor_result_member().map(|_| self.pos()),
            "constructor_result" => self.parse_constructor_result().map(|_| self.pos()),
            "instantiated_type_member" => self.parse_instantiated_type_member().map(|_| self.pos()),
            "instantiated_type_reference" => self.parse_instantiated_type_reference().map(|_| self.pos()),
            "argument_list" => self.parse_argument_list().map(|_| self.pos()),
            "positional_argument_list" => self.parse_positional_argument_list().map(|_| self.pos()),
            "named_argument_list" => self.parse_named_argument_list().map(|_| self.pos()),
            "named_argument_member" => self.parse_named_argument_member().map(|_| self.pos()),
            "named_argument" => self.parse_named_argument().map(|_| self.pos()),
            "parameter_redefinition" => self.parse_parameter_redefinition().map(|_| self.pos()),
            "body_expression" => self.parse_body_expression().map(|_| self.pos()),
            "expression_body_member" => self.parse_expression_body_member().map(|_| self.pos()),
            "expression_body" => self.parse_expression_body().map(|_| self.pos()),
            "literal_expression" => self.parse_literal_expression().map(|_| self.pos()),
            "literal_boolean" => self.parse_literal_boolean().map(|_| self.pos()),
            "boolean_value" => self.parse_boolean_value().map(|_| self.pos()),
            "literal_string" => self.parse_literal_string().map(|_| self.pos()),
            "literal_integer" => self.parse_literal_integer().map(|_| self.pos()),
            "literal_real" => self.parse_literal_real().map(|_| self.pos()),
            "real_value" => self.parse_real_value().map(|_| self.pos()),
            "literal_infinity" => self.parse_literal_infinity().map(|_| self.pos()),
            "interaction" => self.parse_interaction().map(|_| self.pos()),
            "flow" => self.parse_flow().map(|_| self.pos()),
            "succession_flow" => self.parse_succession_flow().map(|_| self.pos()),
            "flow_declaration" => self.parse_flow_declaration().map(|_| self.pos()),
            "payload_feature_member" => self.parse_payload_feature_member().map(|_| self.pos()),
            "payload_feature" => self.parse_payload_feature().map(|_| self.pos()),
            "payload_feature_specialization_part" => self.parse_payload_feature_specialization_part().map(|_| self.pos()),
            "flow_end_member" => self.parse_flow_end_member().map(|_| self.pos()),
            "flow_end" => self.parse_flow_end().map(|_| self.pos()),
            "flow_feature_member" => self.parse_flow_feature_member().map(|_| self.pos()),
            "flow_feature" => self.parse_flow_feature().map(|_| self.pos()),
            "flow_feature_redefinition" => self.parse_flow_feature_redefinition().map(|_| self.pos()),
            "value_part" => self.parse_value_part().map(|_| self.pos()),
            "feature_value" => self.parse_feature_value().map(|_| self.pos()),
            "multiplicity" => self.parse_multiplicity().map(|_| self.pos()),
            "multiplicity_subset" => self.parse_multiplicity_subset().map(|_| self.pos()),
            "multiplicity_range" => self.parse_multiplicity_range().map(|_| self.pos()),
            "owned_multiplicity" => self.parse_owned_multiplicity().map(|_| self.pos()),
            "owned_multiplicity_range" => self.parse_owned_multiplicity_range().map(|_| self.pos()),
            "multiplicity_bounds" => self.parse_multiplicity_bounds().map(|_| self.pos()),
            "multiplicity_expression_member" => self.parse_multiplicity_expression_member().map(|_| self.pos()),
            "metaclass" => self.parse_metaclass().map(|_| self.pos()),
            "prefix_metadata_annotation" => self.parse_prefix_metadata_annotation().map(|_| self.pos()),
            "prefix_metadata_member" => self.parse_prefix_metadata_member().map(|_| self.pos()),
            "prefix_metadata_feature" => self.parse_prefix_metadata_feature().map(|_| self.pos()),
            "metadata_feature" => self.parse_metadata_feature().map(|_| self.pos()),
            "metadata_feature_declaration" => self.parse_metadata_feature_declaration().map(|_| self.pos()),
            "metadata_body" => self.parse_metadata_body().map(|_| self.pos()),
            "metadata_body_element" => self.parse_metadata_body_element().map(|_| self.pos()),
            "metadata_body_feature_member" => self.parse_metadata_body_feature_member().map(|_| self.pos()),
            "metadata_body_feature" => self.parse_metadata_body_feature().map(|_| self.pos()),
            "package" => self.parse_package().map(|_| self.pos()),
            "library_package" => self.parse_library_package().map(|_| self.pos()),
            "package_declaration" => self.parse_package_declaration().map(|_| self.pos()),
            "package_body" => self.parse_package_body().map(|_| self.pos()),
            "element_filter_member" => self.parse_element_filter_member().map(|_| self.pos()),
            _ => Err(ParseError { message: format!("unknown rule: {}", rule), span: self.current_span() }),
        }
    }

    /// Dispatch to a parse function by rule name, returning the AST node.
    pub fn try_parse_rule_ast(&mut self, rule: &str) -> Result<AstNodeKind> {
        match rule {
            "identification" => self.parse_identification().map(|v| AstNodeKind::Identification(Box::new(v))),
            "relationship_body" => self.parse_relationship_body().map(|v| AstNodeKind::RelationshipBody(Box::new(v))),
            "relationship_owned_element" => self.parse_relationship_owned_element().map(|v| AstNodeKind::RelationshipOwnedElement(Box::new(v))),
            "owned_related_element" => self.parse_owned_related_element().map(|v| AstNodeKind::OwnedRelatedElement(v)),
            "dependency" => self.parse_dependency().map(|v| AstNodeKind::Dependency(Box::new(v))),
            "annotation" => self.parse_annotation().map(|v| AstNodeKind::Annotation(Box::new(v))),
            "owned_annotation" => self.parse_owned_annotation().map(|v| AstNodeKind::OwnedAnnotation(Box::new(v))),
            "annotating_element" => self.parse_annotating_element().map(|v| AstNodeKind::AnnotatingElement(v)),
            "comment" => self.parse_comment().map(|v| AstNodeKind::Comment(Box::new(v))),
            "documentation" => self.parse_documentation().map(|v| AstNodeKind::Documentation(Box::new(v))),
            "textual_representation" => self.parse_textual_representation().map(|v| AstNodeKind::TextualRepresentation(Box::new(v))),
            "root_namespace" => self.parse_root_namespace().map(|v| AstNodeKind::RootNamespace(Box::new(v))),
            "namespace" => self.parse_namespace().map(|v| AstNodeKind::Namespace(Box::new(v))),
            "namespace_declaration" => self.parse_namespace_declaration().map(|v| AstNodeKind::NamespaceDeclaration(Box::new(v))),
            "namespace_body" => self.parse_namespace_body().map(|v| AstNodeKind::NamespaceBody(Box::new(v))),
            "namespace_body_element" => self.parse_namespace_body_element().map(|v| AstNodeKind::NamespaceBodyElement(Box::new(v))),
            "member_prefix" => self.parse_member_prefix().map(|v| AstNodeKind::MemberPrefix(Box::new(v))),
            "visibility_indicator" => self.parse_visibility_indicator().map(|v| AstNodeKind::VisibilityIndicator(Box::new(v))),
            "namespace_member" => self.parse_namespace_member().map(|v| AstNodeKind::NamespaceMember(v)),
            "non_feature_member" => self.parse_non_feature_member().map(|v| AstNodeKind::NonFeatureMember(Box::new(v))),
            "namespace_feature_member" => self.parse_namespace_feature_member().map(|v| AstNodeKind::NamespaceFeatureMember(Box::new(v))),
            "alias_member" => self.parse_alias_member().map(|v| AstNodeKind::AliasMember(Box::new(v))),
            "qualified_name" => self.parse_qualified_name().map(|v| AstNodeKind::QualifiedName(Box::new(v))),
            "import" => self.parse_import().map(|v| AstNodeKind::Import(Box::new(v))),
            "import_declaration" => self.parse_import_declaration().map(|v| AstNodeKind::ImportDeclaration(v)),
            "membership_import" => self.parse_membership_import().map(|v| AstNodeKind::MembershipImport(Box::new(v))),
            "namespace_import" => self.parse_namespace_import().map(|v| AstNodeKind::NamespaceImport(Box::new(v))),
            "filter_package" => self.parse_filter_package().map(|v| AstNodeKind::FilterPackage(Box::new(v))),
            "filter_package_member" => self.parse_filter_package_member().map(|v| AstNodeKind::FilterPackageMember(Box::new(v))),
            "member_element" => self.parse_member_element().map(|v| AstNodeKind::MemberElement(v)),
            "non_feature_element" => self.parse_non_feature_element().map(|v| AstNodeKind::NonFeatureElement(v)),
            "feature_element" => self.parse_feature_element().map(|v| AstNodeKind::FeatureElement(v)),
            "type_" => self.parse_type_().map(|v| AstNodeKind::Type(Box::new(v))),
            "type_prefix" => self.parse_type_prefix().map(|v| AstNodeKind::TypePrefix(Box::new(v))),
            "type_declaration" => self.parse_type_declaration().map(|v| AstNodeKind::TypeDeclaration(Box::new(v))),
            "specialization_part" => self.parse_specialization_part().map(|v| AstNodeKind::SpecializationPart(Box::new(v))),
            "conjugation_part" => self.parse_conjugation_part().map(|v| AstNodeKind::ConjugationPart(Box::new(v))),
            "type_relationship_part" => self.parse_type_relationship_part().map(|v| AstNodeKind::TypeRelationshipPart(v)),
            "disjoining_part" => self.parse_disjoining_part().map(|v| AstNodeKind::DisjoiningPart(Box::new(v))),
            "unioning_part" => self.parse_unioning_part().map(|v| AstNodeKind::UnioningPart(Box::new(v))),
            "intersecting_part" => self.parse_intersecting_part().map(|v| AstNodeKind::IntersectingPart(Box::new(v))),
            "differencing_part" => self.parse_differencing_part().map(|v| AstNodeKind::DifferencingPart(Box::new(v))),
            "type_body" => self.parse_type_body().map(|v| AstNodeKind::TypeBody(Box::new(v))),
            "type_body_element" => self.parse_type_body_element().map(|v| AstNodeKind::TypeBodyElement(Box::new(v))),
            "specialization" => self.parse_specialization().map(|v| AstNodeKind::Specialization(Box::new(v))),
            "owned_specialization" => self.parse_owned_specialization().map(|v| AstNodeKind::OwnedSpecialization(Box::new(v))),
            "specific_type" => self.parse_specific_type().map(|v| AstNodeKind::SpecificType(Box::new(v))),
            "general_type" => self.parse_general_type().map(|v| AstNodeKind::GeneralType(Box::new(v))),
            "conjugation" => self.parse_conjugation().map(|v| AstNodeKind::Conjugation(Box::new(v))),
            "owned_conjugation" => self.parse_owned_conjugation().map(|v| AstNodeKind::OwnedConjugation(Box::new(v))),
            "disjoining" => self.parse_disjoining().map(|v| AstNodeKind::Disjoining(Box::new(v))),
            "owned_disjoining" => self.parse_owned_disjoining().map(|v| AstNodeKind::OwnedDisjoining(Box::new(v))),
            "unioning" => self.parse_unioning().map(|v| AstNodeKind::Unioning(Box::new(v))),
            "intersecting" => self.parse_intersecting().map(|v| AstNodeKind::Intersecting(Box::new(v))),
            "differencing" => self.parse_differencing().map(|v| AstNodeKind::Differencing(Box::new(v))),
            "feature_member" => self.parse_feature_member().map(|v| AstNodeKind::FeatureMember(v)),
            "type_feature_member" => self.parse_type_feature_member().map(|v| AstNodeKind::TypeFeatureMember(Box::new(v))),
            "owned_feature_member" => self.parse_owned_feature_member().map(|v| AstNodeKind::OwnedFeatureMember(Box::new(v))),
            "classifier" => self.parse_classifier().map(|v| AstNodeKind::Classifier(Box::new(v))),
            "classifier_declaration" => self.parse_classifier_declaration().map(|v| AstNodeKind::ClassifierDeclaration(Box::new(v))),
            "superclassing_part" => self.parse_superclassing_part().map(|v| AstNodeKind::SuperclassingPart(Box::new(v))),
            "subclassification" => self.parse_subclassification().map(|v| AstNodeKind::Subclassification(Box::new(v))),
            "owned_subclassification" => self.parse_owned_subclassification().map(|v| AstNodeKind::OwnedSubclassification(Box::new(v))),
            "feature" => self.parse_feature().map(|v| AstNodeKind::Feature(Box::new(v))),
            "end_feature_prefix" => self.parse_end_feature_prefix().map(|v| AstNodeKind::EndFeaturePrefix(Box::new(v))),
            "basic_feature_prefix" => self.parse_basic_feature_prefix().map(|v| AstNodeKind::BasicFeaturePrefix(Box::new(v))),
            "feature_prefix" => self.parse_feature_prefix().map(|v| AstNodeKind::FeaturePrefix(Box::new(v))),
            "owned_cross_feature_member" => self.parse_owned_cross_feature_member().map(|v| AstNodeKind::OwnedCrossFeatureMember(Box::new(v))),
            "owned_cross_feature" => self.parse_owned_cross_feature().map(|v| AstNodeKind::OwnedCrossFeature(Box::new(v))),
            "feature_direction" => self.parse_feature_direction().map(|v| AstNodeKind::FeatureDirection(Box::new(v))),
            "feature_declaration" => self.parse_feature_declaration().map(|v| AstNodeKind::FeatureDeclaration(Box::new(v))),
            "feature_identification" => self.parse_feature_identification().map(|v| AstNodeKind::FeatureIdentification(Box::new(v))),
            "feature_relationship_part" => self.parse_feature_relationship_part().map(|v| AstNodeKind::FeatureRelationshipPart(v)),
            "chaining_part" => self.parse_chaining_part().map(|v| AstNodeKind::ChainingPart(Box::new(v))),
            "inverting_part" => self.parse_inverting_part().map(|v| AstNodeKind::InvertingPart(Box::new(v))),
            "type_featuring_part" => self.parse_type_featuring_part().map(|v| AstNodeKind::TypeFeaturingPart(Box::new(v))),
            "feature_specialization_part" => self.parse_feature_specialization_part().map(|v| AstNodeKind::FeatureSpecializationPart(Box::new(v))),
            "multiplicity_part" => self.parse_multiplicity_part().map(|v| AstNodeKind::MultiplicityPart(Box::new(v))),
            "feature_specialization" => self.parse_feature_specialization().map(|v| AstNodeKind::FeatureSpecialization(v)),
            "typings" => self.parse_typings().map(|v| AstNodeKind::Typings(Box::new(v))),
            "typed_by" => self.parse_typed_by().map(|v| AstNodeKind::TypedBy(Box::new(v))),
            "subsettings" => self.parse_subsettings().map(|v| AstNodeKind::Subsettings(Box::new(v))),
            "subsets" => self.parse_subsets().map(|v| AstNodeKind::Subsets(Box::new(v))),
            "references" => self.parse_references().map(|v| AstNodeKind::References(Box::new(v))),
            "crosses" => self.parse_crosses().map(|v| AstNodeKind::Crosses(Box::new(v))),
            "redefinitions" => self.parse_redefinitions().map(|v| AstNodeKind::Redefinitions(Box::new(v))),
            "redefines" => self.parse_redefines().map(|v| AstNodeKind::Redefines(Box::new(v))),
            "feature_typing" => self.parse_feature_typing().map(|v| AstNodeKind::FeatureTyping(Box::new(v))),
            "owned_feature_typing" => self.parse_owned_feature_typing().map(|v| AstNodeKind::OwnedFeatureTyping(Box::new(v))),
            "subsetting" => self.parse_subsetting().map(|v| AstNodeKind::Subsetting(Box::new(v))),
            "owned_subsetting" => self.parse_owned_subsetting().map(|v| AstNodeKind::OwnedSubsetting(Box::new(v))),
            "owned_reference_subsetting" => self.parse_owned_reference_subsetting().map(|v| AstNodeKind::OwnedReferenceSubsetting(Box::new(v))),
            "owned_cross_subsetting" => self.parse_owned_cross_subsetting().map(|v| AstNodeKind::OwnedCrossSubsetting(Box::new(v))),
            "redefinition" => self.parse_redefinition().map(|v| AstNodeKind::Redefinition(Box::new(v))),
            "owned_redefinition" => self.parse_owned_redefinition().map(|v| AstNodeKind::OwnedRedefinition(Box::new(v))),
            "owned_feature_chain" => self.parse_owned_feature_chain().map(|v| AstNodeKind::OwnedFeatureChain(Box::new(v))),
            "feature_chain" => self.parse_feature_chain().map(|v| AstNodeKind::FeatureChain(Box::new(v))),
            "owned_feature_chaining" => self.parse_owned_feature_chaining().map(|v| AstNodeKind::OwnedFeatureChaining(Box::new(v))),
            "feature_inverting" => self.parse_feature_inverting().map(|v| AstNodeKind::FeatureInverting(Box::new(v))),
            "owned_feature_inverting" => self.parse_owned_feature_inverting().map(|v| AstNodeKind::OwnedFeatureInverting(Box::new(v))),
            "type_featuring" => self.parse_type_featuring().map(|v| AstNodeKind::TypeFeaturing(Box::new(v))),
            "owned_type_featuring" => self.parse_owned_type_featuring().map(|v| AstNodeKind::OwnedTypeFeaturing(Box::new(v))),
            "data_type" => self.parse_data_type().map(|v| AstNodeKind::DataType(Box::new(v))),
            "class" => self.parse_class().map(|v| AstNodeKind::Class(Box::new(v))),
            "structure" => self.parse_structure().map(|v| AstNodeKind::Structure(Box::new(v))),
            "association" => self.parse_association().map(|v| AstNodeKind::Association(Box::new(v))),
            "association_structure" => self.parse_association_structure().map(|v| AstNodeKind::AssociationStructure(Box::new(v))),
            "connector" => self.parse_connector().map(|v| AstNodeKind::Connector(Box::new(v))),
            "connector_declaration" => self.parse_connector_declaration().map(|v| AstNodeKind::ConnectorDeclaration(v)),
            "binary_connector_declaration" => self.parse_binary_connector_declaration().map(|v| AstNodeKind::BinaryConnectorDeclaration(Box::new(v))),
            "nary_connector_declaration" => self.parse_nary_connector_declaration().map(|v| AstNodeKind::NaryConnectorDeclaration(Box::new(v))),
            "connector_end_member" => self.parse_connector_end_member().map(|v| AstNodeKind::ConnectorEndMember(Box::new(v))),
            "connector_end" => self.parse_connector_end().map(|v| AstNodeKind::ConnectorEnd(Box::new(v))),
            "owned_cross_multiplicity_member" => self.parse_owned_cross_multiplicity_member().map(|v| AstNodeKind::OwnedCrossMultiplicityMember(Box::new(v))),
            "owned_cross_multiplicity" => self.parse_owned_cross_multiplicity().map(|v| AstNodeKind::OwnedCrossMultiplicity(Box::new(v))),
            "binding_connector" => self.parse_binding_connector().map(|v| AstNodeKind::BindingConnector(Box::new(v))),
            "binding_connector_declaration" => self.parse_binding_connector_declaration().map(|v| AstNodeKind::BindingConnectorDeclaration(Box::new(v))),
            "succession" => self.parse_succession().map(|v| AstNodeKind::Succession(Box::new(v))),
            "succession_declaration" => self.parse_succession_declaration().map(|v| AstNodeKind::SuccessionDeclaration(Box::new(v))),
            "behavior" => self.parse_behavior().map(|v| AstNodeKind::Behavior(Box::new(v))),
            "step" => self.parse_step().map(|v| AstNodeKind::Step(Box::new(v))),
            "function" => self.parse_function().map(|v| AstNodeKind::Function(Box::new(v))),
            "function_body" => self.parse_function_body().map(|v| AstNodeKind::FunctionBody(Box::new(v))),
            "function_body_part" => self.parse_function_body_part().map(|v| AstNodeKind::FunctionBodyPart(Box::new(v))),
            "return_feature_member" => self.parse_return_feature_member().map(|v| AstNodeKind::ReturnFeatureMember(Box::new(v))),
            "result_expression_member" => self.parse_result_expression_member().map(|v| AstNodeKind::ResultExpressionMember(Box::new(v))),
            "expression" => self.parse_expression().map(|v| AstNodeKind::Expression(Box::new(v))),
            "predicate" => self.parse_predicate().map(|v| AstNodeKind::Predicate(Box::new(v))),
            "boolean_expression" => self.parse_boolean_expression().map(|v| AstNodeKind::BooleanExpression(Box::new(v))),
            "invariant" => self.parse_invariant().map(|v| AstNodeKind::Invariant(Box::new(v))),
            "owned_expression_reference_member" => self.parse_owned_expression_reference_member().map(|v| AstNodeKind::OwnedExpressionReferenceMember(Box::new(v))),
            "owned_expression_reference" => self.parse_owned_expression_reference().map(|v| AstNodeKind::OwnedExpressionReference(Box::new(v))),
            "owned_expression_member" => self.parse_owned_expression_member().map(|v| AstNodeKind::OwnedExpressionMember(Box::new(v))),
            "owned_expression" => self.parse_owned_expression().map(|v| AstNodeKind::OwnedExpression(v)),
            "conditional_expression" => self.parse_conditional_expression().map(|v| AstNodeKind::ConditionalExpression(Box::new(v))),
            "conditional_binary_operator_expression" => self.parse_conditional_binary_operator_expression().map(|v| AstNodeKind::ConditionalBinaryOperatorExpression(Box::new(v))),
            "conditional_binary_operator" => self.parse_conditional_binary_operator().map(|v| AstNodeKind::ConditionalBinaryOperator(Box::new(v))),
            "binary_operator_expression" => self.parse_binary_operator_expression().map(|v| AstNodeKind::BinaryOperatorExpression(Box::new(v))),
            "binary_operator" => self.parse_binary_operator().map(|v| AstNodeKind::BinaryOperator(Box::new(v))),
            "unary_operator_expression" => self.parse_unary_operator_expression().map(|v| AstNodeKind::UnaryOperatorExpression(Box::new(v))),
            "unary_operator" => self.parse_unary_operator().map(|v| AstNodeKind::UnaryOperator(Box::new(v))),
            "classification_expression" => self.parse_classification_expression().map(|v| AstNodeKind::ClassificationExpression(Box::new(v))),
            "classification_test_operator" => self.parse_classification_test_operator().map(|v| AstNodeKind::ClassificationTestOperator(Box::new(v))),
            "cast_operator" => self.parse_cast_operator().map(|v| AstNodeKind::CastOperator(Box::new(v))),
            "metaclassification_expression" => self.parse_metaclassification_expression().map(|v| AstNodeKind::MetaclassificationExpression(Box::new(v))),
            "argument_member" => self.parse_argument_member().map(|v| AstNodeKind::ArgumentMember(Box::new(v))),
            "argument" => self.parse_argument().map(|v| AstNodeKind::Argument(Box::new(v))),
            "argument_value" => self.parse_argument_value().map(|v| AstNodeKind::ArgumentValue(Box::new(v))),
            "argument_expression_member" => self.parse_argument_expression_member().map(|v| AstNodeKind::ArgumentExpressionMember(Box::new(v))),
            "argument_expression" => self.parse_argument_expression().map(|v| AstNodeKind::ArgumentExpression(Box::new(v))),
            "argument_expression_value" => self.parse_argument_expression_value().map(|v| AstNodeKind::ArgumentExpressionValue(Box::new(v))),
            "metadata_argument_member" => self.parse_metadata_argument_member().map(|v| AstNodeKind::MetadataArgumentMember(Box::new(v))),
            "metadata_argument" => self.parse_metadata_argument().map(|v| AstNodeKind::MetadataArgument(Box::new(v))),
            "metadata_value" => self.parse_metadata_value().map(|v| AstNodeKind::MetadataValue(Box::new(v))),
            "metadata_reference" => self.parse_metadata_reference().map(|v| AstNodeKind::MetadataReference(Box::new(v))),
            "metaclassification_test_operator" => self.parse_metaclassification_test_operator().map(|v| AstNodeKind::MetaclassificationTestOperator(Box::new(v))),
            "meta_cast_operator" => self.parse_meta_cast_operator().map(|v| AstNodeKind::MetaCastOperator(Box::new(v))),
            "extent_expression" => self.parse_extent_expression().map(|v| AstNodeKind::ExtentExpression(Box::new(v))),
            "type_reference_member" => self.parse_type_reference_member().map(|v| AstNodeKind::TypeReferenceMember(Box::new(v))),
            "type_result_member" => self.parse_type_result_member().map(|v| AstNodeKind::TypeResultMember(Box::new(v))),
            "type_reference" => self.parse_type_reference().map(|v| AstNodeKind::TypeReference(Box::new(v))),
            "reference_typing" => self.parse_reference_typing().map(|v| AstNodeKind::ReferenceTyping(Box::new(v))),
            "empty_result_member" => self.parse_empty_result_member().map(|v| AstNodeKind::EmptyResultMember(Box::new(v))),
            "empty_feature" => self.parse_empty_feature().map(|v| AstNodeKind::EmptyFeature(Box::new(v))),
            "primary_expression" => self.parse_primary_expression().map(|v| AstNodeKind::PrimaryExpression(v)),
            "primary_argument_value" => self.parse_primary_argument_value().map(|v| AstNodeKind::PrimaryArgumentValue(Box::new(v))),
            "primary_argument" => self.parse_primary_argument().map(|v| AstNodeKind::PrimaryArgument(Box::new(v))),
            "primary_argument_member" => self.parse_primary_argument_member().map(|v| AstNodeKind::PrimaryArgumentMember(Box::new(v))),
            "non_feature_chain_primary_expression" => self.parse_non_feature_chain_primary_expression().map(|v| AstNodeKind::NonFeatureChainPrimaryExpression(v)),
            "non_feature_chain_primary_argument_value" => self.parse_non_feature_chain_primary_argument_value().map(|v| AstNodeKind::NonFeatureChainPrimaryArgumentValue(Box::new(v))),
            "non_feature_chain_primary_argument" => self.parse_non_feature_chain_primary_argument().map(|v| AstNodeKind::NonFeatureChainPrimaryArgument(Box::new(v))),
            "non_feature_chain_primary_argument_member" => self.parse_non_feature_chain_primary_argument_member().map(|v| AstNodeKind::NonFeatureChainPrimaryArgumentMember(Box::new(v))),
            "bracket_expression" => self.parse_bracket_expression().map(|v| AstNodeKind::BracketExpression(Box::new(v))),
            "index_expression" => self.parse_index_expression().map(|v| AstNodeKind::IndexExpression(Box::new(v))),
            "sequence_expression" => self.parse_sequence_expression().map(|v| AstNodeKind::SequenceExpression(Box::new(v))),
            "sequence_expression_list" => self.parse_sequence_expression_list().map(|v| AstNodeKind::SequenceExpressionList(Box::new(v))),
            "sequence_operator_expression" => self.parse_sequence_operator_expression().map(|v| AstNodeKind::SequenceOperatorExpression(Box::new(v))),
            "sequence_expression_list_member" => self.parse_sequence_expression_list_member().map(|v| AstNodeKind::SequenceExpressionListMember(Box::new(v))),
            "feature_chain_expression" => self.parse_feature_chain_expression().map(|v| AstNodeKind::FeatureChainExpression(Box::new(v))),
            "collect_expression" => self.parse_collect_expression().map(|v| AstNodeKind::CollectExpression(Box::new(v))),
            "select_expression" => self.parse_select_expression().map(|v| AstNodeKind::SelectExpression(Box::new(v))),
            "function_operation_expression" => self.parse_function_operation_expression().map(|v| AstNodeKind::FunctionOperationExpression(Box::new(v))),
            "body_argument_member" => self.parse_body_argument_member().map(|v| AstNodeKind::BodyArgumentMember(Box::new(v))),
            "body_argument" => self.parse_body_argument().map(|v| AstNodeKind::BodyArgument(Box::new(v))),
            "body_argument_value" => self.parse_body_argument_value().map(|v| AstNodeKind::BodyArgumentValue(Box::new(v))),
            "function_reference_argument_member" => self.parse_function_reference_argument_member().map(|v| AstNodeKind::FunctionReferenceArgumentMember(Box::new(v))),
            "function_reference_argument" => self.parse_function_reference_argument().map(|v| AstNodeKind::FunctionReferenceArgument(Box::new(v))),
            "function_reference_argument_value" => self.parse_function_reference_argument_value().map(|v| AstNodeKind::FunctionReferenceArgumentValue(Box::new(v))),
            "function_reference_expression" => self.parse_function_reference_expression().map(|v| AstNodeKind::FunctionReferenceExpression(Box::new(v))),
            "function_reference_member" => self.parse_function_reference_member().map(|v| AstNodeKind::FunctionReferenceMember(Box::new(v))),
            "function_reference" => self.parse_function_reference().map(|v| AstNodeKind::FunctionReference(Box::new(v))),
            "feature_chain_member" => self.parse_feature_chain_member().map(|v| AstNodeKind::FeatureChainMember(v)),
            "owned_feature_chain_member" => self.parse_owned_feature_chain_member().map(|v| AstNodeKind::OwnedFeatureChainMember(Box::new(v))),
            "base_expression" => self.parse_base_expression().map(|v| AstNodeKind::BaseExpression(v)),
            "null_expression" => self.parse_null_expression().map(|v| AstNodeKind::NullExpression(Box::new(v))),
            "feature_reference_expression" => self.parse_feature_reference_expression().map(|v| AstNodeKind::FeatureReferenceExpression(Box::new(v))),
            "feature_reference_member" => self.parse_feature_reference_member().map(|v| AstNodeKind::FeatureReferenceMember(Box::new(v))),
            "feature_reference" => self.parse_feature_reference().map(|v| AstNodeKind::FeatureReference(Box::new(v))),
            "metadata_access_expression" => self.parse_metadata_access_expression().map(|v| AstNodeKind::MetadataAccessExpression(Box::new(v))),
            "element_reference_member" => self.parse_element_reference_member().map(|v| AstNodeKind::ElementReferenceMember(Box::new(v))),
            "invocation_expression" => self.parse_invocation_expression().map(|v| AstNodeKind::InvocationExpression(Box::new(v))),
            "constructor_expression" => self.parse_constructor_expression().map(|v| AstNodeKind::ConstructorExpression(Box::new(v))),
            "constructor_result_member" => self.parse_constructor_result_member().map(|v| AstNodeKind::ConstructorResultMember(Box::new(v))),
            "constructor_result" => self.parse_constructor_result().map(|v| AstNodeKind::ConstructorResult(Box::new(v))),
            "instantiated_type_member" => self.parse_instantiated_type_member().map(|v| AstNodeKind::InstantiatedTypeMember(Box::new(v))),
            "instantiated_type_reference" => self.parse_instantiated_type_reference().map(|v| AstNodeKind::InstantiatedTypeReference(Box::new(v))),
            "argument_list" => self.parse_argument_list().map(|v| AstNodeKind::ArgumentList(Box::new(v))),
            "positional_argument_list" => self.parse_positional_argument_list().map(|v| AstNodeKind::PositionalArgumentList(Box::new(v))),
            "named_argument_list" => self.parse_named_argument_list().map(|v| AstNodeKind::NamedArgumentList(Box::new(v))),
            "named_argument_member" => self.parse_named_argument_member().map(|v| AstNodeKind::NamedArgumentMember(Box::new(v))),
            "named_argument" => self.parse_named_argument().map(|v| AstNodeKind::NamedArgument(Box::new(v))),
            "parameter_redefinition" => self.parse_parameter_redefinition().map(|v| AstNodeKind::ParameterRedefinition(Box::new(v))),
            "body_expression" => self.parse_body_expression().map(|v| AstNodeKind::BodyExpression(Box::new(v))),
            "expression_body_member" => self.parse_expression_body_member().map(|v| AstNodeKind::ExpressionBodyMember(Box::new(v))),
            "expression_body" => self.parse_expression_body().map(|v| AstNodeKind::ExpressionBody(Box::new(v))),
            "literal_expression" => self.parse_literal_expression().map(|v| AstNodeKind::LiteralExpression(v)),
            "literal_boolean" => self.parse_literal_boolean().map(|v| AstNodeKind::LiteralBoolean(Box::new(v))),
            "boolean_value" => self.parse_boolean_value().map(|v| AstNodeKind::BooleanValue(Box::new(v))),
            "literal_string" => self.parse_literal_string().map(|v| AstNodeKind::LiteralString(Box::new(v))),
            "literal_integer" => self.parse_literal_integer().map(|v| AstNodeKind::LiteralInteger(Box::new(v))),
            "literal_real" => self.parse_literal_real().map(|v| AstNodeKind::LiteralReal(Box::new(v))),
            "real_value" => self.parse_real_value().map(|v| AstNodeKind::RealValue(Box::new(v))),
            "literal_infinity" => self.parse_literal_infinity().map(|v| AstNodeKind::LiteralInfinity(Box::new(v))),
            "interaction" => self.parse_interaction().map(|v| AstNodeKind::Interaction(Box::new(v))),
            "flow" => self.parse_flow().map(|v| AstNodeKind::Flow(Box::new(v))),
            "succession_flow" => self.parse_succession_flow().map(|v| AstNodeKind::SuccessionFlow(Box::new(v))),
            "flow_declaration" => self.parse_flow_declaration().map(|v| AstNodeKind::FlowDeclaration(Box::new(v))),
            "payload_feature_member" => self.parse_payload_feature_member().map(|v| AstNodeKind::PayloadFeatureMember(Box::new(v))),
            "payload_feature" => self.parse_payload_feature().map(|v| AstNodeKind::PayloadFeature(Box::new(v))),
            "payload_feature_specialization_part" => self.parse_payload_feature_specialization_part().map(|v| AstNodeKind::PayloadFeatureSpecializationPart(Box::new(v))),
            "flow_end_member" => self.parse_flow_end_member().map(|v| AstNodeKind::FlowEndMember(Box::new(v))),
            "flow_end" => self.parse_flow_end().map(|v| AstNodeKind::FlowEnd(Box::new(v))),
            "flow_feature_member" => self.parse_flow_feature_member().map(|v| AstNodeKind::FlowFeatureMember(Box::new(v))),
            "flow_feature" => self.parse_flow_feature().map(|v| AstNodeKind::FlowFeature(Box::new(v))),
            "flow_feature_redefinition" => self.parse_flow_feature_redefinition().map(|v| AstNodeKind::FlowFeatureRedefinition(Box::new(v))),
            "value_part" => self.parse_value_part().map(|v| AstNodeKind::ValuePart(Box::new(v))),
            "feature_value" => self.parse_feature_value().map(|v| AstNodeKind::FeatureValue(Box::new(v))),
            "multiplicity" => self.parse_multiplicity().map(|v| AstNodeKind::Multiplicity(v)),
            "multiplicity_subset" => self.parse_multiplicity_subset().map(|v| AstNodeKind::MultiplicitySubset(Box::new(v))),
            "multiplicity_range" => self.parse_multiplicity_range().map(|v| AstNodeKind::MultiplicityRange(Box::new(v))),
            "owned_multiplicity" => self.parse_owned_multiplicity().map(|v| AstNodeKind::OwnedMultiplicity(Box::new(v))),
            "owned_multiplicity_range" => self.parse_owned_multiplicity_range().map(|v| AstNodeKind::OwnedMultiplicityRange(Box::new(v))),
            "multiplicity_bounds" => self.parse_multiplicity_bounds().map(|v| AstNodeKind::MultiplicityBounds(Box::new(v))),
            "multiplicity_expression_member" => self.parse_multiplicity_expression_member().map(|v| AstNodeKind::MultiplicityExpressionMember(Box::new(v))),
            "metaclass" => self.parse_metaclass().map(|v| AstNodeKind::Metaclass(Box::new(v))),
            "prefix_metadata_annotation" => self.parse_prefix_metadata_annotation().map(|v| AstNodeKind::PrefixMetadataAnnotation(Box::new(v))),
            "prefix_metadata_member" => self.parse_prefix_metadata_member().map(|v| AstNodeKind::PrefixMetadataMember(Box::new(v))),
            "prefix_metadata_feature" => self.parse_prefix_metadata_feature().map(|v| AstNodeKind::PrefixMetadataFeature(Box::new(v))),
            "metadata_feature" => self.parse_metadata_feature().map(|v| AstNodeKind::MetadataFeature(Box::new(v))),
            "metadata_feature_declaration" => self.parse_metadata_feature_declaration().map(|v| AstNodeKind::MetadataFeatureDeclaration(Box::new(v))),
            "metadata_body" => self.parse_metadata_body().map(|v| AstNodeKind::MetadataBody(Box::new(v))),
            "metadata_body_element" => self.parse_metadata_body_element().map(|v| AstNodeKind::MetadataBodyElement(v)),
            "metadata_body_feature_member" => self.parse_metadata_body_feature_member().map(|v| AstNodeKind::MetadataBodyFeatureMember(Box::new(v))),
            "metadata_body_feature" => self.parse_metadata_body_feature().map(|v| AstNodeKind::MetadataBodyFeature(Box::new(v))),
            "package" => self.parse_package().map(|v| AstNodeKind::Package(Box::new(v))),
            "library_package" => self.parse_library_package().map(|v| AstNodeKind::LibraryPackage(Box::new(v))),
            "package_declaration" => self.parse_package_declaration().map(|v| AstNodeKind::PackageDeclaration(Box::new(v))),
            "package_body" => self.parse_package_body().map(|v| AstNodeKind::PackageBody(Box::new(v))),
            "element_filter_member" => self.parse_element_filter_member().map(|v| AstNodeKind::ElementFilterMember(Box::new(v))),
            _ => Err(ParseError { message: format!("unknown rule: {}", rule), span: self.current_span() }),
        }
    }
}
