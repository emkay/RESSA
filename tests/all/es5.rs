use resast::ref_tree::prelude::*;

lazy_static! {
    pub static ref ES5: Vec<ProgramPart<'static>> =
        vec![
            labeled_statement("tab"),
            labeled_statement("verticalTab"),
            labeled_statement("formFeed"),
            labeled_statement("space"),
            labeled_statement("nbsp"),
            labeled_statement("bom"),
            line_term("lineFeed"),
            number_literal_part("0"),
            line_term("carriageReturn"),
            number_literal_part("0"),
            line_term("carriageReturnLineFeed"),
            number_literal_part("0"),
            line_term("lineSeparator"),
            number_literal_part("0"),
            line_term("paragraphSeparator"),
            number_literal_part("0"),
            var_decl_one(),
            var_decl_two(),
            null_literal(),
            bool_literal(true),
            bool_literal(false),
            number_literal_part("0"),
            number_literal_part("00"),
            number_literal_part("1234567890"),
            number_literal_part("01234567"),
            number_literal_part("0."),
            number_literal_part("0.00"),
            number_literal_part("10.00"),
            number_literal_part(".0"),
            number_literal_part(".00"),
            number_literal_part("0e0"),
            number_literal_part("0E0"),
            number_literal_part("0.e0"),
            number_literal_part("0.00e+0"),
            number_literal_part(".00e-0"),
            number_literal_part("0x0"),
            number_literal_part("0X0"),
            number_literal_part("0x0123456789abcdefABCDEF"),
            number_literal_part("2e308"),
            string_literal(r#""""#,),
            string_literal(r#""'""#,),
            string_literal(r#""\'\"\\\b\f\n\r\t\v\0""#,),
            string_literal(r#""\1\00\400\000""#,),
            string_literal(r#""\x01\x23\x45\x67\x89\xAB\xCD\xEF""#,),
            string_literal(r#""\u0123\u4567\u89AB\uCDEF""#,),
            string_literal(r#""\
""#),
            string_literal(r"''"),
            string_literal(r#"'"'"#),
            string_literal(r#"'\'\"\\\b\f\n\r\t\v\0'"#),
            string_literal(r#"'\1\00\400\000'"#),
            string_literal(r#"'\x01\x23\x45\x67\x89\xAB\xCD\xEF'"#),
            string_literal(r#"'\u0123\u4567\u89AB\uCDEF'"#),
            string_literal(r#"'\
'"#),
            regex_literal_part(r#"x"#, ""),
            regex_literal_part(r#"|"#, ""),
            regex_literal_part(r#"|||"#, ""),
            regex_literal_part(r#"^$\b\B"#, ""),
            regex_literal_part(r#"(?=(?!(?:(.))))"#, ""),
            regex_literal_part(r#"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000"#,""),
            regex_literal_part(r#"\d\D\s\S\w\W"#, ""),
            regex_literal_part(r#"\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz"#, ""),
            regex_literal_part(r#"\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ"#, ""),
            regex_literal_part(r#"[a-z-]"#,""),
            regex_literal_part(r#"[^\b\-^]"#,""),
            regex_literal_part(r#"[/\]\\]"#, ""),
            regex_literal_part(r#"."#, "i"),
            regex_literal_part(r#"."#, "g"),
            regex_literal_part(r#"."#, "m"),
            regex_literal_part(r#"."#, "igm"),
            regex_literal_part(r#".*"#, ""),
            regex_literal_part(r#".*?"#, ""),
            regex_literal_part(r#".+"#, ""),
            regex_literal_part(r#".+?"#, ""),
            regex_literal_part(r#".?"#, ""),
            regex_literal_part(r#".??"#, ""),
            regex_literal_part(r#".{0}"#, ""),
            regex_literal_part(r#".{0,}"#, ""),
            regex_literal_part(r#".{0,0}"#, ""),
            this_stmt(),
            ident_stmt("x"),
            array(vec![]),
            // TODO: Double Check this
            array(vec![None]),
            array(vec![Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0"))]),
            array(vec![None, Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0")), None, Some(number_literal_expr("0"))]),
            array(vec![None, None]),
            //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            
        ];
}

fn labeled_statement(label: &'static str) -> ProgramPart {
    ProgramPart::Stmt(
        Stmt::Labeled(
            LabeledStmt {
                label,
                body: Box::new(
                    Stmt::For(
                        ForStmt {
                            init: None,
                            test: None,
                            update: None,
                            body: Box::new(
                                Stmt::Break(
                                    Some(label)
                                )
                            )
                        }
                    )
                )
            }
        )
    )
}

fn line_term(label: &str) -> ProgramPart {
    ProgramPart::Stmt(
        Stmt::Labeled(
            LabeledStmt {
                label,
                body: Box::new(
                    Stmt::Expr(
                        Expr::Literal(
                            Literal::Number("0")
                        )
                    )
                )
            }
        )
    )
}

fn number_literal_part(number: &'static str) -> ProgramPart {
    ProgramPart::Stmt(
        Stmt::Expr(
            number_literal_expr(number)
        )
    )
}

fn number_literal_expr(number: &'static str) -> Expr<'static> {
    Expr::Literal(
        Literal::Number(number)
    )
}

fn null_literal() -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::Literal(
                Literal::Null
            )
        )
    )
}

fn bool_literal(b: bool) -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::Literal(
                Literal::Boolean(
                    b
                )
            )
        )
    )
}

fn var_decl_one() -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Var(
            var_decls(&[
                r"$", 
                r"_", 
                r"\u0078", 
                r"x$", 
                r"x_", 
                r"x\u0030", 
                r"xa", 
                r"x0", 
                r"x0a", 
                r"x0123456789",
                r"qwertyuiopasdfghjklzxcvbnm", 
                r"QWERTYUIOPASDFGHJKLZXCVBNM",
            ])      
        )
    )
}

fn var_decl_two() -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Var(
            var_decls(&[
                r"œ一", 
                r"ǻ둘", 
                r"ɤ〩", 
                r"φ", 
                r"ﬁⅷ", 
                r"ユニコード", 
                r"x‌‍",
            ])
        )
    )
}

fn var_decls(decls: &[&'static str]) -> Vec<VariableDecl<'static>> {
    decls.iter().map(|s| var_decl(*s)).collect()
}

fn var_decl(id: &'static str) -> VariableDecl {
    VariableDecl {
        id: Pat::Identifier(id),
        init: None,
    }
}

fn string_literal(s: &'static str) -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::Literal(
                Literal::String(
                    s
                )
            )
        )
    )
}

fn regex_literal_part(pattern: &'static str, flags: &'static str) -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::Literal(
                Literal::RegEx(
                    RegEx {
                        pattern,
                        flags
                    }
                )
            )
        )
    )
}

fn this_stmt() -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::This
        )
    )
}

fn ident_stmt(id: &'static str) -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::Ident(id)
        )
    )
}

fn array(content: Vec<Option<Expr<'static>>>) -> ProgramPart<'static> {
    ProgramPart::Stmt(
        Stmt::Expr(
            Expr::Array(
                content
            )
        )
    )
}
