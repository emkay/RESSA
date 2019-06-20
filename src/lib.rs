//! RESSA (Rusty ECMAScript Syntax Analyzer)
//! A library for parsing js files
//!
//! The main interface for this library would be
//! the `Parser` iterator. A parser is constructed
//! either via the `::new()` function or a `Builder`.
//! As part of the constructor, you have to provide
//! the js you want to parse as an `&str`.
//!
//! Once constructed the parser will return a
//! `ProgramPart` for each iteration.
//!
//! A very simple example might look like this
//! ```
//! use ressa::Parser;
//! use resast::prelude::*;
//! fn main() {
//!     let js = "function helloWorld() { alert('Hello world'); }";
//!     let p = Parser::new(&js).unwrap();
//!     let f = ProgramPart::decl(
//!         Decl::Function(
//!             Function {
//!                 id: Some("helloWorld".to_string()),
//!                 params: vec![],
//!                 body: vec![
//!                     ProgramPart::Stmt(
//!                         Stmt::Expr(
//!                             Expr::call(Expr::ident("alert"), vec![Expr::string("'Hello world'")])
//!                         )
//!                     )
//!                 ],
//!                 generator: false,
//!                 is_async: false,
//!             }
//!         )
//!     );
//!     for part in p {
//!         // assert_eq!(part.unwrap(), f);
//!     }
//! }
//!```
//! checkout the `examples` folders for slightly larger
//! examples.
//!
extern crate ress;
#[macro_use]
extern crate log;
extern crate backtrace;
extern crate env_logger;

use ress::{Item, Keyword, Punct, RefToken as Token, Scanner, Span, Token as _};

mod comment_handler;
mod error;

pub use crate::comment_handler::CommentHandler;
use crate::comment_handler::DefaultCommentHandler;
pub use crate::error::Error;
use resast::{prelude::VariableKind, ref_tree::prelude::*};
use std::{collections::HashSet, mem::replace};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl ::std::fmt::Display for Position {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// The current configuration options.
/// This will most likely increase over time
struct Config {
    /// whether or not to tolerate a subset of errors
    tolerant: bool,
}

/// The current parsing context.
/// This structure holds the relevant
/// information to know when some
/// text might behave differently
/// depending on what has come before it
struct Context<'a> {
    /// If the current JS should be treated
    /// as a JS module
    is_module: bool,
    /// If `in` is allowed as an identifier
    allow_in: bool,
    /// If a strict directive is allowed
    allow_strict_directive: bool,
    /// If `yield` is allowed as an identifier
    allow_yield: bool,
    /// If await is allowed as an identifier
    r#await: bool,
    /// If we have found any possible naming errors
    /// which are not yet resolved
    first_covert_initialized_name_error: Option<Item<Token<'a>>>,
    /// If the current expressions is an assignment target
    is_assignment_target: bool,
    /// If the current expression is a binding element
    is_binding_element: bool,
    /// If we have entered a function body
    in_function_body: bool,
    /// If we have entered a loop block
    in_iteration: bool,
    /// If we have entered a switch block
    in_switch: bool,
    /// The currently known labels, this applies
    /// to labels only, not all identifiers. Errors
    /// at that level would need to be handled by
    /// the calling scope
    label_set: HashSet<String>,
    /// If the current scope has a `'use strict';` directive
    /// in the prelude
    strict: bool,
    /// If the scanner has a pending line terminator
    /// before the next token
    has_line_term: bool,
    /// If we have passed the initial prelude where a valid
    /// `'use strict'` directive would exist
    past_prolog: bool,
    /// If we encounter an error, the iterator should stop
    errored: bool,
}

impl Default for Config {
    fn default() -> Self {
        trace!("default config");
        Self { tolerant: false }
    }
}

impl<'a> Default for Context<'a> {
    fn default() -> Self {
        trace!("default context",);
        Self {
            is_module: false,
            r#await: false,
            allow_in: true,
            allow_strict_directive: true,
            allow_yield: true,
            first_covert_initialized_name_error: None,
            is_assignment_target: false,
            is_binding_element: false,
            in_function_body: false,
            in_iteration: false,
            in_switch: false,
            label_set: HashSet::new(),
            strict: false,
            has_line_term: false,
            past_prolog: false,
            errored: false,
        }
    }
}
/// This is used to create a `Parser` using
/// the builder method
/// ```
/// use ressa::Builder;
/// use resast::prelude::*;
/// fn main() {
///     //et js = "for (var i = 0; i < 100; i++) {
///     //onsole.log('loop', i);
///     //";
///     //et p = Builder::new().module(false).js(js).build().unwrap();
///     //or part in p {
///     //   let var = VariableDecl {
///     //       id: Pat::Identifier("i".to_string()),
///     //       init: Some(Expr::Literal(Literal::Number("0".to_string())))
///     //   };
///     //   let test = Expr::Binary(BinaryExpr {
///     //       left: Box::new(Expr::Ident("i".to_string())),
///     //       operator: BinaryOperator::LessThan,
///     //       right: Box::new(Expr::Literal(Literal::Number("100".to_string()))),
///     //   });
///     //   let body = Box::new(Stmt::Block(vec![
///     //       ProgramPart::Stmt(
///     //           Stmt::Expr(
///     //               Expr::Call(
///     //                   CallExpr {
///     //                       callee: Box::new(Expr::Member(MemberExpr {
///     //                           object: Box::new(Expr::Ident("console".to_string())),
///     //                           property: Box::new(Expr::Ident("log".to_string())),
///     //                           computed: false,
///     //                       })),
///     //                       arguments: vec![Expr::Literal(Literal::String("'loop'".to_string())), Expr::Ident("i".to_string())]
///     //                   }
///     //               )
///     //           )
///     //       )
///     //   ]));
///     //   let expecation = ProgramPart::Stmt(Stmt::For(ForStmt {
///     //       init: Some(LoopInit::Variable(VariableKind::Var, vec![var])),
///     //       test: Some(test),
///     //       update: Some(Expr::Update(UpdateExpr {
///     //           operator: UpdateOperator::Increment,
///     //           argument: Box::new(Expr::Ident("i".to_string())),
///     //           prefix: false,
///     //       })),
///     //       body,
///     //   }));
///     //   //assert_eq!(part.unwrap(), expecation);
///     }
/// }
/// ```
pub struct Builder<'b> {
    tolerant: bool,
    is_module: bool,
    js: &'b str,
}

impl<'b> Builder<'b> {
    pub fn new() -> Self {
        Self {
            tolerant: false,
            is_module: false,
            js: "",
        }
    }
    /// Enable or disable error tolerance
    /// default: `false`
    pub fn set_tolerant(&mut self, value: bool) {
        self.tolerant = value;
    }
    /// Enable or disable error tolerance with a builder
    /// pattern
    /// default: `false`
    pub fn tolerant(&mut self, value: bool) -> &mut Self {
        self.set_tolerant(value);
        self
    }
    /// Set the parsing context to module or script
    /// default: `false` (script)
    pub fn set_module(&mut self, value: bool) {
        self.is_module = value;
    }
    /// Set the parsing context to module or script
    /// with a builder pattern
    /// default: `false` (script)
    pub fn module(&mut self, value: bool) -> &mut Self {
        self.set_module(value);
        self
    }
    /// Set the js text that this parser would operate
    /// on
    pub fn set_js(&mut self, js: &'b str) {
        self.js = js;
    }
    /// Set the js text that this parser would operate
    /// on with a builder pattern
    pub fn js(&mut self, js: &'b str) -> &mut Self {
        self.set_js(js);
        self
    }
    /// Complete the builder pattern returning
    /// `Result<Parser, Error>`
    pub fn build(&self) -> Res<Parser<DefaultCommentHandler>> {
        let is_module = self.is_module;
        let tolerant = self.tolerant;
        let lines = get_lines(&self.js);
        let scanner = Scanner::new(self.js.clone());
        Parser::build(
            tolerant,
            is_module,
            scanner,
            lines,
            DefaultCommentHandler,
            self.js,
        )
    }
}

/// This is the primary interface that you would interact with.
/// There are two main ways to use it, the first is to utilize
/// the `Iterator` implementation. Each iteration will return
/// a `Result<ProgramPart, Error>`.
/// The other option is to use the `parse` method, which is just
/// a wrapper around the `collect` method on `Iterator`, however
/// the final result will be a `Result<Program, Error>` and the
/// `ProgramPart` collection will be the inner data. Since modern
/// js allows for both `Module`s as well as `Script`s, these will be
/// the two `enum` variants.
pub struct Parser<'a, CH>
where
    CH: CommentHandler<'a> + Sized,
{
    /// The current parsing context
    context: Context<'a>,
    /// The configuration provided by the user
    config: Config,
    /// The internal scanner (see the
    /// `ress` crate for more details)
    scanner: Scanner<'a>,
    /// The indexes that each line starts/ends at
    lines: Vec<Line>,
    /// The next item,
    look_ahead: Item<Token<'a>>,
    /// Since we are looking ahead, we need
    /// to make sure we don't miss the eof
    /// by using this flag
    found_eof: bool,
    /// a possible container for tokens, currently
    /// it is unused
    _tokens: Vec<Item<Token<'a>>>,
    /// a possible container for comments, currently
    /// it is unused
    _comments: Vec<Item<Token<'a>>>,
    /// The current position we are parsing
    current_position: Position,
    look_ahead_position: Position,
    /// To ease debugging this will be a String representation
    /// of the look_ahead token, it will be an empty string
    /// unless you are using the `debug_look_ahead` feature
    _look_ahead: String,

    comment_handler: CH,
    original: &'a str,
    last_line_idx: usize,
}
/// The start/end index of a line
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Line {
    start: usize,
    end: usize,
}
/// The result type for the Parser operations
type Res<T> = Result<T, Error>;
/// Capture the lines from the raw js text
fn get_lines(text: &str) -> Vec<Line> {
    // Obviously we will start at 0
    let mut line_start = 0;
    // This is the byte position, not the character
    // position to account for multi byte chars
    let mut byte_position = 0;
    // loop over the characters
    let mut ret: Vec<Line> = text
        .chars()
        .filter_map(|c| {
            let ret = match c {
                '\r' => {
                    // look ahead 1 char to see if it is a newline pair
                    // if so, don't include it, it will get included in the next
                    // iteration
                    if let Some(next) = text.get(byte_position..byte_position + 2) {
                        if next == "\r\n" {
                            None
                        } else {
                            let ret = Line {
                                start: line_start,
                                end: byte_position,
                            };
                            line_start = byte_position + 1;
                            Some(ret)
                        }
                    } else {
                        None
                    }
                }
                '\n' => {
                    let ret = Line {
                        start: line_start,
                        end: byte_position,
                    };
                    line_start = byte_position + 1;
                    Some(ret)
                }
                '\u{2028}' | '\u{2029}' => {
                    //These new line characters are both 3 bytes in length
                    //that means we need to include this calculation in both
                    // the end field and the next line start
                    let ret = Line {
                        start: line_start,
                        end: byte_position + 2,
                    };
                    line_start = byte_position + 3;
                    Some(ret)
                }
                _ => None,
            };
            // Since chars can be up to 4 bytes wide, we
            // want to move the full width of the current byte
            byte_position += c.len_utf8();
            ret
        })
        .collect();
    // Since we shouldn't have only a new line char at EOF,
    // This will capture the last line of the text
    ret.push(Line {
        start: line_start,
        end: text.len().saturating_sub(1),
    });
    ret
}

impl<'a> Parser<'a, DefaultCommentHandler> {
    /// Create a new parser with the provided
    /// javascript
    /// This will default to parsing in the
    /// script context and discard comments.
    /// If you wanted change this behavior
    /// utilize the `Builder` pattern
    pub fn new(text: &'a str) -> Res<Self> {
        let lines = get_lines(text);
        let s = Scanner::new(text);
        let config = Config::default();
        let context = Context::default();
        Self::_new(s, lines, config, context, DefaultCommentHandler, text)
    }
}

impl<'b, CH> Parser<'b, CH>
where
    CH: CommentHandler<'b> + Sized,
{
    /// Internal constructor for completing the builder pattern
    pub fn build(
        tolerant: bool,
        is_module: bool,
        scanner: Scanner<'b>,
        lines: Vec<Line>,
        comment_handler: CH,
        original: &'b str,
    ) -> Res<Self> {
        let config = Config {
            tolerant,
            ..Default::default()
        };
        let context = Context {
            is_module,
            ..Default::default()
        };
        Self::_new(scanner, lines, config, context, comment_handler, original)
    }
    /// Internal constructor to allow for both builder pattern
    /// and `new` construction
    fn _new(
        scanner: Scanner<'b>,
        lines: Vec<Line>,
        config: Config,
        context: Context<'b>,
        comment_handler: CH,
        original: &'b str,
    ) -> Res<Self> {
        let look_ahead = Item {
            token: Token::EoF,
            span: Span { start: 0, end: 0 },
        };
        let mut ret = Self {
            scanner,
            look_ahead,
            lines,
            found_eof: false,
            config,
            context,
            _tokens: vec![],
            _comments: vec![],
            current_position: Position { line: 1, column: 0 },
            look_ahead_position: Position { line: 1, column: 0 },
            _look_ahead: String::new(),
            comment_handler,
            original,
            last_line_idx: 0,
        };
        let _ = ret.next_item()?;
        Ok(ret)
    }
    /// Wrapper around the `Iterator` implementation for
    /// Parser
    /// ```
    /// extern crate ressa;
    /// use ressa::Parser;
    /// use resast::prelude::*;
    /// fn main() {
    ///     let js = "function helloWorld() { alert('Hello world'); }";
    ///     let mut p = Parser::new(&js).unwrap();
    ///     let call = CallExpr {
    ///         callee: Box::new(Expr::Ident(String::from("alert"))),
    ///         arguments: vec![Expr::Literal(Literal::String(String::from("'Hello world'")))],
    ///     };
    ///     let expectation = Program::Script(vec![ProgramPart::Decl(Decl::Function(Function {
    ///         id: Some("helloWorld".to_string()),
    ///         params: vec![],
    ///         body: vec![ProgramPart::Stmt(Stmt::Expr(Expr::Call(call)))],
    ///         generator: false,
    ///         is_async: false,
    ///     }))]);
    ///     let program = p.parse().unwrap();
    ///     //assert_eq!(program, expectation);
    /// }
    /// ```
    pub fn parse(&mut self) -> Res<Program> {
        debug!("{}: parse_script, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut body = vec![];
        while let Some(part) = self.next() {
            match part {
                Ok(part) => body.push(part),
                Err(e) => return Err(e),
            }
        }
        Ok(if self.context.is_module {
            Program::Mod(body)
        } else {
            Program::Script(body)
        })
    }
    /// Parse all of the directives into a single prologue
    #[inline]
    fn parse_directive_prologues(&mut self) -> Res<Vec<ProgramPart<'b>>> {
        debug!("{}: parse_directive_prologues, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut ret = vec![];
        loop {
            if !self.look_ahead.token.is_string() {
                break;
            }
            ret.push(self.parse_directive()?);
        }
        Ok(ret)
    }
    /// Parse a single directive
    #[inline]
    fn parse_directive(&mut self) -> Res<ProgramPart<'b>> {
        debug!("{}: parse_directive, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let orig = self.look_ahead.clone();
        let expr = self.parse_expression()?;
        if let Expr::Literal(lit) = expr {
            if let Literal::String(s) = lit {
                if !self.context.allow_strict_directive && s == "use strict" {
                    return self.unexpected_token_error(&orig, "use strict in an invalid location");
                }
                self.consume_semicolon()?;
                return Ok(ProgramPart::Dir(Dir {
                    dir: s.trim_matches(|c| c == '\'' || c == '"'),
                    expr: Literal::String(s),
                }));
            } else {
                return Ok(ProgramPart::Stmt(Stmt::Expr(Expr::Literal(lit))));
            }
        } else {
            let stmt = ProgramPart::Stmt(Stmt::Expr(expr));
            self.consume_semicolon()?;
            Ok(stmt)
        }
    }
    /// This is where we will begin our recursive decent. First
    /// we check to see if we are at at token that is a known
    /// statement or declaration (import/export/function/const/let/class)
    /// otherwise we move on to `Parser::parse_statement`
    #[inline]
    fn parse_statement_list_item(&mut self) -> Res<ProgramPart<'b>> {
        debug!(
            "{}: parse_statement_list_item_script {:?}",
            self.look_ahead.span.start,
            self.look_ahead.token
        );
        self.context.is_assignment_target = true;
        self.context.is_binding_element = true;
        let tok = self.look_ahead.token.clone();
        let ret = match &tok {
            Token::Keyword(ref k) => match k {
                &Keyword::Import => {
                    if self.at_import_call() {
                        let stmt = self.parse_statement()?;
                        Ok(ProgramPart::Stmt(stmt))
                    } else {
                        if !self.context.is_module {
                            //Error
                        }
                        let import = self.parse_import_decl()?;
                        let decl = Decl::Import(Box::new(import));
                        Ok(ProgramPart::Decl(decl))
                    }
                }
                &Keyword::Export => {
                    let export = self.parse_export_decl()?;
                    let decl = Decl::Export(Box::new(export));
                    Ok(ProgramPart::Decl(decl))
                }
                &Keyword::Const => {
                    let decl = self.parse_lexical_decl(false)?;
                    Ok(ProgramPart::Decl(decl))
                }
                &Keyword::Function => {
                    let func = self.parse_function_decl(true)?;
                    let decl = Decl::Function(func);
                    Ok(ProgramPart::Decl(decl))
                }
                &Keyword::Class => {
                    let class = self.parse_class_decl(false)?;
                    let decl = Decl::Class(class);
                    Ok(ProgramPart::Decl(decl))
                }
                &Keyword::Let => Ok(if self.at_lexical_decl() {
                    let decl = self.parse_lexical_decl(false)?;
                    ProgramPart::Decl(decl)
                } else {
                    let stmt = self.parse_statement()?;
                    ProgramPart::Stmt(stmt)
                }),
                _ => {
                    let stmt = self.parse_statement()?;
                    Ok(ProgramPart::Stmt(stmt))
                }
            },
            _ => {
                let stmt = self.parse_statement()?;
                Ok(ProgramPart::Stmt(stmt))
            }
        };
        ret
    }
    /// This will cover all possible import statements supported
    /// ```js
    /// import * as Stuff from 'place'; //namespace
    /// import Thing from 'place'; //default
    /// import {Thing} from 'place'; //named
    /// import Person, {Thing} from 'place';// default + named
    /// import Thing, * as Stuff from 'place';
    /// import 'place';
    /// ```
    #[inline]
    fn parse_import_decl(&mut self) -> Res<ModImport<'b>> {
        if self.context.in_function_body {
            //error
        }
        self.expect_keyword(Keyword::Import)?;
        // if the next toke is a string we are at an import
        // with not specifiers
        if self.look_ahead.is_string() {
            let source = self.parse_module_specifier()?;
            self.consume_semicolon()?;
            Ok(ModImport {
                specifiers: vec![],
                source,
            })
        } else {
            // If we are at an open brace, this is the named
            //variant
            let specifiers = if self.at_punct(Punct::OpenBrace) {
                self.parse_named_imports()?
            // If we are at ta *, this is the namespace variant
            } else if self.at_punct(Punct::Asterisk) {
                vec![self.parse_import_namespace_specifier()?]
            // if we are at an identifier that is not `default` this is the default variant
            } else if self.at_possible_ident() && !self.at_keyword(Keyword::Default) {
                let mut specifiers = vec![self.parse_import_default_specifier()?];
                // we we find a comma, this will be more complicated than just 1 item
                if self.at_punct(Punct::Comma) {
                    let _ = self.next_item()?;
                    // if we find a `*`, we need to add the namespace variant to the
                    // specifiers
                    if self.at_punct(Punct::Asterisk) {
                        specifiers.push(self.parse_import_namespace_specifier()?);
                    // if we find an `{` we need to extend the specifiers
                    // with the named variants
                    } else if self.at_punct(Punct::OpenBrace) {
                        specifiers.append(&mut self.parse_named_imports()?);
                    } else {
                        // A comma not followed by `{` or `*` is an error
                        return self.expected_token_error(&self.look_ahead, &["{", "*"]);
                    }
                }
                specifiers
            // import must be followed by an `{`, `*`, `identifier`, or `string`
            } else {
                return self
                    .expected_token_error(&self.look_ahead, &["{", "*", "[ident]", "[string]"]);
            };
            // Import declarations require the contextual keyword
            // `from`
            if !self.at_contextual_keyword("from") {
                return self.expected_token_error(&self.look_ahead, &["from"]);
            }
            let _ = self.next_item()?;
            // capture the source string for where this import
            // comes from
            let source = self.parse_module_specifier()?;
            self.consume_semicolon()?;
            Ok(ModImport { specifiers, source })
        }
    }
    /// This will handle the named variant of imports
    /// ```js
    /// import {Thing} from 'place';
    /// ```
    #[inline]
    fn parse_named_imports(&mut self) -> Res<Vec<ImportSpecifier<'b>>> {
        self.expect_punct(Punct::OpenBrace)?;
        let mut ret = vec![];
        while !self.at_punct(Punct::CloseBrace) {
            ret.push(self.parse_import_specifier()?);
            if !self.at_punct(Punct::CloseBrace) {
                self.expect_punct(Punct::Comma)?;
            }
        }
        self.expect_punct(Punct::CloseBrace)?;
        Ok(ret)
    }

    #[inline]
    fn parse_import_specifier(&mut self) -> Res<ImportSpecifier<'b>> {
        let (imported, local) = if self.look_ahead.token.is_ident() {
            let imported = self.parse_var_ident(false)?;
            let local = if self.at_contextual_keyword("as") {
                let _ = self.next_item();
                Some(self.parse_var_ident(false)?)
            } else {
                None
            };
            (imported, local)
        } else {
            let imported = self.parse_ident_name()?;
            let local = if self.at_contextual_keyword("as") {
                let _ = self.next_item()?;
                Some(self.parse_var_ident(false)?)
            } else {
                None
            };
            (imported, local)
        };
        Ok(ImportSpecifier::Normal(imported, local))
    }

    #[inline]
    fn parse_import_namespace_specifier(&mut self) -> Res<ImportSpecifier<'b>> {
        self.expect_punct(Punct::Asterisk)?;
        if !self.at_contextual_keyword("as") {
            return self.expected_token_error(&self.look_ahead, &["as"]);
        }
        let _ = self.next_item()?;
        let ident = self.parse_ident_name()?;
        Ok(ImportSpecifier::Namespace(ident))
    }

    #[inline]
    fn parse_import_default_specifier(&mut self) -> Res<ImportSpecifier<'b>> {
        let ident = self.parse_ident_name()?;
        Ok(ImportSpecifier::Default(ident))
    }

    #[inline]
    fn parse_export_decl(&mut self) -> Res<ModExport<'b>> {
        if self.context.in_function_body {
            //error
        }
        self.expect_keyword(Keyword::Export)?;
        if self.at_keyword(Keyword::Default) {
            let _ = self.next_item()?;
            let decl = if self.at_keyword(Keyword::Function) {
                let func = Decl::Function(self.parse_function_decl(true)?);
                DefaultExportDecl::Decl(func)
            } else if self.at_keyword(Keyword::Class) {
                let class = Decl::Class(self.parse_class_decl(true)?);
                DefaultExportDecl::Decl(class)
            } else if self.at_contextual_keyword("async") {
                if self.at_async_function() {
                    let func = self.parse_function_decl(true)?;
                    let decl = Decl::Function(func);
                    DefaultExportDecl::Decl(decl)
                } else {
                    let expr = self.parse_assignment_expr()?;
                    DefaultExportDecl::Expr(expr)
                }
            } else {
                if self.at_contextual_keyword("from") {
                    //error
                }
                if self.at_punct(Punct::OpenBrace) {
                    let expr = self.parse_obj_init()?;
                    DefaultExportDecl::Expr(expr)
                } else if self.at_punct(Punct::OpenBracket) {
                    let expr = self.parse_array_init()?;
                    DefaultExportDecl::Expr(expr)
                } else {
                    let expr = self.parse_assignment_expr()?;
                    DefaultExportDecl::Expr(expr)
                }
            };
            Ok(ModExport::Default(decl))
        } else if self.at_punct(Punct::Asterisk) {
            let _ = self.next_item()?;
            if !self.at_contextual_keyword("from") {
                //error
            }
            let _ = self.next_item()?;
            let source = self.parse_module_specifier()?;
            self.consume_semicolon()?;
            Ok(ModExport::All(source))
        } else if self.look_ahead.token.is_keyword() {
            if self.look_ahead.token.matches_keyword(Keyword::Let)
                || self.look_ahead.token.matches_keyword(Keyword::Const)
            {
                let lex = self.parse_lexical_decl(false)?;
                let decl = NamedExportDecl::Decl(lex);
                self.consume_semicolon()?;
                Ok(ModExport::Named(decl))
            } else if self.look_ahead.token.matches_keyword(Keyword::Var) {
                let _ = self.next_item()?;
                let var = Decl::Variable(VariableKind::Var, self.parse_variable_decl_list(false)?);
                let decl = NamedExportDecl::Decl(var);
                self.consume_semicolon()?;
                Ok(ModExport::Named(decl))
            } else if self.look_ahead.token.matches_keyword(Keyword::Class) {
                let class = self.parse_class_decl(true)?;
                let decl = Decl::Class(class);
                let decl = NamedExportDecl::Decl(decl);
                Ok(ModExport::Named(decl))
            } else if self.look_ahead.token.matches_keyword(Keyword::Function) {
                let func = self.parse_function_decl(true)?;
                let decl = Decl::Function(func);
                let decl = NamedExportDecl::Decl(decl);
                Ok(ModExport::Named(decl))
            } else {
                return self.expected_token_error(
                    &self.look_ahead,
                    &["let", "var", "const", "class", "function"],
                );
            }
        } else if self.at_async_function() {
            let func = self.parse_function_decl(false)?;
            let decl = Decl::Function(func);
            let decl = NamedExportDecl::Decl(decl);
            Ok(ModExport::Named(decl))
        } else {
            self.expect_punct(Punct::OpenBrace)?;
            let mut specifiers = vec![];
            let mut found_default = false;
            while !self.at_punct(Punct::CloseBrace) {
                if self.at_keyword(Keyword::Default) {
                    found_default = true;
                }
                specifiers.push(self.parse_export_specifier()?);
                if !self.at_punct(Punct::CloseBrace) {
                    self.expect_punct(Punct::Comma)?;
                }
            }
            let _ = self.next_item()?;
            if self.at_contextual_keyword("from") {
                let _ = self.next_item()?;
                let source = self.parse_module_specifier()?;
                self.consume_semicolon()?;
                let decl = NamedExportDecl::Specifier(specifiers, Some(source));
                Ok(ModExport::Named(decl))
            } else if found_default {
                self.expected_token_error(&self.look_ahead, &[""])
            } else {
                self.consume_semicolon()?;
                let decl = NamedExportDecl::Specifier(specifiers, None);
                Ok(ModExport::Named(decl))
            }
        }
    }

    #[inline]
    fn parse_export_specifier(&mut self) -> Res<ExportSpecifier<'b>> {
        let local = self.parse_ident_name()?;
        let exported = if self.at_contextual_keyword("as") {
            let _ = self.next_item()?;
            Some(self.parse_ident_name()?)
        } else {
            None
        };
        Ok(ExportSpecifier { local, exported })
    }

    #[inline]
    fn parse_module_specifier(&mut self) -> Res<Literal<'b>> {
        let item = self.next_item()?;
        match &item.token {
            Token::String(_) => Ok(Literal::String(self.get_string(&item.span)?)),
            _ => self.expected_token_error(&item, &["[string]"]),
        }
    }

    #[inline]
    fn parse_statement(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_statement, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let lh = self.look_ahead.token.clone();
        let stmt = match lh {
            Token::Boolean(_)
            | Token::Null
            | Token::Number(_)
            | Token::String(_)
            | Token::RegEx(_)
            | Token::Template(_) => {
                let expr = self.parse_expression_statement()?;
                Stmt::Expr(expr)
            }
            Token::Punct(ref p) => match p {
                Punct::OpenBrace => {
                    let b = self.parse_block()?;
                    Stmt::Block(b)
                }
                Punct::OpenParen => {
                    let expr = self.parse_expression_statement()?;
                    Stmt::Expr(expr)
                }
                Punct::SemiColon => {
                    let _ = self.next_item()?;
                    Stmt::Empty
                }
                _ => {
                    let expr = self.parse_expression_statement()?;
                    Stmt::Expr(expr)
                }
            },
            Token::Ident(_) => {
                if self.at_async_function() {
                    let f = self.parse_function_decl(true)?;
                    Stmt::Expr(Expr::Function(f))
                } else {
                    self.parse_labelled_statement()?
                }
            }
            Token::Keyword(ref k) => match k {
                Keyword::Break => Stmt::Break(self.parse_break_stmt()?),
                Keyword::Continue => Stmt::Continue(self.parse_continue_stmt()?),
                Keyword::Debugger => self.parse_debugger_stmt()?,
                Keyword::Do => Stmt::DoWhile(self.parse_do_while_stmt()?),
                Keyword::For => self.parse_for_stmt()?,
                Keyword::Function => Stmt::Expr(self.parse_fn_stmt()?),
                Keyword::If => Stmt::If(self.parse_if_stmt()?),
                Keyword::Return => Stmt::Return(self.parse_return_stmt()?),
                Keyword::Switch => Stmt::Switch(self.parse_switch_stmt()?),
                Keyword::Throw => Stmt::Throw(self.parse_throw_stmt()?),
                Keyword::Try => Stmt::Try(self.parse_try_stmt()?),
                Keyword::Var => self.parse_var_stmt()?,
                Keyword::While => Stmt::While(self.parse_while_stmt()?),
                Keyword::With => Stmt::With(self.parse_with_stmt()?),
                _ => Stmt::Expr(self.parse_expression_statement()?),
            },
            _ => return self.expected_token_error(&self.look_ahead, &[]),
        };
        Ok(stmt)
    }

    #[inline]
    fn parse_with_stmt(&mut self) -> Res<WithStmt<'b>> {
        debug!("{}: parse_with_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.context.strict {
            self.tolerate_error(Error::NonStrictFeatureInStrictContext(
                self.current_position,
                "with statements".to_string(),
            ))?;
        }
        self.expect_keyword(Keyword::With)?;
        self.expect_punct(Punct::OpenParen)?;
        let obj = self.parse_expression()?;
        Ok(if !self.at_punct(Punct::CloseParen) {
            if !self.config.tolerant {
                let _ = self.expected_token_error(&self.look_ahead, &[")"])?;
            }
            WithStmt {
                object: obj,
                body: Box::new(Stmt::Empty),
            }
        } else {
            self.expect_punct(Punct::CloseParen)?;
            WithStmt {
                object: obj,
                body: Box::new(self.parse_statement()?),
            }
        })
    }

    #[inline]
    fn parse_while_stmt(&mut self) -> Res<WhileStmt<'b>> {
        debug!("{}: parse_while_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::While)?;
        self.expect_punct(Punct::OpenParen)?;
        let test = self.parse_expression()?;
        let body = if !self.at_punct(Punct::CloseParen) {
            if !self.config.tolerant {
                return self.expected_token_error(&self.look_ahead, &[")"]);
            }
            Stmt::Empty
        } else {
            self.expect_punct(Punct::CloseParen)?;
            let prev_iter = self.context.in_iteration;
            let body = self.parse_statement()?;
            self.context.in_iteration = prev_iter;
            body
        };
        Ok(WhileStmt {
            test,
            body: Box::new(body),
        })
    }

    #[inline]
    fn parse_var_stmt(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_var_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Var)?;
        let decls = self.parse_var_decl_list(false)?;
        let stmt = Stmt::Var(decls);
        self.consume_semicolon()?;
        Ok(stmt)
    }

    #[inline]
    fn parse_var_decl_list(&mut self, in_for: bool) -> Res<Vec<VariableDecl<'b>>> {
        let mut ret = vec![self.parse_var_decl(in_for)?];
        while self.at_punct(Punct::Comma) {
            let _ = self.next_item()?;
            ret.push(self.parse_var_decl(in_for)?)
        }
        Ok(ret)
    }

    #[inline]
    fn parse_var_decl(&mut self, in_for: bool) -> Res<VariableDecl<'b>> {
        let (_, patt) = self.parse_pattern(Some(VariableKind::Var), &mut vec![])?;
        if self.context.strict && Self::is_restricted(&patt) {
            //error
        }
        let init = if self.at_punct(Punct::Equal) {
            let _ = self.next_item()?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let init = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            Some(init)
        } else if !Self::is_pat_ident(&patt) && !in_for {
            return self.expected_token_error(&self.look_ahead, &["="]);
        } else {
            None
        };
        Ok(VariableDecl { id: patt, init })
    }

    #[inline]
    fn parse_try_stmt(&mut self) -> Res<TryStmt<'b>> {
        debug!("{}: parse_try_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Try)?;
        let block = self.parse_block()?;
        let handler = if self.at_keyword(Keyword::Catch) {
            Some(self.parse_catch_clause()?)
        } else {
            None
        };
        let finalizer = if self.at_keyword(Keyword::Finally) {
            Some(self.parse_finally_clause()?)
        } else {
            None
        };
        if handler.is_none() && finalizer.is_none() {
            //error: one or the other must be declared
        }
        Ok(TryStmt {
            block,
            handler,
            finalizer,
        })
    }

    #[inline]
    fn parse_catch_clause(&mut self) -> Res<CatchClause<'b>> {
        debug!("{}: parse_catch_clause, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Catch)?;
        let param = if self.at_punct(Punct::OpenParen) {
            self.expect_punct(Punct::OpenParen)?;
            if self.at_punct(Punct::CloseParen) {
                //error variable named required
            }
            let mut params = vec![];
            let (_, param) = self.parse_pattern(None, &mut params)?;
            self.expect_punct(Punct::CloseParen)?;
            Some(param)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(CatchClause { param, body })
    }

    #[inline]
    fn parse_finally_clause(&mut self) -> Res<BlockStmt<'b>> {
        debug!("{}: parse_finally_clause, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Finally)?;
        self.parse_block()
    }

    #[inline]
    fn parse_throw_stmt(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_throw_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Throw)?;
        if self.context.has_line_term {
            //error: no new line allowed after throw
        }
        let arg = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(arg)
    }

    #[inline]
    fn parse_switch_stmt(&mut self) -> Res<SwitchStmt<'b>> {
        debug!("{}: parse_switch_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Switch)?;
        self.expect_punct(Punct::OpenParen)?;
        let discriminant = self.parse_expression()?;
        self.expect_punct(Punct::CloseParen)?;
        self.expect_punct(Punct::OpenBrace)?;

        let prev_sw = self.context.in_switch;
        self.context.in_switch = true;
        let mut found_default = false;
        let mut cases = vec![];
        loop {
            if self.at_punct(Punct::CloseBrace) {
                break;
            }
            let case = self.parse_switch_case()?;
            if case.test.is_none() {
                if found_default {
                    return self.expected_token_error(&self.look_ahead, &[]);
                }
                found_default = true;
            }
            cases.push(case);
        }
        self.expect_punct(Punct::CloseBrace)?;
        self.context.in_switch = prev_sw;
        Ok(SwitchStmt {
            discriminant,
            cases,
        })
    }

    #[inline]
    fn parse_switch_case(&mut self) -> Res<SwitchCase<'b>> {
        debug!("{}: parse_switch_case, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let test = if self.at_keyword(Keyword::Default) {
            self.expect_keyword(Keyword::Default)?;
            None
        } else {
            self.expect_keyword(Keyword::Case)?;
            Some(self.parse_expression()?)
        };
        self.expect_punct(Punct::Colon)?;
        let mut consequent = vec![];
        loop {
            if self.at_punct(Punct::CloseBrace)
                || self.at_keyword(Keyword::Default)
                || self.at_keyword(Keyword::Case)
            {
                break;
            }
            consequent.push(self.parse_statement_list_item()?)
        }
        Ok(SwitchCase { test, consequent })
    }

    #[inline]
    fn parse_return_stmt(&mut self) -> Res<Option<Expr<'b>>> {
        debug!("{}: parse_return_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if !self.context.in_function_body {
            return self
                .unexpected_token_error(&self.look_ahead, "cannot return in the global context");
        }
        self.expect_keyword(Keyword::Return)?;
        // if we are at a semi-colon,or close curly brace or eof
        //the return doesn't have an arg. If we are at a line term
        //we need to account for a string literal or template literal
        //since they both can have new lines

        let ret = if self.at_return_arg() {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.consume_semicolon()?;
        Ok(ret)
    }

    #[inline]
    fn parse_if_stmt(&mut self) -> Res<IfStmt<'b>> {
        debug!("{}: parse_if_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::If)?;
        self.expect_punct(Punct::OpenParen)?;
        let test = self.parse_expression()?;
        let (consequent, alternate) = if !self.at_punct(Punct::CloseParen) {
            if !self.config.tolerant {
                return self.expected_token_error(&self.look_ahead, &[")"]);
            }
            (Box::new(Stmt::Empty), None)
        } else {
            self.expect_punct(Punct::CloseParen)?;
            let c = self.parse_if_clause()?;
            let a = if self.at_keyword(Keyword::Else) {
                let _ = self.next_item()?;
                Some(Box::new(self.parse_if_clause()?))
            } else {
                None
            };
            (Box::new(c), a)
        };
        Ok(IfStmt {
            test,
            consequent,
            alternate,
        })
    }

    #[inline]
    fn parse_if_clause(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_if_clause, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.context.strict && self.at_keyword(Keyword::Function) {
            if !self.config.tolerant {
                return self.unexpected_token_error(&self.look_ahead, "");
            }
        }
        self.parse_statement()
    }

    #[inline]
    fn parse_fn_stmt(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_fn_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let decl = self.parse_function_decl(true)?;
        Ok(Expr::Function(decl))
    }

    #[inline]
    fn parse_for_stmt(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_for_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);

        self.expect_keyword(Keyword::For)?;
        let is_await = if self.at_keyword(Keyword::Await) {
            let _ = self.next_item()?;
            true
        // for await ([lookahead ≠ let] LeftHandSideExpression [?Yield, ?Await] of AssignmentExpression [+In, ?Yield, ?Await]) Statement [?Yield, ?Await, ?Return]
        // for await (var ForBinding [?Yield, ?Await] of AssignmentExpression [+In, ?Yield, ?Await]) Statement [?Yield, ?Await, ?Return]
        // for await (ForDeclaration [?Yield, ?Await] of AssignmentExpression [+In, ?Yield, ?Await]) Statement[?Yield, ?Await, ?Return]
        } else {
            false
        };
        self.expect_punct(Punct::OpenParen)?;
        if self.at_punct(Punct::SemiColon) {
            // any semi-colon would mean standard C style for loop
            // for (;;) {}
            let stmt = self.parse_for_loop(VariableKind::Var)?;
            return Ok(Stmt::For(stmt));
        }

        if self.at_keyword(Keyword::Var) {
            let _ = self.next_item()?;
            let kind = VariableKind::Var;
            let prev_in = self.context.allow_in;
            self.context.allow_in = false;
            let mut bindings = self.parse_variable_decl_list(true)?;
            self.context.allow_in = prev_in;
            if bindings.len() == 1 {
                let decl = if let Some(d) = bindings.pop() {
                    d
                } else {
                    return self.expected_token_error(&self.look_ahead, &["variable decl"]);
                };
                if self.at_keyword(Keyword::In) {
                    let left = LoopLeft::Variable(kind, decl);
                    let stmt = self.parse_for_in_loop(left)?;
                    return Ok(Stmt::ForIn(stmt));
                } else if self.at_contextual_keyword("of") {
                    let left = LoopLeft::Variable(kind, decl);
                    let stmt = self.parse_for_of_loop(left, is_await)?;
                    return Ok(Stmt::ForOf(stmt));
                } else {
                    let init = LoopInit::Variable(kind, vec![decl]);
                    let stmt = self.parse_for_loop_cont(Some(init))?;
                    return Ok(Stmt::For(stmt));
                }
            } else {
                let init = LoopInit::Variable(kind, bindings);
                let stmt = self.parse_for_loop_cont(Some(init))?;
                return Ok(Stmt::For(stmt));
            }
        } else if self.at_keyword(Keyword::Const) || self.at_keyword(Keyword::Let) {
            let kind = self.next_item()?;
            let kind = match &kind.token {
                Token::Keyword(ref k) => match k {
                    Keyword::Const => VariableKind::Const,
                    Keyword::Let => VariableKind::Let,
                    _ => unreachable!(),
                },
                _ => return self.expected_token_error(&kind, &["const", "let"]),
            };
            if !self.context.strict && self.look_ahead.token.matches_keyword(Keyword::In) {
                let _in = self.next_item()?;
                //const or let becomes an ident
                let k = match kind {
                    VariableKind::Var => "var",
                    VariableKind::Let => "let",
                    VariableKind::Const => "const",
                };
                let left = LoopLeft::Expr(Expr::Ident(k));
                let right = self.parse_expression()?;
                Ok(Stmt::ForIn(ForInStmt {
                    left,
                    right,
                    body: Box::new(self.parse_loop_body()?),
                }))
            } else {
                let prev_in = self.context.allow_in;
                self.context.allow_in = false;
                let mut decls = self.parse_binding_list(kind, true)?;
                self.context.allow_in = prev_in;
                if decls.len() == 1 {
                    let decl = if let Some(d) = decls.pop() {
                        d
                    } else {
                        return self.expected_token_error(&self.look_ahead, &["variable decl"]);
                    };
                    if decl.init.is_none() && self.at_keyword(Keyword::In) {
                        let left = LoopLeft::Variable(kind, decl);
                        let _in = self.next_item()?;
                        let right = self.parse_expression()?;
                        return Ok(Stmt::ForIn(ForInStmt {
                            left,
                            right,
                            body: Box::new(self.parse_loop_body()?),
                        }));
                    } else if decl.init.is_none() && self.at_contextual_keyword("of") {
                        let left = LoopLeft::Variable(kind, decl);
                        return Ok(Stmt::ForOf(self.parse_for_of_loop(left, is_await)?));
                    } else {
                        let init = LoopInit::Variable(kind, vec![decl]);
                        let stmt = self.parse_for_loop_cont(Some(init))?;
                        return Ok(Stmt::For(stmt));
                    }
                } else {
                    let init = LoopInit::Variable(kind, decls);
                    let stmt = self.parse_for_loop_cont(Some(init))?;
                    return Ok(Stmt::For(stmt));
                }
            }
        } else {
            let prev_in = self.context.allow_in;
            self.context.allow_in = false;
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let init = self.parse_assignment_expr()?;
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            self.context.allow_in = prev_in;
            if self.at_keyword(Keyword::In) {
                let _ = self.next_item()?;
                let left = LoopLeft::Expr(init);
                let right = self.parse_expression()?;
                return Ok(Stmt::ForIn(ForInStmt {
                    left,
                    right,
                    body: Box::new(self.parse_loop_body()?),
                }));
            } else if self.at_contextual_keyword("of") {
                let _ = self.next_item()?;
                let left = LoopLeft::Expr(init);
                let right = self.parse_assignment_expr()?;
                let body = self.parse_loop_body()?;
                return Ok(Stmt::ForOf(ForOfStmt {
                    left,
                    right,
                    body: Box::new(body),
                    is_await,
                }));
            } else {
                let init = if self.at_punct(Punct::Comma) {
                    let mut seq = vec![init];
                    while self.at_punct(Punct::Comma) {
                        let _comma = self.next_item()?;
                        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
                        seq.push(self.parse_assignment_expr()?);
                        self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
                    }
                    LoopInit::Expr(Expr::Sequence(seq))
                } else {
                    LoopInit::Expr(init)
                };
                return Ok(Stmt::For(self.parse_for_loop_cont(Some(init))?));
            }
        }
    }

    #[inline]
    fn parse_for_loop(&mut self, kind: VariableKind) -> Res<ForStmt<'b>> {
        debug!("{}: parse_for_loop, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let init = if self.at_punct(Punct::SemiColon) {
            None
        } else {
            let list = self.parse_variable_decl_list(true)?;
            Some(LoopInit::Variable(kind, list))
        };
        self.parse_for_loop_cont(init)
    }

    #[inline]
    fn parse_for_loop_cont(&mut self, init: Option<LoopInit<'b>>) -> Res<ForStmt<'b>> {
        debug!("{}: parse_for_loop_cont, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::SemiColon)?;
        let test = if self.at_punct(Punct::SemiColon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        let _d = format!("{:?}", test);
        self.expect_punct(Punct::SemiColon)?;
        let update = if self.at_punct(Punct::CloseParen) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        let body = self.parse_loop_body()?;
        Ok(ForStmt {
            init,
            test,
            update,
            body: Box::new(body),
        })
    }

    #[inline]
    fn parse_for_in_loop(&mut self, left: LoopLeft<'b>) -> Res<ForInStmt<'b>> {
        debug!("{}: parse_for_in_loop, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let _ = self.next_item()?;
        let right = self.parse_expression()?;
        let body = self.parse_loop_body()?;
        Ok(ForInStmt {
            left,
            right,
            body: Box::new(body),
        })
    }

    #[inline]
    fn parse_for_of_loop(&mut self, left: LoopLeft<'b>, is_await: bool) -> Res<ForOfStmt<'b>> {
        debug!("{}: parse_for_of_loop, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let _ = self.next_item()?;
        let right = self.parse_assignment_expr()?;
        let body = self.parse_loop_body()?;
        Ok(ForOfStmt {
            left,
            right,
            body: Box::new(body),
            is_await,
        })
    }

    #[inline]
    fn parse_loop_body(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_loop_body, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::CloseParen)?;
        let prev_iter = self.context.in_iteration;
        self.context.in_iteration = true;
        let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
        let ret = self.parse_statement()?;
        self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
        self.context.in_iteration = prev_iter;
        Ok(ret)
    }

    #[inline]
    fn parse_do_while_stmt(&mut self) -> Res<DoWhileStmt<'b>> {
        debug!("{}: parse_do_while_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Do)?;
        let prev_iter = self.context.in_iteration;
        self.context.in_iteration = true;
        let body = self.parse_statement()?;
        self.context.in_iteration = prev_iter;
        self.expect_keyword(Keyword::While)?;
        self.expect_punct(Punct::OpenParen)?;
        let test = self.parse_expression()?;
        self.expect_punct(Punct::CloseParen)?;
        if self.look_ahead.token.matches_punct(Punct::SemiColon) {
            self.expect_punct(Punct::SemiColon)?;
        }
        Ok(DoWhileStmt {
            test,
            body: Box::new(body),
        })
    }

    #[inline]
    fn parse_break_stmt(&mut self) -> Res<Option<&'b str>> {
        debug!("{}: parse_break_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.parse_optionally_labeled_statement(Keyword::Break)
    }

    #[inline]
    fn parse_continue_stmt(&mut self) -> Res<Option<&'b str>> {
        debug!("{}: parse_continue_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.parse_optionally_labeled_statement(Keyword::Continue)
    }

    #[inline]
    fn parse_optionally_labeled_statement(&mut self, k: Keyword) -> Res<Option<&'b str>> {
        debug!(
            "{}: parse_optionally_labeled_statement",
            self.look_ahead.span.start
        );
        self.expect_keyword(k)?;
        let ret = if self.look_ahead.token.is_ident() && !self.context.has_line_term {
            let id = self.parse_var_ident(false)?;
            if !self.context.label_set.contains(id) {
                //error: unknown label
            }
            Some(id)
        } else {
            None
        };
        self.consume_semicolon()?;
        if ret.is_some() && !self.context.in_iteration && !self.context.in_switch {
            //error: invalid break
        }
        Ok(ret)
    }

    #[inline]
    fn parse_debugger_stmt(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_debugger_stmt, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Debugger)?;
        self.consume_semicolon()?;
        Ok(Stmt::Debugger)
    }

    #[inline]
    fn parse_labelled_statement(&mut self) -> Res<Stmt<'b>> {
        debug!("{}: parse_labelled_statement, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let ret = self.parse_expression()?;
        if Self::is_ident(&ret) && self.at_punct(Punct::Colon) {
            let _colon = self.next_item()?;
            let id = if let Expr::Ident(ref ident) = ret {
                ident.clone()
            } else {
                return Err(self.reinterpret_error("expression", "ident"));
            };
            if !self.context.label_set.insert(format!("${}", &id)) {
                return Err(self.redecl_error(&id));
            }
            let body = if self.at_keyword(Keyword::Class) {
                let class = self.next_item()?;
                if !self.config.tolerant {
                    return self.unexpected_token_error(&class, "");
                }
                let body = self.parse_class_body()?;
                let cls = Class {
                    id: None,
                    super_class: None,
                    body,
                };
                let expr = Expr::Class(cls);
                Stmt::Expr(expr)
            } else if self.at_keyword(Keyword::Function) {
                let f = self.parse_function_decl(true)?;
                let expr = Expr::Function(f);
                Stmt::Expr(expr)
            } else {
                self.parse_statement()?
            };
            self.context.label_set.remove(&format!("${}", &id));
            Ok(Stmt::Labeled(LabeledStmt {
                label: id,
                body: Box::new(body),
            }))
        } else {
            self.consume_semicolon()?;
            Ok(Stmt::Expr(ret))
        }
    }

    #[inline]
    fn parse_expression_statement(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_expression_statement, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let ret = self.parse_expression()?;
        self.consume_semicolon()?;
        Ok(ret)
    }

    #[inline]
    fn parse_expression<'a>(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_expression, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
        let ret = self.parse_assignment_expr()?;
        self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
        if self.at_punct(Punct::Comma) {
            let mut list = vec![ret];
            while !self.look_ahead.token.is_eof() {
                if !self.at_punct(Punct::Comma) {
                    break;
                }
                let _comma = self.next_item()?;
                let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                let expr = self.parse_assignment_expr()?;
                self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                list.push(expr);
            }
            return Ok(Expr::Sequence(list));
        }
        Ok(ret)
    }

    #[inline]
    fn parse_block(&mut self) -> Res<BlockStmt<'b>> {
        debug!("{}: parse_block, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenBrace)?;
        let mut ret = vec![];
        loop {
            if self.at_punct(Punct::CloseBrace) {
                break;
            }
            let part = self.parse_statement_list_item()?;
            // if part.is_export() {
            //     //error
            // }
            // if part.is_import() {
            //     //error
            // }
            ret.push(part);
        }
        self.expect_punct(Punct::CloseBrace)?;
        Ok(ret)
    }

    #[inline]
    fn parse_lexical_decl(&mut self, in_for: bool) -> Res<Decl<'b>> {
        debug!("{}: parse_lexical_decl, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let next = self.next_item()?;
        let kind = match &next.token {
            &Token::Keyword(ref k) => match k {
                Keyword::Let => VariableKind::Let,
                Keyword::Const => VariableKind::Const,
                _ => return self.expected_token_error(&next, &["let", "const"]),
            },
            _ => return self.expected_token_error(&next, &["let", "const"]),
        };
        let decl = self.parse_binding_list(kind, in_for)?;
        self.consume_semicolon()?;
        Ok(Decl::Variable(kind, decl))
    }

    #[inline]
    fn parse_binding_list(
        &mut self,
        kind: VariableKind,
        in_for: bool,
    ) -> Res<Vec<VariableDecl<'b>>> {
        debug!("{}: parse_binding_list, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut ret = vec![self.parse_lexical_binding(kind, in_for)?];
        while self.at_punct(Punct::Comma) {
            let _comma = self.next_item()?;
            ret.push(self.parse_lexical_binding(kind, in_for)?)
        }
        Ok(ret)
    }

    #[inline]
    fn parse_variable_decl_list(&mut self, in_for: bool) -> Res<Vec<VariableDecl<'b>>> {
        let mut ret = vec![self.parse_variable_decl(in_for)?];
        while self.at_punct(Punct::Comma) {
            let _ = self.next_item()?;
            ret.push(self.parse_variable_decl(in_for)?);
        }
        Ok(ret)
    }

    #[inline]
    fn parse_variable_decl(&mut self, in_for: bool) -> Res<VariableDecl<'b>> {
        let start = self.look_ahead.clone();
        let (_, id) = self.parse_pattern(Some(VariableKind::Var), &mut vec![])?;
        if self.context.strict && Self::is_restricted(&id) {
            if !self.config.tolerant {
                return self.unexpected_token_error(&start, "restricted word");
            }
        }
        let init = if self.at_punct(Punct::Equal) {
            let _ = self.next_item()?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let init = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            Some(init)
        } else if !Self::is_pat_ident(&id) && !in_for {
            self.expect_punct(Punct::Equal)?;
            None
        } else {
            None
        };
        Ok(VariableDecl { id, init })
    }

    #[inline]
    fn is_pat_ident(pat: &Pat) -> bool {
        match pat {
            Pat::Identifier(_) => true,
            _ => false,
        }
    }

    #[inline]
    fn parse_lexical_binding(&mut self, kind: VariableKind, in_for: bool) -> Res<VariableDecl<'b>> {
        debug!("{}: parse_lexical_binding, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let start = self.look_ahead.clone();
        let (_, id) = self.parse_pattern(Some(kind), &mut vec![])?;
        if self.context.strict && Self::is_restricted(&id) {
            if !self.config.tolerant {
                return self.unexpected_token_error(&start, "restricted word");
            }
        }
        let init = if kind == VariableKind::Const {
            if !self.at_keyword(Keyword::In) && !self.at_contextual_keyword("of") {
                if self.at_punct(Punct::Equal) {
                    let _ = self.next_item()?;
                    let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                    let init = self.parse_assignment_expr()?;
                    self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                    Some(init)
                } else {
                    return self.expected_token_error(&self.look_ahead, &["="]);
                }
            } else {
                None
            }
        } else if !in_for && !Self::is_pat_ident(&id) || self.at_punct(Punct::Equal) {
            self.expect_punct(Punct::Equal)?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let init = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            Some(init)
        } else {
            None
        };
        Ok(VariableDecl { id, init })
    }

    #[inline]
    fn is_restricted(id: &Pat) -> bool {
        match id {
            Pat::Identifier(ref ident) => ident == &"eval" || ident == &"arguments",
            _ => false,
        }
    }

    #[inline]
    fn parse_function_decl(&mut self, opt_ident: bool) -> Res<Function<'b>> {
        debug!("{}: parse_function_decl, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let is_async = if self.at_contextual_keyword("async") {
            let _ = self.next_item()?;
            true
        } else {
            false
        };
        self.expect_keyword(Keyword::Function)?;
        let is_gen = if is_async {
            false
        } else {
            let is_gen = self.at_punct(Punct::Asterisk);
            if is_gen {
                let _ = self.next_item()?;
            }
            is_gen
        };
        let (id, first_restricted) = if !opt_ident || !self.at_punct(Punct::OpenParen) {
            let start = self.look_ahead.clone();
            let id = self.parse_var_ident(false)?;
            if self.context.strict && start.token.is_restricted() {
                return self.expected_token_error(&start, &[]);
            }
            let first_restricted = if !self.context.strict {
                if start.token.is_restricted() {
                    Some(start)
                } else if start.token.is_strict_reserved() {
                    Some(start)
                } else {
                    None
                }
            } else {
                None
            };
            (Some(id), first_restricted)
        } else {
            (None, None)
        };
        let prev_await = self.context.r#await;
        let prev_yield = self.context.allow_yield;
        self.context.r#await = is_async;
        self.context.allow_yield = !is_gen;

        let formal_params = self.parse_formal_params()?;
        let strict = formal_params.strict;
        let params = formal_params.params;
        let prev_strict = self.context.strict;
        let prev_allow_strict = self.context.allow_strict_directive;
        self.context.allow_strict_directive = formal_params.simple;
        let body = self.parse_function_source_el()?;
        if self.context.strict {
            if let Some(ref item) = first_restricted {
                return self.expected_token_error(item, &[]);
            }
        }
        if self.context.strict && strict {
            return self.expected_token_error(&self.look_ahead, &[]);
        }
        self.context.strict = prev_strict;
        self.context.allow_strict_directive = prev_allow_strict;
        self.context.r#await = prev_await;
        self.context.allow_yield = prev_yield;
        Ok(Function {
            id,
            params,
            body,
            generator: is_gen,
            is_async,
        })
    }

    #[inline]
    fn parse_function_source_el(&mut self) -> Res<FunctionBody<'b>> {
        debug!("{}: parse_function_source_el, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenBrace)?;
        let mut body = self.parse_directive_prologues()?;
        let prev_label = self.context.label_set.clone();
        let prev_iter = self.context.in_iteration;
        let prev_switch = self.context.in_switch;
        let prev_in_fn = self.context.in_function_body;
        self.context.label_set = HashSet::new();
        self.context.in_iteration = false;
        self.context.in_switch = false;
        self.context.in_function_body = true;
        while !self.look_ahead.token.is_eof() {
            if self.at_punct(Punct::CloseBrace) {
                break;
            }
            body.push(self.parse_statement_list_item()?)
        }
        self.expect_punct(Punct::CloseBrace)?;
        self.context.label_set = prev_label;
        self.context.in_iteration = prev_iter;
        self.context.in_switch = prev_switch;
        self.context.in_function_body = prev_in_fn;
        Ok(body)
    }

    #[inline]
    fn parse_class_decl(&mut self, opt_ident: bool) -> Res<Class<'b>> {
        debug!("{}: parse_class_decl, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let prev_strict = self.context.strict;
        self.context.strict = true;
        self.expect_keyword(Keyword::Class)?;
        let mut super_class = if self.at_contextual_keyword("extends") {
            let _ = self.next_item()?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let super_class = self.parse_left_hand_side_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            Some(Box::new(super_class))
        } else {
            None
        };
        let id = if opt_ident && !self.look_ahead.token.is_ident() {
            None
        } else {
            Some(self.parse_var_ident(false)?)
        };
        super_class = if super_class.is_none() && self.at_contextual_keyword("extends") {
            let _ = self.next_item()?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let super_class = self.parse_left_hand_side_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            Some(Box::new(super_class))
        } else {
            None
        };
        let body = self.parse_class_body()?;

        self.context.strict = prev_strict;
        Ok(Class {
            id,
            super_class,
            body,
        })
    }

    #[inline]
    fn parse_class_body(&mut self) -> Res<Vec<Property<'b>>> {
        debug!("{}: parse_class_body, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut ret = vec![];
        let mut has_ctor = false;
        self.expect_punct(Punct::OpenBrace)?;
        while !self.at_punct(Punct::CloseBrace) {
            if self.at_punct(Punct::SemiColon) {
                let _ = self.next_item()?;
            } else {
                let (ctor, el) = self.parse_class_el(has_ctor)?;
                has_ctor = ctor;
                ret.push(el)
            }
        }
        self.expect_punct(Punct::CloseBrace)?;
        Ok(ret)
    }

    #[inline]
    fn parse_class_el(&mut self, has_ctor: bool) -> Res<(bool, Property<'b>)> {
        debug!("{}: parse_class_el, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut token = self.look_ahead.token.clone();
        let mut has_ctor = has_ctor;
        let mut key: Option<PropertyKey> = None;
        let mut value: Option<PropertyValue> = None;
        let mut computed = false;
        let mut is_static = false;
        let mut is_async = false;
        if self.at_contextual_keyword("async") {
            let _async = self.next_item()?;
            is_async = true;
        }
        if self.at_punct(Punct::Asterisk) {
            let _ = self.next_item()?;
        } else {
            computed = self.at_punct(Punct::OpenBracket);

            let new_key = self.parse_object_property_key()?;

            if Self::is_static(&new_key)
                && (Self::qualified_prop_name(&self.look_ahead.token)
                    || self.at_punct(Punct::Asterisk))
            {
                token = self.look_ahead.token.clone();
                computed = self.at_punct(Punct::OpenBracket);
                is_static = true;
                if self.at_punct(Punct::Asterisk) {
                    let _ = self.next_item()?;
                } else {
                    key = Some(self.parse_object_property_key()?);
                }
            } else {
                key = Some(new_key);
            }
            if token.is_ident()
                && !self.context.has_line_term
                && self.at_contextual_keyword("async")
            {
                if !self.look_ahead.token.matches_punct(Punct::Colon)
                    && !self.look_ahead.token.matches_punct(Punct::OpenParen)
                    && !self.look_ahead.token.matches_punct(Punct::Asterisk)
                {
                    return self.expected_token_error(&self.look_ahead, &[":", "(", "*"]);
                }
            }
        }

        let mut kind: Option<PropertyKind> = None;
        let mut method = false;

        let look_ahead_prop_key = Self::qualified_prop_name(&self.look_ahead.token);
        if token.is_ident() {
            let (at_get, at_set) = if let Some(ref k) = key {
                (
                    Self::is_key(&k, "get") && look_ahead_prop_key,
                    Self::is_key(&k, "set") && look_ahead_prop_key,
                )
            } else {
                (false, false)
            };

            if at_get {
                kind = Some(PropertyKind::Get);
                computed = self.at_punct(Punct::OpenBracket);
                self.context.allow_yield = false;
                key = Some(self.parse_object_property_key()?);
                value = Some(self.parse_getter_method()?);
            } else if at_set {
                kind = Some(PropertyKind::Set);
                computed = self.at_punct(Punct::OpenBracket);
                key = Some(self.parse_object_property_key()?);
                value = Some(self.parse_setter_method()?);
            }
        } else if token.matches_punct(Punct::Asterisk) && look_ahead_prop_key {
            kind = Some(PropertyKind::Init);
            computed = self.at_punct(Punct::OpenBracket);
            key = Some(self.parse_object_property_key()?);
            value = Some(self.parse_generator_method()?);
            method = true;
        }

        if kind.is_none() && key.is_some() && self.at_punct(Punct::OpenParen) {
            kind = Some(PropertyKind::Init);
            method = true;
            value = Some(if is_async {
                self.parse_async_property_method()?
            } else {
                self.parse_property_method()?
            });
        }

        let mut kind = if let Some(k) = kind {
            k
        } else {
            return self.expected_token_error(&self.look_ahead, &["method identifier"]);
        };

        if kind == PropertyKind::Init {
            kind = PropertyKind::Method;
        }

        let key = if let Some(k) = key {
            k
        } else {
            return self.expected_token_error(&self.look_ahead, &[]);
        };
        if !computed {
            if is_static && Self::is_key(&key, "prototype") {
                return self.expected_token_error(&self.look_ahead, &[]);
            }
            if !is_static && Self::is_key(&key, "constructor") {
                if kind != PropertyKind::Method || !method {
                    return self
                        .expected_token_error(&self.look_ahead, &["[constructor declaration]"]);
                }
                if let Some(ref v) = value {
                    if Self::is_generator(&v) {
                        return self.expected_token_error(
                            &self.look_ahead,
                            &["[non-generator function declaration]"],
                        );
                    }
                }
                if has_ctor {
                    return self.expected_token_error(&self.look_ahead, &[]);
                } else {
                    has_ctor = true;
                }
                kind = PropertyKind::Ctor;
            }
        }

        let value = if let Some(v) = value {
            v
        } else {
            return self.expected_token_error(&self.look_ahead, &[]);
        };

        Ok((
            has_ctor,
            Property {
                key,
                value,
                kind,
                method,
                computed,
                short_hand: false,
            },
        ))
    }

    #[inline]
    fn is_key(key: &PropertyKey, other: &str) -> bool {
        match key {
            PropertyKey::Literal(ref l) => match l {
                Literal::String(ref s) => s == &other,
                _ => false,
            },
            PropertyKey::Expr(ref e) => match e {
                Expr::Ident(ref s) => s == &other,
                _ => false,
            },
            PropertyKey::Pat(ref p) => match p {
                Pat::Identifier(ref s) => s == &other,
                _ => false,
            },
        }
    }

    #[inline]
    fn is_generator(val: &PropertyValue) -> bool {
        match val {
            PropertyValue::Expr(ref e) => match e {
                Expr::Function(ref f) => f.generator,
                Expr::ArrowFunction(ref f) => f.generator,
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn is_static(key: &PropertyKey) -> bool {
        match key {
            PropertyKey::Literal(ref l) => match l {
                Literal::String(ref s) => s == &"static",
                _ => false,
            },
            PropertyKey::Expr(ref e) => match e {
                Expr::Ident(ref s) => s == &"static",
                _ => false,
            },
            PropertyKey::Pat(ref p) => match p {
                Pat::Identifier(ref s) => s == &"static",
                _ => false,
            },
        }
    }

    #[inline]
    fn parse_async_property_method(&mut self) -> Res<PropertyValue<'b>> {
        debug!(
            "{}: parse_property_method_async_fn, {:?}",
            self.look_ahead.span.start,
            self.look_ahead.token
        );
        let prev_yield = self.context.allow_yield;
        let prev_await = self.context.r#await;
        self.context.allow_yield = false;
        self.context.r#await = true;
        let params = self.parse_formal_params()?;
        let body = self.parse_property_method_body(params.simple, params.found_restricted)?;
        self.context.allow_yield = prev_yield;
        self.context.r#await = prev_await;
        let func = Function {
            id: None,
            params: params.params,
            is_async: true,
            generator: false,
            body,
        };
        Ok(PropertyValue::Expr(Expr::Function(func)))
    }

    #[inline]
    fn parse_property_method(&mut self) -> Res<PropertyValue<'b>> {
        debug!("{}: parse_property_method, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let prev_yield = self.context.allow_yield;
        self.context.allow_yield = false;
        let params = self.parse_formal_params()?;
        let body = self.parse_property_method_body(params.simple, params.found_restricted)?;
        self.context.allow_yield = prev_yield;
        let func = Function {
            id: None,
            params: params.params,
            is_async: false,
            generator: false,
            body,
        };
        Ok(PropertyValue::Expr(Expr::Function(func)))
    }

    #[inline]
    fn parse_generator_method(&mut self) -> Res<PropertyValue<'b>> {
        debug!("{}: pares_generator_method, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let prev_yield = self.context.allow_yield;
        self.context.allow_yield = true;
        let params = self.parse_formal_params()?;
        self.context.allow_yield = false;
        let body = self.parse_method_body(params.simple, params.found_restricted)?;
        self.context.allow_yield = prev_yield;
        let func = Function {
            id: None,
            params: params.params,
            is_async: false,
            generator: true,
            body,
        };
        Ok(PropertyValue::Expr(Expr::Function(func)))
    }

    #[inline]
    fn parse_getter_method(&mut self) -> Res<PropertyValue<'b>> {
        debug!("{}: parse_getter_method, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let is_gen = false;
        let prev_yield = self.context.allow_yield;
        let start_position = self.look_ahead_position;
        let formal_params = self.parse_formal_params()?;
        if formal_params.params.len() > 0 {
            self.tolerate_error(Error::InvalidGetterParams(start_position))?;
        }
        let body = self.parse_method_body(formal_params.simple, formal_params.found_restricted)?;
        self.context.allow_yield = prev_yield;
        Ok(PropertyValue::Expr(Expr::Function(Function {
            id: None,
            params: formal_params.params,
            body,
            generator: is_gen,
            is_async: false,
        })))
    }

    #[inline]
    fn parse_method_body(
        &mut self,
        simple: bool,
        found_restricted: bool,
    ) -> Res<Vec<ProgramPart<'b>>> {
        debug!("{}: parse_method_body, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.context.is_assignment_target = false;
        self.context.is_binding_element = false;
        let prev_strict = self.context.strict;
        let prev_allow_strict = self.context.allow_strict_directive;
        self.context.allow_strict_directive = simple;
        let start = self.look_ahead.clone();
        let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
        let body = self.parse_function_source_el()?;
        self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
        if self.context.strict && found_restricted && !self.config.tolerant {
            self.unexpected_token_error(&start, "restricted ident")?;
        }
        self.context.strict = prev_strict;
        self.context.allow_strict_directive = prev_allow_strict;
        Ok(body)
    }

    #[inline]
    fn parse_setter_method(&mut self) -> Res<PropertyValue<'b>> {
        debug!("{}: parse_setter_method, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let prev_allow = self.context.allow_yield;
        self.context.allow_yield = true;
        let start_position = self.look_ahead_position;
        let params = self.parse_formal_params()?;
        self.context.allow_yield = prev_allow;
        if params.params.len() != 1 {
            self.tolerate_error(Error::InvalidSetterParams(start_position))?;
        } else if let Some(ref param) = params.params.get(0) {
            if Self::is_rest(param) {
                self.tolerate_error(Error::InvalidSetterParams(start_position))?;
            }
        }
        let body = self.parse_property_method_body(params.simple, params.found_restricted)?;
        let func = Function {
            id: None,
            params: params.params,
            body,
            generator: false,
            is_async: false,
        };
        Ok(PropertyValue::Expr(Expr::Function(func)))
    }

    #[inline]
    fn is_rest(arg: &FunctionArg) -> bool {
        match arg {
            FunctionArg::Expr(ref e) => match e {
                Expr::Spread(_) => true,
                _ => false,
            },
            FunctionArg::Pat(ref p) => match p {
                Pat::RestElement(_) => true,
                _ => false,
            },
        }
    }

    #[inline]
    fn parse_property_method_body(
        &mut self,
        simple: bool,
        found_restricted: bool,
    ) -> Res<FunctionBody<'b>> {
        debug!("{}: parse_property_method_fn, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.context.is_assignment_target = false;
        self.context.is_binding_element = false;
        let prev_strict = self.context.strict;
        let prev_allow = self.context.allow_strict_directive;
        self.context.allow_strict_directive = simple;
        let start_pos = self.look_ahead_position;
        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
        let ret = self.parse_function_source_el()?;
        self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
        if self.context.strict && found_restricted {
            self.tolerate_error(Error::NonStrictFeatureInStrictContext(
                start_pos,
                "restriced ident".to_string(),
            ))?;
        }
        self.context.strict = prev_strict;
        self.context.allow_strict_directive = prev_allow;
        Ok(ret)
    }

    fn qualified_prop_name(tok: &Token) -> bool {
        tok.is_ident()
            || tok.is_keyword()
            || tok.is_literal()
            || tok.matches_punct(Punct::OpenBracket)
    }

    #[inline]
    fn parse_object_property_key(&mut self) -> Res<PropertyKey<'b>> {
        debug!("{}: parse_object_property_key, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let item = self.next_item()?;
        if item.token.is_string()
            || match item.token {
                Token::Number(_) => true,
                _ => false,
            }
        {
            // if item.token.is_oct_literal() {
            //     //FIXME: possible tolerable error
            // }
            let id = match &item.token {
                Token::String(_) => Literal::String(self.get_string(&item.span)?),
                Token::Number(_) => {
                    if self.at_big_int_flag() {
                        return self.unexpected_token_error(
                            &self.look_ahead,
                            "BigInt cannot be uses as an object literal key",
                        );
                    }
                    Literal::Number(self.get_string(&item.span)?)
                }
                _ => return Err(self.reinterpret_error("number or string", "literal")),
            };
            Ok(PropertyKey::Literal(id))
        } else if item.token.is_ident()
            // || item.token.is_bool()
            || item.token.is_null()
            || item.token.is_keyword()
            || match item.token {
                Token::Boolean(_) => true,
                _ => false,
            }
        {
            let id = self.get_string(&item.span)?;
            Ok(PropertyKey::Expr(Expr::Ident(id)))
        } else if item.token.matches_punct(Punct::OpenBracket) {
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let key = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            let id = if Self::is_valid_property_key_literal(&key) {
                match key {
                    Expr::Literal(lit) => PropertyKey::Literal(lit),
                    _ => {
                        return self
                            .expected_token_error(&self.look_ahead, &["property key literal"]);
                    }
                }
            } else {
                PropertyKey::Expr(key)
            };
            self.expect_punct(Punct::CloseBracket)?;
            Ok(id)
        } else {
            self.expected_token_error(
                &item,
                &[
                    "[string]",
                    "[number]",
                    "[ident]",
                    "[boolean]",
                    "null",
                    "[keyword]",
                    "[",
                ],
            )
        }
    }

    #[inline]
    fn is_valid_property_key_literal(expr: &Expr) -> bool {
        match expr {
            Expr::Literal(ref l) => match l {
                Literal::String(_) | Literal::Number(_) | Literal::Boolean(_) => true,
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn parse_primary_expression(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_primary_expression, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.look_ahead.token.is_ident() {
            if (self.context.is_module || self.context.r#await) && self.at_keyword(Keyword::Await) {
                if !self.config.tolerant {
                    return self.unexpected_token_error(
                        &self.look_ahead,
                        "Modules do not allow 'await' to be used as an identifier",
                    );
                }
            }
            if self.at_async_function() {
                self.parse_function_expr()
            } else {
                let ident = self.next_item()?;
                Ok(Expr::Ident(self.get_string(&ident.span)?))
            }
        } else if self.look_ahead.token.is_number() || self.look_ahead.token.is_string() {
            // if self.context.strict && self.look_ahead.token.is_oct_literal() {
            //     //FIXME: possible tolerable error
            // }
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let item = self.next_item()?;
            let lit = match item.token {
                Token::Number(_) => {
                    let mut span = item.span;
                    if self.at_big_int_flag() {
                        span.end += 1;
                        // Consume the ident
                        let _n = self.next_item();
                    }
                    Literal::Number(self.scanner.str_for(&span).unwrap_or(""))
                }
                Token::String(_) => Literal::String(self.scanner.str_for(&item.span).unwrap_or("")),
                _ => unreachable!(),
            };
            Ok(Expr::Literal(lit))
        } else if self.look_ahead.token.is_boolean() {
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let item = self.next_item()?;
            let lit = match item.token {
                Token::Boolean(b) => Literal::Boolean(b.into()),
                _ => unreachable!(),
            };
            Ok(Expr::Literal(lit))
        } else if self.look_ahead.token.is_null() {
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let _ = self.next_item()?;
            Ok(Expr::Literal(Literal::Null))
        } else if self.look_ahead.is_template() {
            let lit = self.parse_template_literal()?;
            Ok(Expr::Literal(Literal::Template(lit)))
        } else if self.look_ahead.token.is_punct() {
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let expr = if self.at_punct(Punct::OpenParen) {
                self.parse_group_expr()?
            } else if self.at_punct(Punct::OpenBracket) {
                self.parse_array_init()?
            } else if self.at_punct(Punct::OpenBrace) {
                self.parse_obj_init()?
            } else {
                return self.expected_token_error(&self.look_ahead, &["{", "[", "("]);
            };
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            Ok(expr)
        } else if self.look_ahead.token.is_regex() {
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let regex = self.next_item()?;
            let lit = match regex.token {
                Token::RegEx(r) => {
                    let flags = if let Some(f) = r.flags { f } else { "" };
                    RegEx {
                        pattern: r.body,
                        flags,
                    }
                }
                _ => unreachable!(),
            };
            Ok(Expr::Literal(Literal::RegEx(lit)))
        } else if self.look_ahead.token.is_keyword() {
            if !self.context.strict
                && ((self.context.allow_yield && self.at_keyword(Keyword::Yield))
                    || self.at_keyword(Keyword::Let))
            {
                let ident = self.parse_ident_name()?;
                Ok(Expr::Ident(ident))
            } else if !self.context.strict && self.look_ahead.token.is_strict_reserved() {
                let ident = self.parse_ident_name()?;
                Ok(Expr::Ident(ident))
            } else {
                self.context.is_assignment_target = false;
                self.context.is_binding_element = false;
                if self.at_keyword(Keyword::Function) {
                    self.parse_function_expr()
                } else if self.at_keyword(Keyword::This) {
                    let _ = self.next_item()?;
                    Ok(Expr::This)
                } else if self.at_keyword(Keyword::Class) {
                    let cls = self.parse_class_decl(true)?;
                    Ok(Expr::Class(cls))
                } else if self.at_import_call() {
                    // TODO: Double check this
                    let ident = self.parse_ident_name()?;
                    Ok(Expr::Ident(ident))
                } else {
                    self.expected_token_error(
                        &self.look_ahead,
                        &["function", "this", "class", "import"],
                    )
                }
            }
        } else {
            self.expected_token_error(
                &self.look_ahead,
                &[
                    "[identifier]",
                    "async",
                    "[Number]",
                    "[String]",
                    "[RegEx]",
                    "yield",
                    "let",
                    "function",
                    "this",
                    "class",
                    "import",
                ],
            )
        }
    }

    #[inline]
    fn parse_group_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_group_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenParen)?;
        if self.at_punct(Punct::CloseParen) {
            let _ = self.next_item()?;
            if !self.at_punct(Punct::EqualGreaterThan) {
                self.expect_punct(Punct::EqualGreaterThan)?;
            }
            Ok(Expr::ArrowParamPlaceHolder(vec![], false))
        } else {
            let mut params = vec![];
            if self.at_punct(Punct::Ellipsis) {
                let (_, expr) = self.parse_rest_element(&mut params)?;
                let arg = FunctionArg::Pat(expr);
                self.expect_punct(Punct::CloseParen)?;
                if !self.at_punct(Punct::EqualGreaterThan) {
                    self.expect_punct(Punct::EqualGreaterThan)?;
                }
                Ok(Expr::ArrowParamPlaceHolder(vec![arg], false))
            } else {
                self.context.is_binding_element = true;
                let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
                let mut ex = self.parse_assignment_expr()?;
                self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
                if self.at_punct(Punct::Comma) {
                    let mut exprs = vec![ex];
                    while !self.look_ahead.token.is_eof() {
                        if !self.at_punct(Punct::Comma) {
                            break;
                        }
                        let _ = self.next_item()?;
                        if self.at_punct(Punct::CloseParen) {
                            let _ = self.next_item()?;
                            return Ok(Expr::ArrowParamPlaceHolder(
                                exprs.into_iter().map(|e| FunctionArg::Expr(e)).collect(),
                                false,
                            ));
                        } else if self.at_punct(Punct::Ellipsis) {
                            if !self.context.is_binding_element {
                                return self.expected_token_error(&self.look_ahead, &["not ..."]);
                            }
                            let (_, rest) = self.parse_rest_element(&mut params)?;
                            let mut args: Vec<FunctionArg> =
                                exprs.into_iter().map(|e| FunctionArg::Expr(e)).collect();
                            args.push(FunctionArg::Pat(rest));
                            self.expect_punct(Punct::CloseParen)?;
                            return Ok(Expr::ArrowParamPlaceHolder(args, false));
                        } else {
                            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
                            exprs.push(self.parse_assignment_expr()?);
                            self.set_inherit_cover_grammar_state(
                                prev_bind,
                                prev_assign,
                                prev_first,
                            );
                        }
                    }
                    ex = Expr::Sequence(exprs);
                }
                self.expect_punct(Punct::CloseParen)?;
                if self.at_punct(Punct::EqualGreaterThan) {
                    if Self::is_ident(&ex) {
                        self.context.is_binding_element = false;
                        return Ok(Expr::ArrowParamPlaceHolder(
                            vec![FunctionArg::Expr(ex)],
                            false,
                        ));
                    }
                    if !self.context.is_binding_element {
                        return self.expected_token_error(&self.look_ahead, &["binding element"]);
                    }
                    if let Expr::Sequence(seq) = ex {
                        let args = seq.into_iter().map(|e| FunctionArg::Expr(e)).collect();
                        return Ok(Expr::ArrowParamPlaceHolder(args, false));
                    } else {
                        return Ok(Expr::ArrowParamPlaceHolder(
                            vec![FunctionArg::Expr(ex)],
                            false,
                        ));
                    }
                }
                Ok(ex)
            }
        }
    }

    #[inline]
    fn parse_array_init(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_array_init, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenBracket)?;
        let mut elements = vec![];
        while !self.at_punct(Punct::CloseBracket) {
            if self.at_punct(Punct::Comma) {
                self.next_item()?;
                elements.push(None);
            } else if self.at_punct(Punct::Ellipsis) {
                let el = self.parse_spread_element()?;
                if !self.at_punct(Punct::CloseBracket) {
                    self.context.is_assignment_target = false;
                    self.context.is_binding_element = false;
                    self.expect_punct(Punct::Comma)?;
                }
                elements.push(Some(el))
            } else {
                let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
                elements.push(Some(self.parse_assignment_expr()?));
                self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
                if !self.at_punct(Punct::CloseBracket) {
                    self.expect_punct(Punct::Comma)?;
                }
            }
        }
        self.expect_punct(Punct::CloseBracket)?;
        Ok(Expr::Array(elements))
    }
    #[inline]
    fn parse_obj_init(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_obj_init, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenBrace)?;
        let mut props = vec![];
        let mut has_proto = false;
        while !self.at_punct(Punct::CloseBrace) {
            let prop = if self.at_punct(Punct::Ellipsis) {
                let spread = self.parse_spread_element()?;
                ObjectProperty::Spread(Box::new(spread))
            } else {
                let (found_proto, prop) = self.parse_obj_prop(has_proto)?;
                has_proto = has_proto || found_proto;
                prop
            };
            props.push(prop);
            if !self.at_punct(Punct::CloseBrace) {
                self.expect_comma_sep()?;
            }
        }
        self.expect_punct(Punct::CloseBrace)?;
        Ok(Expr::Object(props))
    }

    #[inline]
    fn parse_obj_prop(&mut self, has_proto: bool) -> Res<(bool, ObjectProperty<'b>)> {
        debug!("{}: parse_obj_prop, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let start = self.look_ahead.clone();
        let start_pos = self.look_ahead_position;
        let mut has_proto = has_proto;
        let mut at_get = false;
        let mut at_set = false;
        let (key, is_async, computed) = if let Token::Ident(ref id) = start.token {
            let id = id.as_str();
            at_get = id == "get";
            at_set = id == "set";
            let _ = self.next_item()?;
            let computed = self.at_punct(Punct::OpenBracket);
            let is_async = self.context.has_line_term
                && id == "async"
                && !self.at_punct(Punct::Colon)
                && !self.at_punct(Punct::Asterisk)
                && !self.at_punct(Punct::Comma);
            let key = if is_async {
                self.parse_object_property_key()?
            } else {
                PropertyKey::Expr(Expr::Ident(id))
            };
            (Some(key), is_async, computed)
        } else if self.at_punct(Punct::Asterisk) {
            self.next_item()?;
            (None, false, false)
        } else {
            let computed = self.at_punct(Punct::OpenBracket);
            let key = self.parse_object_property_key()?;
            (Some(key), false, computed)
        };
        let at_qualified = self.at_qualified_prop_key();
        let prop = if at_get && at_qualified && !is_async {
            ObjectProperty::Property(Property {
                computed: self.at_punct(Punct::OpenBracket),
                key: self.parse_object_property_key()?,
                value: self.parse_getter_method()?,
                kind: PropertyKind::Get,
                method: false,
                short_hand: false,
            })
        } else if at_set && at_qualified && !is_async {
            ObjectProperty::Property(Property {
                computed: self.at_punct(Punct::OpenBracket),
                key: self.parse_object_property_key()?,
                value: self.parse_setter_method()?,
                kind: PropertyKind::Set,
                method: false,
                short_hand: false,
            })
        } else if start.token.matches_punct(Punct::Asterisk) && at_qualified {
            ObjectProperty::Property(Property {
                computed: self.at_punct(Punct::OpenBracket),
                key: self.parse_object_property_key()?,
                value: self.parse_generator_method()?,
                kind: PropertyKind::Init,
                method: true,
                short_hand: false,
            })
        } else {
            if let Some(key) = key {
                let kind = PropertyKind::Init;
                if self.at_punct(Punct::Colon) && !is_async {
                    if !computed && Self::is_proto_(&key) {
                        if has_proto {
                            self.tolerate_error(Error::Redecl(
                                start_pos,
                                "prototype can only be declared once".to_string(),
                            ))?;
                        }
                        has_proto = true;
                    }
                    let _ = self.next_item()?;
                    let (prev_bind, prev_assign, prev_first) = self.get_cover_grammar_state();
                    let value = self.parse_assignment_expr()?;
                    self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
                    ObjectProperty::Property(Property {
                        computed,
                        key,
                        value: PropertyValue::Expr(value),
                        kind,
                        method: false,
                        short_hand: false,
                    })
                } else if self.at_punct(Punct::OpenParen) {
                    ObjectProperty::Property(Property {
                        computed,
                        key,
                        value: if is_async {
                            self.parse_async_property_method()?
                        } else {
                            self.parse_property_method()?
                        },
                        kind,
                        method: true,
                        short_hand: false,
                    })
                } else if start.token.is_ident() {
                    if self.at_punct(Punct::Equal) {
                        self.context.first_covert_initialized_name_error =
                            Some(self.look_ahead.clone());
                        let _ = self.next_item()?;
                        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
                        let inner = self.parse_assignment_expr()?;
                        self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                        ObjectProperty::Property(Property {
                            computed,
                            key,
                            value: PropertyValue::Expr(inner),
                            kind,
                            method: false,
                            short_hand: true,
                        })
                    } else {
                        ObjectProperty::Property(Property {
                            computed,
                            key,
                            value: PropertyValue::None,
                            kind,
                            method: false,
                            short_hand: true,
                        })
                    }
                } else {
                    return self.expected_token_error(&start, &["object property value"]);
                }
            } else {
                return self.expected_token_error(&start, &["object property key"]);
            }
        };
        Ok((has_proto, prop))
    }

    #[inline]
    fn is_proto_(key: &PropertyKey) -> bool {
        match key {
            PropertyKey::Literal(ref l) => match l {
                Literal::String(ref s) => s == &"_proto_",
                _ => false,
            },
            _ => false,
        }
    }

    #[inline]
    fn at_possible_ident(&self) -> bool {
        self.look_ahead.token.is_ident()
            || self.look_ahead.token.is_keyword()
            // || self.look_ahead.token.is_bool()
            || self.look_ahead.token.is_null()
            || if let Token::Boolean(_) = self.look_ahead.token {
                true
            } else {
                false
            }
    }

    #[inline]
    fn parse_template_literal(&mut self) -> Res<TemplateLiteral<'b>> {
        debug!("{}: parse_template_literal, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if !self.look_ahead.token.is_template_head() {
            return self
                .expected_token_error(&self.look_ahead, &["template head", "template no sub"]);
        }
        let mut expressions = vec![];
        let mut quasis = vec![];
        let quasi = self.parse_template_element()?;
        let mut breaking = quasi.tail;
        quasis.push(quasi);
        while !breaking {
            expressions.push(self.parse_expression()?);
            let quasi = self.parse_template_element()?;
            breaking = quasi.tail;
            quasis.push(quasi);
        }
        Ok(TemplateLiteral {
            expressions,
            quasis,
        })
    }

    #[inline]
    fn parse_template_element(&mut self) -> Res<TemplateElement<'b>> {
        debug!("{}: parse_template_element, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let item = self.next_item()?;
        if let Token::Template(t) = item.token {
            let raw = self.get_string(&item.span)?;
            let (cooked, tail) = match t {
                ress::Template::Head(c) => (c, false),
                ress::Template::Middle(c) => (c, false),
                ress::Template::Tail(c) => (c, true),
                ress::Template::NoSub(c) => (c, true),
            };
            Ok(TemplateElement { raw, cooked, tail })
        } else {
            self.expected_token_error(&self.look_ahead, &["Template part"])
        }
    }

    #[inline]
    fn parse_function_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_function_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let is_async = self.at_contextual_keyword("async");
        if is_async {
            let _ = self.next_item()?;
        }
        self.expect_keyword(Keyword::Function)?;
        let is_gen = self.at_punct(Punct::Asterisk);
        if is_gen {
            let _ = self.next_item()?;
        }
        let prev_await = self.context.r#await;
        let prev_yield = self.context.allow_yield;
        self.context.r#await = is_async;
        self.context.allow_yield = !is_gen;
        let mut found_restricted = false;
        let id = if !self.at_punct(Punct::OpenParen) {
            let item = self.look_ahead.clone();
            let id = self.parse_fn_name(is_gen)?;
            if item.token.is_restricted() {
                if self.context.strict {
                    if !self.config.tolerant {
                        return self
                            .unexpected_token_error(&item, "restricted ident in strict context");
                    }
                } else {
                    found_restricted = true;
                }
            }
            if item.token.is_strict_reserved() {
                found_restricted = true;
            }
            Some(id)
        } else {
            None
        };
        let formal_params = self.parse_formal_params()?;
        found_restricted = found_restricted || formal_params.found_restricted;
        let prev_strict = self.context.strict;
        let prev_strict_dir = formal_params.simple;
        let start = self.look_ahead.clone();
        let body = self.parse_function_source_el()?;
        if self.context.strict && found_restricted {
            //TODO: Double check this
            if !self.config.tolerant {
                return self.unexpected_token_error(&start, "restricted ident in strict context");
            }
        }
        self.context.strict = prev_strict;
        self.context.allow_strict_directive = prev_strict_dir;
        self.context.allow_yield = prev_yield;
        self.context.r#await = prev_await;
        let func = Function {
            id,
            params: formal_params.params,
            body,
            generator: is_gen,
            is_async,
        };
        Ok(Expr::Function(func))
    }

    #[inline]
    fn parse_fn_name(&mut self, is_gen: bool) -> Res<&'b str> {
        debug!("{}: parse_fn_name, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.context.strict && !is_gen && self.at_keyword(Keyword::Yield) {
            self.parse_ident_name()
        } else {
            self.parse_var_ident(false)
        }
    }

    #[inline]
    fn parse_ident_name(&mut self) -> Res<&'b str> {
        debug!("{}: parse_ident_name, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let lh = self.get_string(&self.look_ahead.span);
        let ident = self.next_item()?;
        let ret = self.get_string(&ident.span)?;
        Ok(ret)
    }

    #[inline]
    fn parse_var_ident(&mut self, is_var: bool) -> Res<&'b str> {
        debug!("{}: parse_var_ident, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let ident = self.next_item()?;
        if ident.token.matches_keyword(Keyword::Yield) {
            if self.context.strict || !self.context.allow_yield {
                return self.expected_token_error(&ident, &["variable identifier"]);
            }
        } else if !ident.token.is_ident() {
            if self.context.strict && ident.token.is_strict_reserved() {
                return self.expected_token_error(&ident, &["variable identifier"]);
            }
            if self.context.strict || ident.token.matches_keyword(Keyword::Let) || !is_var {
                return self.expected_token_error(&ident, &["variable identifier"]);
            }
        } else if (self.context.is_module || self.context.r#await)
            && &self.original[ident.span.start..ident.span.end] == "await"
        {
            return self.expected_token_error(&ident, &["variable identifier"]);
        }
        let i: &'b str = match &ident.token {
            &Token::Ident(_) | &Token::Keyword(_) => {
                self.scanner.str_for(&ident.span).unwrap_or("")
            }
            _ => self.expected_token_error(&ident, &["variable identifier"])?,
        };
        Ok(i)
    }

    #[inline]
    fn parse_formal_params(&mut self) -> Res<FormalParams<'b>> {
        debug!("{}: parse_formal_params, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenParen)?;
        let mut args = vec![];
        let mut simple: bool = true;
        let mut found_restricted = false;
        if !self.at_punct(Punct::CloseParen) {
            while !self.look_ahead.token.is_eof() {
                let (s, r, arg) = self.parse_formal_param(simple)?;
                simple = simple && s;
                found_restricted = found_restricted || r;
                args.push(arg);
                if self.at_punct(Punct::CloseParen) {
                    break;
                }
                self.expect_punct(Punct::Comma)?;
                if self.at_punct(Punct::CloseParen) {
                    break;
                }
            }
        }
        self.expect_punct(Punct::CloseParen)?;

        Ok(FormalParams {
            params: args,
            strict: false,
            found_restricted,
            simple,
        })
    }

    #[inline]
    fn parse_formal_param(&mut self, simple: bool) -> Res<(bool, bool, FunctionArg<'b>)> {
        debug!("{}: parse_formal_param, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut params: Vec<Item<Token<'b>>> = Vec::new();
        let (found_restricted, param) = if self.at_punct(Punct::Ellipsis) {
            let (found_restricted, pat) = self.parse_rest_element(&mut params)?;
            (found_restricted, FunctionArg::Pat(pat))
        } else {
            let (found_restricted, pat) = self.parse_pattern_with_default(&mut params)?;
            (found_restricted, FunctionArg::Pat(pat))
        };
        let simple = simple && Self::is_simple(&param);
        Ok((simple, found_restricted, param))
    }

    #[inline]
    fn parse_rest_element(&mut self, params: &mut Vec<Item<Token<'b>>>) -> Res<(bool, Pat<'b>)> {
        debug!("{}: parse_rest_element, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::Ellipsis)?;
        let (restricted, arg) = self.parse_pattern(None, params)?;
        let ret = Pat::RestElement(Box::new(arg));
        if self.at_punct(Punct::Equal) {
            return self.expected_token_error(&self.look_ahead, &["not assignment"]);
        }
        if !self.at_punct(Punct::CloseParen) {
            return self.expected_token_error(&self.look_ahead, &[")"]);
        }
        Ok((restricted, ret))
    }

    #[inline]
    fn parse_binding_rest_el(&mut self, params: &mut Vec<Item<Token<'b>>>) -> Res<(bool, Pat<'b>)> {
        debug!("{}: parse_binding_rest_el, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::Ellipsis)?;
        self.parse_pattern(None, params)
    }

    #[inline]
    fn parse_pattern_with_default(
        &mut self,
        params: &mut Vec<Item<Token<'b>>>,
    ) -> Res<(bool, Pat<'b>)> {
        debug!("{}: parse_pattern_with_default, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let (is_restricted, ret) = self.parse_pattern(None, params)?;
        if self.at_punct(Punct::Equal) {
            let _assign = self.next_item()?;
            let prev_yield = self.context.allow_yield;
            self.context.allow_yield = true;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let right = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            self.context.allow_yield = prev_yield;
            return Ok((
                is_restricted,
                Pat::Assignment(AssignmentPat {
                    left: Box::new(ret),
                    right: Box::new(right),
                }),
            ));
        }
        Ok((is_restricted, ret))
    }

    #[inline]
    fn parse_pattern(
        &mut self,
        kind: Option<VariableKind>,
        params: &mut Vec<Item<Token<'b>>>,
    ) -> Res<(bool, Pat<'b>)> {
        debug!("{}: parse_pattern, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.at_punct(Punct::OpenBracket) {
            let kind = kind.unwrap_or(VariableKind::Var);
            self.parse_array_pattern(params, kind)
        } else if self.at_punct(Punct::OpenBrace) {
            self.parse_object_pattern()
        } else {
            let is_var = if let Some(kind) = kind {
                match kind {
                    VariableKind::Const | VariableKind::Let => {
                        if self.at_keyword(Keyword::Let) {
                            return self.expected_token_error(&self.look_ahead, &["identifier"]);
                        }
                        false
                    }
                    VariableKind::Var => true,
                }
            } else {
                true
            };
            let ident = self.parse_var_ident(is_var)?;
            let restricted = &ident == &"eval" || &ident == &"arguments";
            params.push(self.look_ahead.clone());
            Ok((restricted, Pat::Identifier(ident)))
        }
    }

    #[inline]
    fn parse_array_pattern(
        &mut self,
        params: &mut Vec<Item<Token<'b>>>,
        _kind: VariableKind,
    ) -> Res<(bool, Pat<'b>)> {
        debug!("{}: parse_array_pattern, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenBracket)?;
        let mut elements = vec![];
        while !self.at_punct(Punct::CloseBracket) {
            if self.at_punct(Punct::Comma) {
                let _ = self.next_item()?;
                elements.push(None);
            } else {
                if self.at_punct(Punct::Ellipsis) {
                    let (_, el) = self.parse_binding_rest_el(params)?;
                    elements.push(Some(ArrayPatPart::Pat(el)));
                    break;
                } else {
                    let (_, el) = self.parse_pattern_with_default(params)?;
                    elements.push(Some(ArrayPatPart::Pat(el)));
                }
                if !self.at_punct(Punct::CloseBracket) {
                    self.expect_punct(Punct::Comma)?;
                }
            }
        }
        self.expect_punct(Punct::CloseBracket)?;
        Ok((false, Pat::Array(elements)))
    }

    #[inline]
    fn parse_object_pattern(&mut self) -> Res<(bool, Pat<'b>)> {
        debug!("{}: parse_object_pattern, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenBrace)?;
        let mut body = vec![];
        while !self.at_punct(Punct::CloseBrace) {
            let el = if self.at_punct(Punct::Ellipsis) {
                self.parse_rest_prop()?
            } else {
                self.parse_property_pattern()?
            };
            body.push(el);
            if !self.at_punct(Punct::CloseBrace) {
                self.expect_punct(Punct::Comma)?;
            }
        }
        self.expect_punct(Punct::CloseBrace)?;
        Ok((false, Pat::Object(body)))
    }

    #[inline]
    fn parse_rest_prop(&mut self) -> Res<ObjectPatPart<'b>> {
        debug!("{}: parse_rest_prop, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::Ellipsis)?;
        let (_, arg) = self.parse_pattern(None, &mut vec![])?;
        if self.at_punct(Punct::Equal) {
            //unexpected token
        }
        if !self.at_punct(Punct::CloseBrace) {
            //unable to parse props after rest
        }
        let rest = Pat::RestElement(Box::new(arg));
        let part = ObjectPatPart::Rest(Box::new(rest));
        Ok(part)
    }

    #[inline]
    fn parse_property_pattern(&mut self) -> Res<ObjectPatPart<'b>> {
        debug!("{}: parse_property_pattern, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let mut computed = false;
        let mut short_hand = false;
        let method = false;
        let (key, value) = if self.look_ahead.token.is_ident() {
            let key = PropertyKey::Expr(Expr::Ident(self.parse_var_ident(false)?));
            let value = if self.at_punct(Punct::Equal) {
                self.expect_punct(Punct::Equal)?;
                short_hand = true;
                let e = self.parse_assignment_expr()?;
                PropertyValue::Expr(e)
            } else if !self.at_punct(Punct::Colon) {
                short_hand = true;
                PropertyValue::None
            } else {
                self.expect_punct(Punct::Colon)?;
                let (_, p) = self.parse_pattern_with_default(&mut vec![])?;
                PropertyValue::Pat(p)
            };
            (key, value)
        } else {
            computed = self.at_punct(Punct::OpenBracket);
            let key = self.parse_object_property_key()?;
            self.expect_punct(Punct::Colon)?;
            let (_, v) = self.parse_pattern_with_default(&mut vec![])?;
            let value = PropertyValue::Pat(v);
            (key, value)
        };
        Ok(ObjectPatPart::Assignment(Property {
            key,
            value,
            computed,
            short_hand,
            method,
            kind: PropertyKind::Init,
        }))
    }

    #[inline]
    fn parse_assignment_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_assignment_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if !self.context.allow_yield && self.at_keyword(Keyword::Yield) {
            return self.parse_yield_expr();
        } else {
            let ress::Span { start, end } = self.look_ahead.span;
            let start_pos = self.look_ahead_position;
            let mut current = self.parse_conditional_expr()?;
            let curr_line = self.look_ahead_position.line;
            let start_line = start_pos.line;
            if &self.original[start..end] == "async"
                && curr_line == start_line
                && (self.look_ahead.token.is_ident() || self.at_keyword(Keyword::Yield))
            {
                let arg = self.parse_primary_expression()?;
                let arg = self.reinterpret_expr_as_pat(arg)?;
                let arg = FunctionArg::Pat(arg);
                current = Expr::ArrowParamPlaceHolder(vec![arg], true);
            }
            if Self::is_arrow_param_placeholder(&current) || self.at_punct(Punct::EqualGreaterThan)
            {
                self.context.is_assignment_target = false;
                self.context.is_binding_element = false;
                let is_async = Self::is_async(&current);
                if let Some(params) = self.reinterpret_as_cover_formals_list(current.clone())? {
                    self.expect_punct(Punct::EqualGreaterThan)?;
                    if self.at_punct(Punct::OpenBrace) {
                        let prev_in = self.context.allow_in;
                        self.context.allow_in = true;
                        let body = self.parse_function_source_el()?;
                        self.context.allow_in = prev_in;
                        current = Expr::ArrowFunction(ArrowFunctionExpr {
                            id: None,
                            expression: false,
                            generator: false,
                            is_async,
                            params,
                            body: ArrowFunctionBody::FunctionBody(body),
                        });
                    } else {
                        let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                        let a = self.parse_assignment_expr()?;
                        self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                        current = Expr::ArrowFunction(ArrowFunctionExpr {
                            id: None,
                            expression: true,
                            generator: false,
                            is_async,
                            params,
                            body: ArrowFunctionBody::Expr(Box::new(a)),
                        });
                    };
                }
            } else {
                if self.at_assign() {
                    if !self.context.is_assignment_target {
                        if !self.config.tolerant {
                            return self.unexpected_token_error(
                                &self.look_ahead,
                                "Not at assignment target",
                            );
                        }
                    }
                    if self.context.strict && Self::is_ident(&current) {
                        if let Expr::Ident(ref i) = current {
                            if Self::is_restricted_word(i) {
                                return self.expected_token_error(
                                    &self.look_ahead,
                                    &[&format!("not {}", i)],
                                );
                            }
                            if Self::is_strict_reserved(i) {
                                return self.expected_token_error(
                                    &self.look_ahead,
                                    &[&format!("not {}", i)],
                                );
                            }
                        }
                    }
                    let left = if !self.at_punct(Punct::Equal) {
                        self.context.is_assignment_target = false;
                        self.context.is_binding_element = false;
                        AssignmentLeft::Expr(Box::new(current))
                    } else {
                        AssignmentLeft::Expr(Box::new(current))
                    };
                    let item = self.next_item()?;
                    let op = match &item.token {
                        &Token::Punct(ref p) => {
                            if let Some(op) = Self::assignment_operator(*p) {
                                op
                            } else {
                                return self.expected_token_error(
                                    &item,
                                    &[
                                        "=", "+=", "-=", "/=", "*=", "**=", "|=", "&=", "~=", "%=",
                                        "<<=", ">>=", ">>>=",
                                    ],
                                );
                            }
                        }
                        _ => {
                            return self.expected_token_error(
                                &item,
                                &[
                                    "=", "+=", "-=", "/=", "*=", "**=", "|=", "&=", "~=", "%=",
                                    "<<=", ">>=", ">>>=",
                                ],
                            );
                        }
                    };
                    let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                    let right = self.parse_assignment_expr()?;
                    self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                    self.context.first_covert_initialized_name_error = None;
                    return Ok(Expr::Assignment(AssignmentExpr {
                        operator: op,
                        left,
                        right: Box::new(right),
                    }));
                }
            }
            return Ok(current);
        }
    }

    #[inline]
    fn is_async(expr: &Expr) -> bool {
        match expr {
            &Expr::Function(ref f) => f.is_async,
            &Expr::ArrowFunction(ref f) => f.is_async,
            &Expr::ArrowParamPlaceHolder(_, b) => b,
            _ => false,
        }
    }

    fn assignment_operator(p: Punct) -> Option<AssignmentOperator> {
        match p {
            ress::Punct::Equal => Some(AssignmentOperator::Equal),
            ress::Punct::PlusEqual => Some(AssignmentOperator::PlusEqual),
            ress::Punct::DashEqual => Some(AssignmentOperator::MinusEqual),
            ress::Punct::AsteriskEqual => Some(AssignmentOperator::TimesEqual),
            ress::Punct::ForwardSlashEqual => Some(AssignmentOperator::DivEqual),
            ress::Punct::PercentEqual => Some(AssignmentOperator::ModEqual),
            ress::Punct::DoubleLessThanEqual => Some(AssignmentOperator::LeftShiftEqual),
            ress::Punct::DoubleGreaterThanEqual => Some(AssignmentOperator::RightShiftEqual),
            ress::Punct::TripleGreaterThanEqual => {
                Some(AssignmentOperator::UnsignedRightShiftEqual)
            }
            ress::Punct::PipeEqual => Some(AssignmentOperator::OrEqual),
            ress::Punct::CaretEqual => Some(AssignmentOperator::XOrEqual),
            ress::Punct::AmpersandEqual => Some(AssignmentOperator::AndEqual),
            ress::Punct::DoubleAsteriskEqual => Some(AssignmentOperator::PowerOfEqual),
            _ => None,
        }
    }

    #[inline]
    fn is_arrow_param_placeholder(expr: &Expr) -> bool {
        match expr {
            &Expr::ArrowParamPlaceHolder(_, _) => true,
            _ => false,
        }
    }

    fn reinterpret_as_cover_formals_list(
        &mut self,
        expr: Expr<'b>,
    ) -> Res<Option<Vec<FunctionArg<'b>>>> {
        let (mut params, async_arrow) = if Self::is_ident(&expr) {
            (vec![FunctionArg::Expr(expr)], false)
        } else if let Expr::ArrowParamPlaceHolder(params, is_async) = expr {
            (params, is_async)
        } else {
            return Ok(None);
        };
        let mut invalid_param = false;
        params = params
            .into_iter()
            .map(|p| {
                if Self::is_assignment(&p) {
                    match &p {
                        FunctionArg::Pat(ref p) => match p {
                            Pat::Assignment(ref a) => match &*a.right {
                                Expr::Yield(ref y) => {
                                    if y.argument.is_some() {
                                        invalid_param = true;
                                    } else {
                                        return FunctionArg::Pat(Pat::Identifier("yield"));
                                    }
                                }
                                _ => (),
                            },
                            _ => (),
                        },
                        FunctionArg::Expr(ref e) => match e {
                            Expr::Assignment(ref a) => match &*a.right {
                                Expr::Yield(ref y) => {
                                    if y.argument.is_some() {
                                        invalid_param = true;
                                    } else {
                                        return FunctionArg::Expr(Expr::Ident("yield"));
                                    }
                                }
                                _ => (),
                            },
                            _ => (),
                        },
                    }
                    p
                } else if async_arrow && Self::is_await(&p) {
                    invalid_param = true;
                    p
                } else {
                    p
                }
            })
            .collect();
        if invalid_param {
            return self.expected_token_error(
                &self.look_ahead,
                &["not a yield expression in a function param"],
            );
        }
        if self.context.strict && !self.context.allow_yield {
            for param in params.iter() {
                match param {
                    FunctionArg::Expr(ref e) => match e {
                        Expr::Yield(_) => {
                            return self.expected_token_error(
                                &self.look_ahead,
                                &["not a yield expression in a function param"],
                            );
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
        }
        Ok(Some(params))
    }

    #[inline]
    fn is_await(arg: &FunctionArg) -> bool {
        match arg {
            FunctionArg::Expr(ref e) => match e {
                Expr::Ident(ref i) => i == &"await",
                _ => false,
            },
            FunctionArg::Pat(ref p) => match p {
                Pat::Identifier(ref i) => i == &"await",
                _ => false,
            },
        }
    }

    #[inline]
    fn is_assignment(arg: &FunctionArg) -> bool {
        match arg {
            FunctionArg::Pat(ref p) => match p {
                Pat::Assignment(_) => true,
                _ => false,
            },
            FunctionArg::Expr(ref e) => match e {
                Expr::Assignment(_) => true,
                _ => false,
            },
        }
    }
    #[inline]
    pub fn is_simple(arg: &FunctionArg) -> bool {
        match arg {
            FunctionArg::Pat(ref p) => match p {
                Pat::Identifier(_) => true,
                _ => false,
            },
            FunctionArg::Expr(ref e) => match e {
                Expr::Ident(_) => true,
                _ => false,
            },
        }
    }

    fn reinterpret_expr_as_pat(&self, ex: Expr<'b>) -> Res<Pat<'b>> {
        match ex {
            Expr::Array(a) => {
                let parts = a
                    .into_iter()
                    .map(|e| {
                        if let Some(e) = e {
                            Some(ArrayPatPart::Expr(e))
                        } else {
                            None
                        }
                    })
                    .collect();
                Ok(Pat::Array(parts))
            }
            Expr::Spread(s) => Ok(Pat::RestElement(Box::new(
                self.reinterpret_expr_as_pat(*s)?,
            ))),
            Expr::Object(o) => {
                let mut patts = vec![];
                for expr in o {
                    match expr {
                        ObjectProperty::Property(p) => patts.push(ObjectPatPart::Assignment(p)),
                        ObjectProperty::Spread(s) => {
                            let p = self.reinterpret_expr_as_pat(*s)?;
                            patts.push(ObjectPatPart::Rest(Box::new(p)));
                        }
                    }
                }
                Ok(Pat::Object(patts))
            }
            Expr::Assignment(a) => {
                let left = match a.left {
                    AssignmentLeft::Pat(p) => p,
                    AssignmentLeft::Expr(e) => self.reinterpret_expr_as_pat(*e)?,
                };
                let ret = AssignmentPat {
                    left: Box::new(left),
                    right: a.right,
                };
                Ok(Pat::Assignment(ret))
            }
            Expr::Ident(i) => Ok(Pat::Identifier(i)),
            _ => Err(self.reinterpret_error("expression", "pattern")),
        }
    }

    #[inline]
    fn parse_yield_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_yield_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::Yield)?;
        let mut arg: Option<Box<Expr>> = None;
        let mut delegate = false;
        if !self.context.has_line_term {
            let prev_yield = self.context.allow_yield;
            self.context.allow_yield = false;
            delegate = self.at_punct(Punct::Asterisk);
            if delegate {
                let _start = self.next_item()?;
                arg = Some(Box::new(self.parse_assignment_expr()?));
            } else if self.is_start_of_expr() {
                arg = Some(Box::new(self.parse_assignment_expr()?));
            }
            self.context.allow_yield = prev_yield;
        }
        let y = YieldExpr {
            argument: arg,
            delegate,
        };
        Ok(Expr::Yield(y))
    }

    #[inline]
    fn parse_conditional_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_conditional_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
        let expr = self.parse_binary_expression()?;
        self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
        if self.at_punct(Punct::QuestionMark) {
            let _question_mark = self.next_item()?;
            let prev_in = self.context.allow_in;
            self.context.allow_in = true;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let if_true = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            self.context.allow_in = prev_in;

            self.expect_punct(Punct::Colon)?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let if_false = self.parse_assignment_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;

            let c = ConditionalExpr {
                test: Box::new(expr),
                alternate: Box::new(if_false),
                consequent: Box::new(if_true),
            };
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            return Ok(Expr::Conditional(c));
        }
        Ok(expr)
    }

    #[inline]
    fn parse_binary_expression(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_binary_expression, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
        let mut current = self.parse_exponentiation_expression()?;
        self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
        let token = self.look_ahead.clone();
        let mut prec = self.bin_precedence(&token.token);
        if prec > 0 {
            self.next_item()?;
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let mut left = current.clone();
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let mut right = self.parse_exponentiation_expression()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            let mut stack = vec![left.clone(), right.clone()];
            let mut ops = vec![token.token.clone()];
            let mut precs = vec![prec];
            loop {
                prec = self.bin_precedence(&self.look_ahead.token);
                if prec <= 0 {
                    break;
                }
                while stack.len() > 1 && ops.len() > 0 && prec <= precs[precs.len() - 1] {
                    right = stack
                        .pop()
                        .ok_or(self.op_error("invalid binary operation, no right expr in stack"))?;
                    let op = ops
                        .pop()
                        .ok_or(self.op_error("invalid binary operation, too few operators"))?;
                    let _ = precs.pop();
                    left = stack
                        .pop()
                        .ok_or(self.op_error("invalid binary operation, no left expr in stack"))?;
                    if op.matches_punct(Punct::DoubleAmpersand)
                        || op.matches_punct(Punct::DoublePipe)
                    {
                        stack.push(Expr::Logical(LogicalExpr {
                            operator: Self::logical_operator(&op)
                                .ok_or(self.op_error("Unable to convert logical operator"))?,
                            left: Box::new(left),
                            right: Box::new(right),
                        }));
                    } else {
                        let operator = Self::binary_operator(&op)
                            .ok_or(self.op_error("Unable to convert binary operator"))?;
                        stack.push(Expr::Binary(BinaryExpr {
                            operator,
                            left: Box::new(left),
                            right: Box::new(right),
                        }));
                    }
                }
                ops.push(self.next_item()?.token);
                precs.push(prec);
                let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                let exp = self.parse_exponentiation_expression()?;
                self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                stack.push(exp);
            }
            current = stack
                .pop()
                .ok_or(self.op_error("invalid binary operation, too few expressions"))?;

            while ops.len() > 0 && stack.len() > 0 {
                let op = ops
                    .pop()
                    .ok_or(self.op_error("invalid binary operation, too few operators"))?;
                if op.matches_punct(Punct::DoubleAmpersand) || op.matches_punct(Punct::DoublePipe) {
                    let operator = Self::logical_operator(&op)
                        .ok_or(self.op_error("Unable to convert logical operator"))?;
                    current = Expr::Logical(LogicalExpr {
                        operator,
                        left: Box::new(stack.pop().ok_or(
                            self.op_error("invalid logical operation, too few expressions"),
                        )?),
                        right: Box::new(current),
                    })
                } else {
                    let operator = Self::binary_operator(&op)
                        .ok_or(self.op_error("Unable to convert binary operator"))?;
                    current = Expr::Binary(BinaryExpr {
                        operator,
                        left: Box::new(stack.pop().ok_or(
                            self.op_error("invalid binary operation, too few expressions"),
                        )?),
                        right: Box::new(current),
                    });
                }
            }
        }
        Ok(current)
    }

    #[inline]
    fn parse_exponentiation_expression(&mut self) -> Res<Expr<'b>> {
        debug!(
            "{}: parse_exponentiation_expression, {:?}",
            self.look_ahead.span.start,
            self.look_ahead.token
        );
        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
        let expr = self.parse_unary_expression()?;
        self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
        if self.at_punct(Punct::DoubleAsterisk) {
            let _stars = self.next_item()?;
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let left = expr;
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let right = self.parse_exponentiation_expression()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            return Ok(Expr::Binary(BinaryExpr {
                operator: BinaryOperator::PowerOf,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }

        Ok(expr)
    }

    #[inline]
    fn parse_unary_expression(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_unary_expression, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.at_punct(Punct::Plus)
            || self.at_punct(Punct::Dash)
            || self.at_punct(Punct::Tilde)
            || self.at_punct(Punct::Bang)
            || self.at_keyword(Keyword::Delete)
            || self.at_keyword(Keyword::Void)
            || self.at_keyword(Keyword::TypeOf)
        {
            let op = self.next_item()?;
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let arg = self.parse_unary_expression()?;
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            if op.token.matches_keyword(Keyword::Delete)
                && self.context.strict
                && Self::is_ident(&arg)
            {
                if !self.config.tolerant {
                    return self.unexpected_token_error(&op, "Cannot delete ident in strict mode");
                }
            }
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let operator = Self::unary_operator(&op.token)
                .ok_or(self.op_error("Unable to convert unary operator"))?;
            Ok(Expr::Unary(UnaryExpr {
                prefix: true,
                operator,
                argument: Box::new(arg),
            }))
        } else if self.context.r#await && self.at_keyword(Keyword::Await) {
            self.parse_await_expr()
        } else {
            self.parse_update_expr()
        }
    }

    fn unary_operator(token: &Token) -> Option<UnaryOperator> {
        match token {
            Token::Punct(ref p) => match p {
                ress::Punct::Dash => Some(UnaryOperator::Minus),
                ress::Punct::Plus => Some(UnaryOperator::Plus),
                ress::Punct::Bang => Some(UnaryOperator::Not),
                ress::Punct::Tilde => Some(UnaryOperator::Tilde),
                _ => None,
            },
            Token::Keyword(ref k) => match k {
                ress::Keyword::TypeOf => Some(UnaryOperator::TypeOf),
                ress::Keyword::Void => Some(UnaryOperator::Void),
                ress::Keyword::Delete => Some(UnaryOperator::Delete),
                _ => None,
            },
            _ => None,
        }
    }

    fn binary_operator(token: &Token) -> Option<BinaryOperator> {
        match token {
            Token::Keyword(ref key) => match key {
                ress::Keyword::InstanceOf => Some(BinaryOperator::InstanceOf),
                ress::Keyword::In => Some(BinaryOperator::In),
                _ => None,
            },
            Token::Punct(ref p) => match p {
                ress::Punct::DoubleEqual => Some(BinaryOperator::Equal),
                ress::Punct::BangEqual => Some(BinaryOperator::NotEqual),
                ress::Punct::TripleEqual => Some(BinaryOperator::StrictEqual),
                ress::Punct::BangDoubleEqual => Some(BinaryOperator::StrictNotEqual),
                ress::Punct::LessThan => Some(BinaryOperator::LessThan),
                ress::Punct::LessThanEqual => Some(BinaryOperator::LessThanEqual),
                ress::Punct::GreaterThan => Some(BinaryOperator::GreaterThan),
                ress::Punct::GreaterThanEqual => Some(BinaryOperator::GreaterThanEqual),
                ress::Punct::DoubleLessThan => Some(BinaryOperator::LeftShift),
                ress::Punct::DoubleGreaterThan => Some(BinaryOperator::RightShift),
                ress::Punct::TripleGreaterThan => Some(BinaryOperator::UnsignedRightShift),
                ress::Punct::Plus => Some(BinaryOperator::Plus),
                ress::Punct::Dash => Some(BinaryOperator::Minus),
                ress::Punct::Asterisk => Some(BinaryOperator::Times),
                ress::Punct::ForwardSlash => Some(BinaryOperator::Over),
                ress::Punct::Percent => Some(BinaryOperator::Mod),
                ress::Punct::Ampersand => Some(BinaryOperator::And),
                ress::Punct::Pipe => Some(BinaryOperator::Or),
                ress::Punct::Caret => Some(BinaryOperator::XOr),
                _ => None,
            },
            _ => None,
        }
    }

    fn logical_operator(token: &Token) -> Option<LogicalOperator> {
        match token {
            Token::Punct(ref p) => match p {
                ress::Punct::DoubleAmpersand => Some(LogicalOperator::And),
                ress::Punct::DoublePipe => Some(LogicalOperator::Or),
                _ => None,
            },
            _ => None,
        }
    }

    #[inline]
    fn parse_await_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_await_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let _await = self.next_item()?;
        let arg = self.parse_unary_expression()?;
        Ok(Expr::Await(Box::new(arg)))
    }

    #[inline]
    fn parse_update_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_update_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let start = self.look_ahead.clone();
        if self.at_punct(Punct::DoublePlus) || self.at_punct(Punct::DoubleDash) {
            let op = self.next_item()?;
            let operator = match op.token {
                Token::Punct(ref p) => match p {
                    Punct::DoublePlus => UpdateOperator::Increment,
                    Punct::DoubleDash => UpdateOperator::Decrement,
                    _ => unreachable!("Already validated that the next token would be ++ or --"),
                },
                _ => unreachable!("Already validated that the next token would be ++ or --"),
            };
            let start = self.look_ahead.clone();
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let ex = self.parse_unary_expression()?;
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            if self.context.strict && Self::is_ident(&ex) {
                match &ex {
                    &Expr::Ident(ref i) => {
                        if Self::is_restricted_word(i) {
                            // TODO: double check this
                            if !self.config.tolerant {
                                return self.unexpected_token_error(&start, "restricted ident");
                            }
                        }
                    }
                    _ => (),
                }
            }
            if !self.context.is_assignment_target && !self.config.tolerant {
                return self
                    .unexpected_token_error(&op, "Cannot increment when not at assignment target");
            }
            let prefix = true;
            let ret = UpdateExpr {
                operator,
                argument: Box::new(ex),
                prefix,
            };
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            Ok(Expr::Update(ret))
        } else {
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let expr = self.parse_left_hand_side_expr_allow_call()?;
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            if !self.context.has_line_term && self.look_ahead.token.is_punct() {
                if self.at_punct(Punct::DoublePlus) || self.at_punct(Punct::DoubleDash) {
                    if self.context.strict {
                        match &expr {
                            &Expr::Ident(ref i) => {
                                if Self::is_restricted_word(i) {
                                    return self.expected_token_error(&start, &[]);
                                }
                            }
                            _ => (),
                        }
                    }
                    let op = self.next_item()?;
                    if !self.context.is_assignment_target && !self.config.tolerant {
                        return self.unexpected_token_error(
                            &op,
                            "Cannot increment when not at assignment target",
                        );
                    }
                    self.context.is_assignment_target = false;
                    self.context.is_binding_element = false;
                    let prefix = false;
                    let ret = UpdateExpr {
                        operator: if op.token.matches_punct(Punct::DoublePlus) {
                            UpdateOperator::Increment
                        } else if op.token.matches_punct(Punct::DoubleDash) {
                            UpdateOperator::Decrement
                        } else {
                            return self.expected_token_error(&op, &["++", "--"]);
                        },
                        argument: Box::new(expr),
                        prefix,
                    };
                    return Ok(Expr::Update(ret));
                }
            }
            Ok(expr)
        }
    }

    #[inline]
    fn is_ident(expr: &Expr) -> bool {
        match expr {
            Expr::Ident(_) => true,
            _ => false,
        }
    }

    #[inline]
    fn parse_left_hand_side_expr(&mut self) -> Res<Expr<'b>> {
        if !self.context.allow_in {
            // error
        }
        let mut expr = if self.at_keyword(Keyword::Super) && self.context.in_function_body {
            self.parse_super()?
        } else {
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let ret = if self.at_keyword(Keyword::New) {
                self.parse_new_expr()?
            } else {
                self.parse_primary_expression()?
            };
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            ret
        };
        loop {
            if self.at_punct(Punct::OpenBracket) {
                self.context.is_binding_element = false;
                self.context.is_assignment_target = true;
                self.expect_punct(Punct::OpenBracket)?;
                let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                let prop = self.parse_expression()?;
                self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                self.expect_punct(Punct::CloseBracket)?;
                let member = MemberExpr {
                    computed: true,
                    object: Box::new(expr),
                    property: Box::new(prop),
                };
                expr = Expr::Member(member);
            } else if self.at_punct(Punct::Period) {
                self.context.is_binding_element = false;
                self.context.is_assignment_target = false;
                self.expect_punct(Punct::Period)?;
                let prop = self.parse_ident_name()?;
                let member = MemberExpr {
                    object: Box::new(expr),
                    property: Box::new(Expr::Ident(prop)),
                    computed: false,
                };
                expr = Expr::Member(member);
            } else if self.look_ahead.is_template() {
                let quasi = self.parse_template_literal()?;
                expr = Expr::TaggedTemplate(TaggedTemplateExpr {
                    tag: Box::new(expr),
                    quasi,
                });
            } else {
                break;
            }
        }
        Ok(expr)
    }

    #[inline]
    fn parse_super(&mut self) -> Res<Expr<'b>> {
        self.expect_keyword(Keyword::Super)?;
        if !self.at_punct(Punct::OpenBracket) && !self.at_punct(Punct::Period) {
            return self.expected_token_error(&self.look_ahead, &["[", "."]);
        }
        Ok(Expr::Super)
    }

    #[inline]
    fn parse_left_hand_side_expr_allow_call(&mut self) -> Res<Expr<'b>> {
        debug!(
            "{}: parse_left_hand_side_expr_allow_call, {:?}",
            self.look_ahead.span.start,
            self.look_ahead.token,
        );
        let start_pos = self.look_ahead_position;
        let is_async = self.at_contextual_keyword("async");
        let prev_in = self.context.allow_in;
        self.context.allow_in = true;

        let mut expr = if self.at_keyword(Keyword::Super) && self.context.in_function_body {
            let _ = self.next_item()?;
            if !self.at_punct(Punct::OpenParen)
                && !self.at_punct(Punct::Period)
                && !self.at_punct(Punct::OpenBracket)
            {
                return self.expected_token_error(&self.look_ahead, &["(", ".", "["]);
            }
            Expr::Super
        } else {
            let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
            let ret = if self.at_keyword(Keyword::New) {
                self.parse_new_expr()?
            } else {
                self.parse_primary_expression()?
            };
            self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
            ret
        };
        loop {
            if self.at_punct(Punct::Period) {
                self.context.is_binding_element = false;
                self.context.is_assignment_target = true;
                self.expect_punct(Punct::Period)?;
                let prop = Expr::Ident(self.parse_ident_name()?);
                expr = Expr::Member(MemberExpr {
                    object: Box::new(expr),
                    property: Box::new(prop),
                    computed: false,
                });
            } else if self.at_punct(Punct::OpenParen) {
                let current_pos = self.look_ahead_position;
                let async_arrow = is_async && start_pos.line == current_pos.line;
                self.context.is_binding_element = false;
                self.context.is_assignment_target = false;
                let args = if async_arrow {
                    self.parse_async_args()?
                } else {
                    self.parse_args()?
                };
                //TODO: check for bad import call
                if async_arrow && self.at_punct(Punct::EqualGreaterThan) {
                    let args = args.into_iter().map(|a| FunctionArg::Expr(a)).collect();
                    expr = Expr::ArrowParamPlaceHolder(args, true);
                } else {
                    let inner = CallExpr {
                        callee: Box::new(expr),
                        arguments: args,
                    };
                    expr = Expr::Call(inner);
                }
            } else if self.at_punct(Punct::OpenBracket) {
                self.context.is_assignment_target = true;
                self.context.is_binding_element = false;
                self.expect_punct(Punct::OpenBracket)?;
                let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                let prop = self.parse_expression()?;
                self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                self.expect_punct(Punct::CloseBracket)?;
                let member = MemberExpr {
                    object: Box::new(expr),
                    computed: true,
                    property: Box::new(prop),
                };
                expr = Expr::Member(member);
            } else if self.look_ahead.token.is_template_head() {
                let quasi = self.parse_template_literal()?;
                let temp = TaggedTemplateExpr {
                    tag: Box::new(expr),
                    quasi,
                };
                expr = Expr::TaggedTemplate(temp);
            } else {
                break;
            }
        }
        self.context.allow_in = prev_in;
        Ok(expr)
    }
    /// Parse the arguments of an async function
    #[inline]
    fn parse_async_args(&mut self) -> Res<Vec<Expr<'b>>> {
        debug!("{}: parse_async_args, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenParen)?;
        let mut ret = vec![];
        if !self.at_punct(Punct::CloseParen) {
            loop {
                let arg = if self.at_punct(Punct::Ellipsis) {
                    self.parse_spread_element()?
                } else {
                    let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                    let arg = self.parse_async_arg()?;
                    self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                    arg
                };
                ret.push(arg);
                if self.at_punct(Punct::CloseParen) {
                    break;
                }
                self.expect_comma_sep()?;
                if self.at_punct(Punct::CloseParen) {
                    break;
                }
            }
        }
        self.expect_punct(Punct::CloseParen)?;
        Ok(ret)
    }
    /// Parse an argument of an async function
    /// note: not sure this is needed
    #[inline]
    fn parse_async_arg(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_async_arg, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        let expr = self.parse_assignment_expr()?;
        self.context.first_covert_initialized_name_error = None;
        Ok(expr)
    }
    /// Expect a comma separator,
    /// if parsing with tolerance we can tolerate
    /// a non-existent comma
    fn expect_comma_sep(&mut self) -> Res<()> {
        debug!("{}: expect_comma_sep, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        if self.config.tolerant {
            if self.at_punct(Punct::Comma) {
                let _ = self.next_item()?;
                Ok(())
            } else if self.at_punct(Punct::SemiColon) {
                //tolerate unexpected
                Ok(())
            } else {
                //TODO: double check this
                Ok(())
            }
        } else {
            self.expect_punct(Punct::Comma)
        }
    }

    /// Parse an expression preceded by the `...` operator
    #[inline]
    fn parse_spread_element(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_spread_element, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::Ellipsis)?;
        let (prev_bind, prev_assign, prev_first) = self.inherit_cover_grammar();
        let arg = self.parse_assignment_expr()?;
        self.set_inherit_cover_grammar_state(prev_bind, prev_assign, prev_first);
        Ok(Expr::Spread(Box::new(arg)))
    }
    /// Parse function arguments, expecting to open with `(` and close with `)`
    #[inline]
    fn parse_args(&mut self) -> Res<Vec<Expr<'b>>> {
        debug!("{}: parse_args, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_punct(Punct::OpenParen)?;
        let mut args = vec![];
        if !self.at_punct(Punct::CloseParen) {
            loop {
                let expr = if self.at_punct(Punct::Ellipsis) {
                    self.parse_spread_element()?
                } else {
                    let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
                    let expr = self.parse_assignment_expr()?;
                    self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
                    expr
                };
                args.push(expr);
                if self.at_punct(Punct::CloseParen) {
                    break;
                }
                self.expect_comma_sep()?;
                if self.at_punct(Punct::CloseParen) {
                    break;
                }
            }
        }
        self.expect_punct(Punct::CloseParen)?;
        Ok(args)
    }
    /// This will parse one of two expressions `new Thing()`
    /// or `new.target`. The later is only valid in a function
    /// body
    #[inline]
    fn parse_new_expr(&mut self) -> Res<Expr<'b>> {
        debug!("{}: parse_new_expr, {:?}", self.look_ahead.span.start, self.look_ahead.token);
        self.expect_keyword(Keyword::New)?;
        if self.at_punct(Punct::Period) {
            let _ = self.next_item()?;
            if self.at_contextual_keyword("target") && self.context.in_function_body {
                let property: &'b str = self.parse_ident_name()?;
                Ok(Expr::MetaProperty(MetaProperty {
                    meta: "new",
                    property,
                }))
            } else {
                self.expected_token_error(&self.look_ahead, &["[constructor function call]"])
            }
        } else if self.at_keyword(Keyword::Import) {
            self.expected_token_error(&self.look_ahead, &["not import"])
        } else {
            let (prev_bind, prev_assign, prev_first) = self.isolate_cover_grammar();
            let callee = self.parse_left_hand_side_expr()?;
            self.set_isolate_cover_grammar_state(prev_bind, prev_assign, prev_first)?;
            let args = if self.at_punct(Punct::OpenParen) {
                self.parse_args()?
            } else {
                vec![]
            };
            self.context.is_assignment_target = false;
            self.context.is_binding_element = false;
            let new = NewExpr {
                callee: Box::new(callee),
                arguments: args,
            };

            Ok(Expr::New(new))
        }
    }
    /// Determine the precedence for a specific token,
    /// this will return zero for all tokens except
    /// `instanceOf`, `in`, or binary punctuation
    fn bin_precedence(&self, tok: &Token) -> usize {
        match tok {
            &Token::Punct(ref p) => Self::determine_precedence(*p),
            &Token::Keyword(ref k) => {
                if k == &Keyword::InstanceOf || (self.context.allow_in && k == &Keyword::In) {
                    7
                } else {
                    0
                }
            }
            _ => 0,
        }
    }
    /// Determine the precedence for a specific
    /// punctuation
    fn determine_precedence(p: Punct) -> usize {
        match p {
            Punct::CloseParen
            | Punct::SemiColon
            | Punct::Comma
            | Punct::Equal
            | Punct::CloseBracket => 0,
            Punct::DoublePipe => 1,
            Punct::DoubleAmpersand => 2,
            Punct::Pipe => 3,
            Punct::Caret => 4,
            Punct::Ampersand => 5,
            Punct::DoubleEqual | Punct::BangEqual | Punct::TripleEqual | Punct::BangDoubleEqual => {
                6
            }
            Punct::GreaterThan
            | Punct::LessThan
            | Punct::LessThanEqual
            | Punct::GreaterThanEqual => 7,
            Punct::DoubleLessThan | Punct::DoubleGreaterThan | Punct::TripleGreaterThan => 8,
            Punct::Plus | Punct::Dash => 9,
            Punct::Asterisk | Punct::ForwardSlash | Punct::Percent => 11,
            _ => 0,
        }
    }
    /// Set the state back to the previous state
    /// isolating the previous state
    fn set_isolate_cover_grammar_state(
        &mut self,
        prev_bind: bool,
        prev_assign: bool,
        prev_first: Option<Item<Token<'b>>>,
    ) -> Res<()> {
        if let Some(ref _e) = prev_first {
            //FIXME this needs to do something
            //like an error?
        }
        self.context.is_binding_element = prev_bind;
        self.context.is_assignment_target = prev_assign;
        self.context.first_covert_initialized_name_error = prev_first;
        Ok(())
    }
    /// Get the context state in order to isolate this state from the
    /// following operation
    #[inline]
    fn isolate_cover_grammar(&mut self) -> (bool, bool, Option<Item<Token<'b>>>) {
        let ret = self.get_cover_grammar_state();
        self.context.is_binding_element = true;
        self.context.is_assignment_target = true;
        self.context.first_covert_initialized_name_error = None;
        ret
    }
    /// Get the context state for cover grammar operations
    fn get_cover_grammar_state(&self) -> (bool, bool, Option<Item<Token<'b>>>) {
        (
            self.context.is_binding_element,
            self.context.is_assignment_target,
            self.context.first_covert_initialized_name_error.clone(),
        )
    }
    /// Set the context state to the provided values,
    /// inheriting the previous state
    fn set_inherit_cover_grammar_state(
        &mut self,
        is_binding_element: bool,
        is_assignment: bool,
        first_covert_initialized_name_error: Option<Item<Token<'b>>>,
    ) {
        self.context.is_binding_element = self.context.is_binding_element && is_binding_element;
        self.context.is_assignment_target = self.context.is_assignment_target && is_assignment;
        if first_covert_initialized_name_error.is_some() {
            self.context.first_covert_initialized_name_error = first_covert_initialized_name_error;
        }
    }
    /// Capture the context state for a binding_element, assignment and
    /// first_covert_initialized_name
    fn inherit_cover_grammar(&mut self) -> (bool, bool, Option<Item<Token<'b>>>) {
        trace!("inherit_cover_grammar");
        let ret = self.get_cover_grammar_state();
        self.context.is_binding_element = true;
        self.context.is_assignment_target = true;
        self.context.first_covert_initialized_name_error = None;
        ret
    }
    /// Request the next token from the scanner
    /// swap the last look ahead with this new token
    /// and return the last token
    fn next_item(&mut self) -> Res<Item<Token<'b>>> {
        let mut look_ahead_span = self.look_ahead.span;
        loop {
            self.context.has_line_term = self.scanner.pending_new_line;
            if let Some(look_ahead) = self.scanner.next() {
                let look_ahead = look_ahead.unwrap();
                if cfg!(feature = "debug_look_ahead") {
                    self._look_ahead = format!(
                        "{:?}: {:?}",
                        look_ahead.token,
                        self.scanner.string_for(&look_ahead.span)
                    );
                }
                self.update_positions(look_ahead_span, look_ahead.span.start)?;
                if look_ahead.token.is_comment() {
                    look_ahead_span = look_ahead.span;
                    self.comment_handler.handle_comment(look_ahead);
                    continue;
                }
                let ret = replace(&mut self.look_ahead, look_ahead);
                return Ok(ret);
            } else {
                // if the next item is None, the iterator is spent
                // if the last token was EOF then we want to return that
                // and mark that we have found EOF, if we get here a second
                // time we want to return the ParseAfterEoF error
                if self.look_ahead.token.is_eof() {
                    if self.found_eof {
                        return Err(Error::ParseAfterEoF);
                    } else {
                        self.found_eof = true;
                        return Ok(self.look_ahead.clone());
                    }
                } else {
                    return Err(Error::UnexpectedEoF);
                }
            }
        }
    }

    fn update_positions(&mut self, look_ahead_span: Span, next_look_ahead_start: usize) -> Res<()> {
        // let mut next_idx = self.last_line_idx;
        // while let Some(line) = self.lines.get(next_idx) {
        //     println!("looping for line, line {}-{}, nlas: {}", line.start, line.end, next_look_ahead_start);
        //     if next_look_ahead_start >= line.start && next_look_ahead_start <= line.end {
        //             break;
        //     } else {
        //         next_idx += 1;
        //     }
        // }
        // let old_look_ahead_pos = self.look_ahead_position;
        // self.current_position = old_look_ahead_pos;
        // if next_idx == self.last_line_idx {
        //     self.look_ahead_position.column += next_look_ahead_start.saturating_sub(look_ahead_span.start);
        //     return Ok(())
        // }
        // if self.lines.len() <= next_idx {
        //     self.look_ahead_position.column += 1;
        //     return Ok(());
        // }
        // self.look_ahead_position.line = next_idx + 1;
        // self.look_ahead_position.column = next_look_ahead_start - self.lines[next_idx].start;
        // self.last_line_idx = next_idx;
        // Ok(())

        let prev_text = &self.original[look_ahead_span.start..next_look_ahead_start];
        let whitespace = &self.original[look_ahead_span.end..next_look_ahead_start];

        let old_look_ahead_pos = self.look_ahead_position;
        self.current_position = old_look_ahead_pos;
        let line_counts = prev_text
            .chars()
            .filter(|c| c == &'\n' || c == &'\r' || c == &'\u{2028}' || c == &'\u{2029}')
            .count()
            - whitespace.matches("\r\n").count();
        if line_counts == 0 {
            self.look_ahead_position.column += prev_text.len();
            return Ok(());
        }
        let last_line_len = if let Some(last_line) = prev_text.lines().last() {
            // Lines doesn't include a final empty line if the string ends with a new line
            // we need to make sure we reset the column to 0 if the string ends with a new line
            if let Some(ref c) = prev_text.chars().last() {
                if c == &'\n' || c == &'\r' || c == &'\u{2028}' || c == &'\u{2029}' {
                    0
                } else {
                    last_line.len()
                }
            } else {
                0
            }
        } else {
            0
        };
        self.look_ahead_position.line += line_counts;
        self.look_ahead_position.column = last_line_len;
        Ok(())
    }
    /// Get the next token and validate that it matches
    /// the punct provided, discarding the result
    /// if it does
    #[inline]
    fn expect_punct(&mut self, p: Punct) -> Res<()> {
        let next = self.next_item()?;
        if !next.token.matches_punct(p) {
            return self.expected_token_error(&next, &[&format!("{:?}", p)]);
        }
        Ok(())
    }
    /// move on to the next item and validate it matches
    /// the keyword provided, discarding the result
    /// if it does
    #[inline]
    fn expect_keyword(&mut self, k: Keyword) -> Res<()> {
        let next = self.next_item()?;
        if !next.token.matches_keyword(k) {
            return self.expected_token_error(&next, &[&format!("{:?}", k)]);
        }
        Ok(())
    }
    #[inline]
    fn at_return_arg(&self) -> bool {
        if self.context.has_line_term {
            return self.look_ahead.is_string() || self.look_ahead.is_template();
        }
        !self.at_punct(Punct::SemiColon)
            && !self.at_punct(Punct::CloseBrace)
            && !self.look_ahead.is_eof()
    }
    #[inline]
    fn at_import_call(&mut self) -> bool {
        if self.at_keyword(Keyword::Import) {
            let state = self.scanner.get_state();
            self.scanner.skip_comments().unwrap();
            let ret = if let Some(next) = self.scanner.next() {
                let next = next.unwrap();
                next.token.matches_punct(Punct::OpenParen)
            } else {
                false
            };
            self.scanner.set_state(state);
            ret
        } else {
            false
        }
    }
    #[inline]
    #[inline]
    fn at_qualified_prop_key(&self) -> bool {
        match &self.look_ahead.token {
            Token::Ident(_)
            | Token::String(_)
            | Token::Boolean(_)
            | Token::Null
            | Token::Keyword(_) => true,
            Token::Number(_) => {
                let start = self.look_ahead.span.end + 1;
                if let Some(n) = self.scanner.str_for(&Span {
                    start,
                    end: start + 1,
                }) {
                    n != "n"
                } else {
                    false
                }
            }
            Token::Punct(ref p) => p == &Punct::OpenBracket,
            _ => false,
        }
    }

    /// Lexical declarations require the next token
    /// (not including any comments)
    /// must be an identifier, `let`, `yield`
    /// `{`, or `[`
    #[inline]
    fn at_lexical_decl(&mut self) -> bool {
        let state = self.scanner.get_state();
        self.scanner.skip_comments().unwrap();
        let ret = if let Some(next) = self.scanner.next() {
            let next = next.unwrap();
            next.token.is_ident()
                || next.token.matches_punct(Punct::OpenBracket)
                || next.token.matches_punct(Punct::OpenBrace)
                || next.token.matches_keyword(Keyword::Let)
                || next.token.matches_keyword(Keyword::Yield)
        } else {
            false
        };
        self.scanner.set_state(state);
        ret
    }
    /// Test for if the next token is a specific punct
    #[inline]
    #[inline]
    fn at_punct(&self, p: Punct) -> bool {
        self.look_ahead.token.matches_punct(p)
    }
    /// Test for if the next token is a specific keyword
    #[inline]
    #[inline]
    fn at_keyword(&self, k: Keyword) -> bool {
        self.look_ahead.token.matches_keyword(k)
    }
    /// This test is for all the operators that might be part
    /// of an assignment statement
    #[inline]
    #[inline]
    fn at_assign(&self) -> bool {
        self.look_ahead.token.matches_punct(Punct::Equal)
            || self.look_ahead.token.matches_punct(Punct::AsteriskEqual)
            || self
                .look_ahead
                .token
                .matches_punct(Punct::DoubleAsteriskEqual)
            || self
                .look_ahead
                .token
                .matches_punct(Punct::ForwardSlashEqual)
            || self.look_ahead.token.matches_punct(Punct::PercentEqual)
            || self.look_ahead.token.matches_punct(Punct::PlusEqual)
            || self.look_ahead.token.matches_punct(Punct::DashEqual)
            || self
                .look_ahead
                .token
                .matches_punct(Punct::DoubleLessThanEqual)
            || self
                .look_ahead
                .token
                .matches_punct(Punct::DoubleGreaterThanEqual)
            || self
                .look_ahead
                .token
                .matches_punct(Punct::TripleGreaterThanEqual)
            || self.look_ahead.token.matches_punct(Punct::PipeEqual)
            || self.look_ahead.token.matches_punct(Punct::CaretEqual)
            || self.look_ahead.token.matches_punct(Punct::AmpersandEqual)
    }
    /// The keyword `async` is conditional, that means to decided
    /// if we are actually at an async function we need to check the
    /// next token would need to be on the same line
    #[inline]
    #[inline]
    fn at_async_function(&mut self) -> bool {
        if self.at_contextual_keyword("async") {
            self.scanner.pending_new_line
                && if let Some(peek) = self.scanner.look_ahead() {
                    if let Ok(peek) = peek {
                        peek.token.matches_keyword(Keyword::Function)
                    } else {
                        false
                    }
                } else {
                    false
                }
        } else {
            false
        }
    }
    /// Since semi-colons are options, this function will
    /// check the next token, if it is a semi-colon it will
    /// consume it otherwise we need to either be at a line terminator
    /// EoF or a close brace
    #[inline]
    fn consume_semicolon(&mut self) -> Res<()> {
        if self.at_punct(Punct::SemiColon) {
            let _semi = self.next_item()?;
        } else if !self.context.has_line_term {
            if !self.look_ahead.token.is_eof() && !self.at_punct(Punct::CloseBrace) {
                return self.expected_token_error(&self.look_ahead, &["`;`", "`eof`", "`}`"]);
            }
        }
        Ok(())
    }
    /// Tests if a token matches an &str that might represent
    /// a contextual keyword like `async`
    #[inline]
    fn at_contextual_keyword(&self, s: &str) -> bool {
        if let Some(current) = self.scanner.str_for(&self.look_ahead.span) {
            current == s
        } else {
            false
        }
    }
    /// Sort of keywords `eval` and `arguments` have
    /// a special meaning and will cause problems
    /// if used in the wrong scope
    #[inline]
    fn is_restricted_word(word: &str) -> bool {
        word == "eval" || word == "arguments"
    }
    /// Check if this &str is in the list of reserved
    /// words in the context of 'use strict'
    #[inline]
    fn is_strict_reserved(word: &str) -> bool {
        word == "implements"
            || word == "interface"
            || word == "package"
            || word == "private"
            || word == "protected"
            || word == "public"
            || word == "static"
            || word == "yield"
            || word == "let"
    }
    /// Tests if the parser is currently at the
    /// start of an expression. This consists of a
    /// subset of punctuation, keywords or a regex literal
    #[inline]
    fn is_start_of_expr(&self) -> bool {
        let mut ret = true;
        let token = &self.look_ahead.token;
        if token.is_punct() {
            ret = token.matches_punct(Punct::OpenBracket)
                || token.matches_punct(Punct::OpenParen)
                || token.matches_punct(Punct::OpenBracket)
                || token.matches_punct(Punct::Plus)
                || token.matches_punct(Punct::Dash)
                || token.matches_punct(Punct::Bang)
                || token.matches_punct(Punct::Tilde)
                || token.matches_punct(Punct::DoublePlus)
                || token.matches_punct(Punct::DoubleDash)
        }
        if token.is_keyword() {
            ret = token.matches_keyword(Keyword::Class)
                || token.matches_keyword(Keyword::Delete)
                || token.matches_keyword(Keyword::Function)
                || token.matches_keyword(Keyword::Let)
                || token.matches_keyword(Keyword::New)
                || token.matches_keyword(Keyword::Super)
                || token.matches_keyword(Keyword::This)
                || token.matches_keyword(Keyword::TypeOf)
                || token.matches_keyword(Keyword::Void)
                || token.matches_keyword(Keyword::Yield)
        }
        if token.is_regex() {
            ret = true;
        }
        ret
    }
    #[inline]
    fn at_big_int_flag(&self) -> bool {
        let Span { start, end } = self.look_ahead.span;
        &self.original[start..end] == "n"
    }

    fn get_string(&self, span: &Span) -> Res<&'b str> {
        self.scanner
            .str_for(span)
            .ok_or_else(|| self.op_error("Unable to get &str from scanner"))
    }
    /// performs a binary search of the list of lines to determine
    /// which line the item exists within and calculates the relative
    /// column
    pub(crate) fn get_item_position(&self, item: &Item<Token<'b>>) -> Position {
        // This inner recursive function is required because we need
        // to keep track of the index or the line number, the only
        // way to get that would be to call .iter on the lines property
        // which would perform a linear search, this allows for a binary
        // search
        fn search<'c>(lines: &[Line], item: &Item<Token<'c>>, index: usize) -> (usize, Line) {
            let current_len = lines.len();
            if current_len == 1 {
                (index, lines[0])
            } else {
                let half = current_len >> 1;
                if lines[half - 1].end > item.span.start {
                    search(&lines[..half], item, index)
                } else {
                    search(&lines[half..], item, index + half)
                }
            }
        }
        let (idx, line) = search(&self.lines, item, 0);
        let column = item.span.start.saturating_sub(line.start);
        Position {
            line: idx + 1,
            column,
        }
    }

    fn expected_token_error<T>(&self, item: &Item<Token<'b>>, expectation: &[&str]) -> Res<T> {
        let bt = backtrace::Backtrace::new();
        error!("{:?}", bt);
        let pos = self.get_item_position(item);
        let expectation = expectation
            .iter()
            .enumerate()
            .map(|(i, s)| {
                if i == expectation.len() - 1 && expectation.len() > 1 {
                    format!("or `{}`", s)
                } else {
                    format!("`{}`", s)
                }
            })
            .collect::<Vec<String>>()
            .join(", ");
        Err(Error::UnexpectedToken(
            pos,
            format!("Expected {}; found {:?}", expectation, item.token),
        ))
    }
    fn unexpected_token_error<T>(&self, item: &Item<Token<'b>>, msg: &str) -> Res<T> {
        let bt = backtrace::Backtrace::new();
        error!("{:?}", bt);
        let pos = self.get_item_position(item);

        let name = self.scanner.string_for(&item.span).unwrap_or(String::new());
        Err(Error::UnexpectedToken(
            pos,
            format!("Found unexpected token: {}; {}", name, msg),
        ))
    }
    fn tolerate_error(&self, err: Error) -> Result<(), Error> {
        if !self.config.tolerant {
            let bt = backtrace::Backtrace::new();
            error!("{:?}", bt);
            Err(err)
        } else {
            Ok(())
        }
    }
    fn op_error(&self, msg: &str) -> Error {
        Error::OperationError(self.current_position, msg.to_owned())
    }
    fn redecl_error(&self, name: &str) -> Error {
        Error::Redecl(self.current_position, name.to_owned())
    }
    fn reinterpret_error(&self, from: &str, to: &str) -> Error {
        Error::UnableToReinterpret(self.current_position, from.to_owned(), to.to_owned())
    }

    fn next_part(&mut self) -> Res<ProgramPart<'b>> {
        if !self.context.past_prolog {
            if self.look_ahead.is_string() {
                let next_part = match self.parse_directive() {
                    Ok(n) => n,
                    Err(e) => {
                        self.context.errored = true;
                        return Err(e);
                    }
                };
                self.context.past_prolog = match &next_part {
                    ProgramPart::Dir(_) => false,
                    _ => true,
                };
                return Ok(next_part);
            } else {
                self.context.past_prolog = true;
            }
        }
        let ret = match self.parse_statement_list_item() {
            Ok(p) => p,
            Err(e) => {
                self.context.errored = true;
                return Err(e);
            }
        };
        trace!(target: "resp:trace", "{:?}", ret);
        Ok(ret)
    }
}

impl<'b, CH> Iterator for Parser<'b, CH>
where
    CH: CommentHandler<'b> + Sized,
{
    type Item = Res<ProgramPart<'b>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.look_ahead.token.is_eof() || self.context.errored {
            None
        } else {
            Some(self.next_part())
        }
    }
}

#[allow(unused)]
struct FormalParams<'a> {
    simple: bool,
    params: Vec<FunctionArg<'a>>,
    strict: bool,
    found_restricted: bool,
}

#[allow(unused)]
struct CoverFormalListOptions<'a> {
    simple: bool,
    params: Vec<FunctionArg<'a>>,
    stricted: bool,
    first_restricted: Option<Expr<'a>>,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn split_lines() {
        println!("line feed");
        let lf = "lf:0
0;";
        let lines = get_lines(lf);
        let expectation = vec![Line { start: 0, end: 4 }, Line { start: 5, end: 6 }];
        assert_eq!(lines, expectation);
        println!("carriage return");
        let cr = format!("cr:0{}0;", '\r');
        let lines = get_lines(&cr);
        let expectation = vec![Line { start: 0, end: 4 }, Line { start: 5, end: 6 }];
        assert_eq!(lines, expectation);
        println!("carriage return line feed");
        let crlf = format!("crlf:0{}{}0;", '\r', '\n');
        let lines = get_lines(&crlf);
        let expectation = vec![Line { start: 0, end: 7 }, Line { start: 8, end: 9 }];
        assert_eq!(lines, expectation);
        println!("line seperator");
        let ls = "ls:0 0;";
        let lines = get_lines(ls);
        let expectation = vec![Line { start: 0, end: 6 }, Line { start: 7, end: 8 }];
        assert_eq!(lines, expectation);
        println!("paragraph separator");
        let ps = "ps:0 0;";
        let lines = get_lines(ps);
        let expectation = vec![Line { start: 0, end: 6 }, Line { start: 7, end: 8 }];
        assert_eq!(lines, expectation);
    }
    #[test]
    fn position_for_item() {
        let js = "function thing() {
    return 'stuff';
}";
        let item1 = Item {
            span: Span { start: 9, end: 13 },
            token: Token::Ident("thing".into()),
        };
        let position1 = Position { line: 1, column: 9 };
        let p = Parser::new(js).unwrap();
        assert_eq!(p.get_item_position(&item1), position1);
        let item2 = Item {
            span: Span { start: 30, end: 36 },
            token: Token::String(ress::StringLit::single("stuff")),
        };
        let position2 = Position {
            line: 2,
            column: 11,
        };
        assert_eq!(p.get_item_position(&item2), position2);
        let item3 = Item {
            span: Span { start: 39, end: 40 },
            token: Token::Punct(ress::Punct::CloseBrace),
        };
        let position3 = Position { line: 3, column: 0 };
        assert_eq!(p.get_item_position(&item3), position3);
    }
}
