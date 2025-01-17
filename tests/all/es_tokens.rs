use resast::decl::VariableKind;
use resast::expr::{
    AssignmentOperator, BinaryOperator, LogicalOperator, PropertyKind, UnaryOperator,
    UpdateOperator,
};
use resast::ref_tree::prelude::*;
use resast::ref_tree::stmt::CatchClause;
type Part = ProgramPart<'static>;
type S = Stmt<'static>;
type E = Expr<'static>;
type L = Literal<'static>;
type OP = ObjectProperty<'static>;
lazy_static! {
    pub static ref ES5: Vec<ProgramPart<'static>> =
        vec![
            preamble_labeled_statement("tab"),
            preamble_labeled_statement("verticalTab"),
            preamble_labeled_statement("formFeed"),
            preamble_labeled_statement("space"),
            preamble_labeled_statement("nbsp"),
            preamble_labeled_statement("bom"),
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
            null_literal_part(),
            bool_literal_part(true),
            bool_literal_part(false),
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
            string_literal_part(r#""""#,),
            string_literal_part(r#""'""#,),
            string_literal_part(r#""\'\"\\\b\f\n\r\t\v\0""#,),
            string_literal_part(r#""\1\00\400\000""#,),
            string_literal_part(r#""\x01\x23\x45\x67\x89\xAB\xCD\xEF""#,),
            string_literal_part(r#""\u0123\u4567\u89AB\uCDEF""#,),
            string_literal_part(r#""\
""#),
            string_literal_part(r"''"),//20
            string_literal_part(r#"'"'"#),
            string_literal_part(r#"'\'\"\\\b\f\n\r\t\v\0'"#),
            string_literal_part(r#"'\1\00\400\000'"#),
            string_literal_part(r#"'\x01\x23\x45\x67\x89\xAB\xCD\xEF'"#),
            string_literal_part(r#"'\u0123\u4567\u89AB\uCDEF'"#),
            string_literal_part(r#"'\
'"#),
            regex_literal_part(r#"x"#, ""),
            regex_literal_part(r#"|"#, ""),
            regex_literal_part(r#"|||"#, ""),
            regex_literal_part(r#"^$\b\B"#, ""), //30
            regex_literal_part(r#"(?=(?!(?:(.))))"#, ""),
            regex_literal_part(r#"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000"#,""),
            regex_literal_part(r#"\d\D\s\S\w\W"#, ""),
            regex_literal_part(r#"\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz"#, ""),
            regex_literal_part(r#"\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ"#, ""),
            regex_literal_part(r#"[a-z-]"#,""),
            regex_literal_part(r#"[^\b\-^]"#,""),
            regex_literal_part(r#"[/\]\\]"#, ""),
            regex_literal_part(r#"."#, "i"),
            regex_literal_part(r#"."#, "g"),//40
            regex_literal_part(r#"."#, "m"),
            regex_literal_part(r#"."#, "igm"),
            regex_literal_part(r#".*"#, ""),
            regex_literal_part(r#".*?"#, ""),
            regex_literal_part(r#".+"#, ""),
            regex_literal_part(r#".+?"#, ""),
            regex_literal_part(r#".?"#, ""),
            regex_literal_part(r#".??"#, ""),
            regex_literal_part(r#".{0}"#, ""),
            regex_literal_part(r#".{0,}"#, ""), //50
            regex_literal_part(r#".{0,0}"#, ""),
            this_part(),
            ident_stmt("x"),
            array(vec![]),
            // TODO: Double Check this
            array(vec![None]),
            array(vec![Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0"))]),
            array(vec![None, Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]),
            array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]), //60
            array(vec![Some(number_literal_expr("0")), None, Some(number_literal_expr("0"))]),
            array(vec![None, None]),
            //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
            obj_literal_part(vec![]),
            obj_literal_part(vec![
                obj_prop_ident_number("x", "0")
            ]),
            obj_literal_part(vec![
                obj_prop_ident_number("x", "0"),
                obj_prop_ident_number("y", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_ident_number("x", "0")
            ]),
            obj_literal_part(vec![
                obj_prop_str_number("'x'", "0"),
                obj_prop_str_number("\"y\"", "0"),
                obj_prop_ident_number("var", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_number_number("0", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_number_number("0.", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_number_number("0.0", "0"), // 80
            ]),
            obj_literal_part(vec![
                obj_prop_number_number(".0", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_number_number("0e0", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_number_number("0x0", "0"),
            ]),
            obj_literal_part(vec![
                obj_prop_ident_getter("x"),
                obj_prop_ident_setter("x", vec![
                    fn_arg_ident_pat("a")
                ]),
                obj_prop_str_getter("'y'"),
                obj_prop_str_setter("\"y\"", vec![
                    fn_arg_ident_pat("a")
                ]),
                obj_prop_number_getter("0"),
                obj_prop_number_setter("0", vec![
                    fn_arg_ident_pat("a")
                ]),
                obj_prop_ident_getter("var"),
                obj_prop_ident_setter("var", vec![
                    fn_arg_ident_pat("x")
                ]),
            ]),
            member_number_ident_part("0.", "a"),
            member_number_number_part("0", "0"),
            assign_part(
                assign_left_ident("x"),
                func_expr(
                    "f", vec![], vec![
                        return_ident_part("f")
                    ]
                )
            ),
            assign_part(
                assign_left_expr(
                    member_ident_number_expr("x", "0")
                ),
                Expr::Ident("x")
            ),
            assign_part(
                assign_left_expr(
                    member_ident_ident_expr("x", "a")
                ),
                Expr::Ident("x")
            ),
            new_ident_part("x"),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                Expr::Member(member_ident_number("x", "0")),
                vec![]
            ),
            new_part(
                Expr::Member(member_ident_ident("x", "a")),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        Expr::Member(
                            member_ident_number("x", "0")
                        ),
                        Expr::Ident("a"),
                        false
                    )
                ),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        Expr::Member(
                            member_ident_ident("x", "a")
                        ),
                        number_literal_expr("0"),
                        true,
                    )
                ),
                vec![]
            ),
            new_ident_part("x"),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        new_ident_expr("x", vec![]),
                        Expr::Ident("a"),
                        false
                    )
                ),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        new_ident_expr("x", vec![]),
                        number_literal_expr("0"),
                        true,
                    )
                ),
                vec![]
            ),
            call_ident_part("x", vec![]), //100
            call_part(
                call_ident_expr("x", vec![]),
                vec![],
            ),
            call_ident_part(
                "x", vec![
                    Expr::Ident("x")
                ]
            ),
            call_ident_part(
                "x", vec![
                    Expr::Ident("x"),
                    Expr::Ident("x"),
                ]
            ),
            call_part(
                member_expr(
                    call_expr(
                        member_ident_ident_expr(
                            "x", "a"
                        ),
                        vec![]
                    ),
                    Expr::Ident("a"),
                    false,
                ),
                vec![]
            ),
            call_part(
                member_expr(
                    call_expr(
                        member_ident_number_expr("x", "0"),
                        vec![],
                    ),
                    number_literal_expr("0"),
                    true
                ),
                vec![]
            ),
            call_part(
                member_expr(
                    member_expr(
                        call_expr(
                            Expr::Ident("x"),
                            vec![]
                        ),
                        Expr::Ident("a"),
                        false,
                    ),
                    number_literal_expr("0"),
                    true,
                )
                , vec![]
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Increment,
                false
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Decrement,
                false
            ),
            unary_part(
                UnaryOperator::Delete,
                unary_expr(
                    UnaryOperator::Void,
                    unary_expr(
                        UnaryOperator::TypeOf,
                        unary_expr(
                            UnaryOperator::Plus,
                            unary_expr(
                                UnaryOperator::Minus,
                                unary_expr(
                                    UnaryOperator::Tilde,
                                    unary_expr(
                                        UnaryOperator::Not,
                                        Expr::Ident("x"),
                                        true,
                                    ),
                                    true,
                                ),
                                true,
                            ),
                            true,
                        ),
                        true,
                    ),
                    true,
                ),
                true
            ),
            update_part( //110
                Expr::Ident("x"),
                UpdateOperator::Increment,
                true
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Decrement,
                true
            ),
            zero_bin_zero_part(BinaryOperator::Times),
            zero_bin_zero_part(BinaryOperator::Over),
            zero_bin_zero_part(BinaryOperator::Mod),
            zero_bin_zero_part(BinaryOperator::Plus),
            zero_bin_zero_part(BinaryOperator::Minus),
            zero_bin_zero_part(BinaryOperator::LeftShift),
            zero_bin_zero_part(BinaryOperator::RightShift),
            zero_bin_zero_part(BinaryOperator::UnsignedRightShift),
            zero_bin_zero_part(BinaryOperator::LessThan), //120
            zero_bin_zero_part(BinaryOperator::GreaterThan),
            zero_bin_zero_part(BinaryOperator::LessThanEqual),
            zero_bin_zero_part(BinaryOperator::GreaterThanEqual),
            zero_bin_expr_part(
                BinaryOperator::InstanceOf,
                Expr::Function(empty_anon_fn(vec![])),
            ),
            zero_bin_expr_part(
                BinaryOperator::In,
                obj_literal_expr(vec![]),
            ),
            zero_bin_zero_part(BinaryOperator::Equal),
            zero_bin_zero_part(BinaryOperator::NotEqual),
            zero_bin_zero_part(BinaryOperator::StrictEqual),
            zero_bin_zero_part(BinaryOperator::StrictNotEqual),
            zero_bin_zero_part(BinaryOperator::And),//130
            zero_bin_zero_part(BinaryOperator::XOr),
            zero_bin_zero_part(BinaryOperator::Or),
            zero_log_zero(LogicalOperator::And),
            zero_log_zero(LogicalOperator::Or),
            conditional_part(
                number_literal_expr("0"),
                number_literal_expr("0"),
                number_literal_expr("0"),
            ),
            // TODO: Validate this
            conditional_part(
                number_literal_expr("0"),
                conditional_expr(
                    number_literal_expr("0"),
                    number_literal_expr("0"),
                    number_literal_expr("0"),
                ),
                number_literal_expr("0"),
            ),
            conditional_part(
                zero_log_zero_expr(LogicalOperator::Or),
                assign_expr(
                    assign_left_ident("x"),
                    number_literal_expr("0"),
                ),
                assign_expr(
                    assign_left_ident("x"),
                    number_literal_expr("0"),
                ),
            ),
            // ^^^^^^^^^^^^^^^^^^^^^^^^^^
            ident_assign_zero_part("x", AssignmentOperator::Equal),
            ident_assign_zero_part("x", AssignmentOperator::TimesEqual),//140
            ident_assign_zero_part("x", AssignmentOperator::DivEqual),
            ident_assign_zero_part("x", AssignmentOperator::ModEqual),
            ident_assign_zero_part("x", AssignmentOperator::PlusEqual),
            ident_assign_zero_part("x", AssignmentOperator::MinusEqual),
            ident_assign_zero_part("x", AssignmentOperator::LeftShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::RightShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::UnsignedRightShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::AndEqual),
            ident_assign_zero_part("x", AssignmentOperator::XOrEqual),
            ident_assign_zero_part("x", AssignmentOperator::OrEqual), //150
            zero_sequence(2),
            zero_sequence(3),
            sequence(vec![
                ident_assign_zero_expr("x", AssignmentOperator::Equal),
                ident_assign_zero_expr("x", AssignmentOperator::Equal),
            ]),
            block_part(vec![]),
            block_part(vec![ProgramPart::Stmt(Stmt::Empty)]),
            block_part(vec![number_literal_part("0")]),
            block_part(vec![number_literal_part("0")]),
            block_part(vec![
                number_literal_part("0"),
                number_literal_part("0"),
            ]),
            block_part(vec![
                number_literal_part("0"),
                number_literal_part("0"),
            ]),
            variable_decl_part(
                VariableKind::Var,
                vec![
                    variable_decl_none("x") //160
                ]
            ),
            variable_decl_part(
                VariableKind::Var,
                vec![
                variable_decl_none("x"),
                variable_decl_none("y"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_none("x"),
                variable_decl_none("y"),
                variable_decl_none("z"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x"),
                variable_decl_none("y"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_none("x"),
                variable_decl_zero("y"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x"),
                variable_decl_zero("y"),
            ]),
            ProgramPart::Stmt(Stmt::Empty),
            if_zero_empty(),
            if_zero_empty_else(),
            do_while_zero(),//170
            while_zero(),
            for_exprs_part(None, None, None, Stmt::Break(None)),
            for_exprs_part(
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Stmt::Empty,
            ),
            for_exprs_part(
                Some(zero_bin_expr_expr(
                    BinaryOperator::In,
                    array_expr(vec![]),
                )),
                Some(number_literal_expr("0")),
                None,
                Stmt::Empty,
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                var_decl("a")
                ],
                None,
                None,
                Stmt::Break(None),
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                var_decl("a"),
                var_decl("b"),
                ],
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Stmt::Empty,
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                variable_decl_zero("a")
                ],
                None,
                None,
                Stmt::Break(None),
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                    VariableDecl {
                        id: Pat::Identifier("a"),
                        init: Some(
                            zero_bin_expr_expr(
                                BinaryOperator::In,
                                array_expr(vec![])
                            )
                        )
                    }
                ],
                Some(number_literal_expr("0")),
                None,
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Expr(Expr::Ident("x")),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Variable(
                    VariableKind::Var,
                    var_decl("x")
                ),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Variable(
                    VariableKind::Var,
                    var_decl_init("x", array_expr(vec![]))
                ),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Variable(
                    VariableKind::Var,
                    var_decl_init(
                        "x",
                        zero_bin_expr_expr(BinaryOperator::In, array_expr(vec![]))
                    )
                ),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            for_exprs_part(None, Some(number_literal_expr("0")), None, Stmt::Continue(None)),
            labeled_statement_continue("x"),
            for_exprs_part(None, None, None, Stmt::Break(None)),
            preamble_labeled_statement("x"),
            switch_zero_part(vec![
                case_zero(vec![ProgramPart::Stmt(Stmt::Break(None))])
            ]),
            func_part(
                "f",
                vec![],
                vec![ProgramPart::Stmt(Stmt::Return(None))]
            ),
            func_part(
                "f",
                vec![],
                vec![ProgramPart::Stmt(Stmt::Return(
                    Some(number_literal_expr("0"))
                ))]
            ),
            ProgramPart::Stmt(Stmt::With(WithStmt { object: number_literal_expr("0"), body: Box::new(Stmt::Empty)})),
            switch_zero_part(vec![]),
            switch_zero_part(vec![
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![]),
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                default_case(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![]),
                default_case(vec![]),
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![
                    empty_part(),
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                default_case(vec![
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                default_case(vec![
                    empty_part(),
                    empty_part(),
                ]),
            ]),
            labeled_part("x", Stmt::Empty),
            labeled_part(
                "x",
                labeled_stmt("y", Stmt::Empty)
            ),
            try_part(
                vec![
                    throw_part(
                        number_literal_expr("0")
                    )
                ],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                None
            ),
            try_part(
                vec![],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                None
            ),
            try_part(
                vec![],
                None,
                Some(vec![])
            ),
            try_part(
                vec![],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                Some(vec![])
            ),
            ProgramPart::Stmt(Stmt::Debugger),
            func_part("f", vec![], vec![]),
            func_part(
                "f",
                vec![
                    fn_arg_ident_pat("x"),
                ],
                vec![]
            ),
            func_part(
                "f",
                vec![
                    fn_arg_ident_pat("x"),
                    fn_arg_ident_pat("y"),
                ],
                vec![]
            ),
            func_part(
                "f",
                vec![],
                vec![
                    func_part(
                        "f",
                        vec![],
                        vec![]
                    ),
                ]
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("\"use strict\""),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("'use strict'"),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("\"other directive\""),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("'other directive'"),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    string_literal_part("\"string\""),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    string_literal_part("'string'"),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    ProgramPart::Stmt(
                        Stmt::Expr(
                            Expr::Binary(
                                binary(
                                    string_literal_expr("'string'"),
                                    BinaryOperator::Plus,
                                    number_literal_expr("0")
                                )
                            )
                        )
                    ),
                ],
            ),
            func_expr_part(None, vec![], vec![]),
            func_expr_part(None, vec![
                fn_arg_ident_pat("x")
            ], vec![]),
            func_expr_part(None, vec![
                fn_arg_ident_pat("x"),
                fn_arg_ident_pat("y"),
            ], vec![]),
            func_expr_part(None, vec![], vec![
                func_part("f", vec![], vec![]),
            ]),
            func_expr_part(Some("f"), vec![], vec![]),
            func_expr_part(Some("f"), vec![
                fn_arg_ident_pat("x")
            ], vec![]),
            func_expr_part(Some("f"), vec![
                fn_arg_ident_pat("x"),
                fn_arg_ident_pat("y"),
            ], vec![]),
            func_expr_part(Some("f"), vec![], vec![
                func_part("f", vec![], vec![]),
            ]),
        ];
}

lazy_static! {
    pub static ref ES2015: Vec<ProgramPart<'static>> = vec![
        preamble_labeled_statement("tab"),
        preamble_labeled_statement("verticalTab"),
        preamble_labeled_statement("formFeed"),
        preamble_labeled_statement("space"),
        preamble_labeled_statement("nbsp"),
        preamble_labeled_statement("bom"),
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
        var_decl_one_2015(),
        var_decl_two_2015(),
        variable_decl_part(
            VariableKind::Var,
            vec![var_decl("yield")]
        ),
        variable_decl_part(
            VariableKind::Let,
            vec![var_decl("letx")],
        ),
        variable_decl_part(
            VariableKind::Let,
            vec![
                VariableDecl {
                    id: Pat::Array(
                        vec![
                            Some(
                                ArrayPatPart::Pat(
                                    Pat::Identifier(r"x\u0078")
                                )
                            )
                        ]
                    ),
                    init: Some(number_literal_expr("0"))
                }
            ]
        ),
        variable_decl_part(
            VariableKind::Const,
            vec![var_decl_init("constx", number_literal_expr("0"))],
        ),
        block_part(
            vec![
                variable_decl_part(
                    VariableKind::Let,
                    vec![var_decl("x")]
                ),
                variable_decl_part(
                    VariableKind::Let,
                    vec![variable_decl("y", number_literal_expr("0"))],
                ),
                variable_decl_part(
                    VariableKind::Const,
                    vec![variable_decl("z", number_literal_expr("0"))],
                )
            ]
        ),
        null_literal_part(),
        bool_literal_part(true),
        bool_literal_part(false),
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
        number_literal_part("0b0"),
        number_literal_part("0B0"),
        number_literal_part("0b01"),
        number_literal_part("0b10"),
        number_literal_part("0b10101010"),
        number_literal_part("0o0"),
        number_literal_part("0O0"),
        number_literal_part("0o01234567"),
        number_literal_part("2e308"),
        string_literal_part(r#""""#,),
        string_literal_part(r#""'""#,),
        string_literal_part(r#""\'\"\\\b\f\n\r\t\v\0""#,),
        string_literal_part(r#""\1\00\400\000""#,),
        string_literal_part(r#""\x01\x23\x45\x67\x89\xAB\xCD\xEF\xab\xcd\xef""#,),
        string_literal_part(r#""\u0123\u4567\u89AB\uCDEF\u00ab\ucdef""#,),
        string_literal_part(r#""\uD834\uDF06\u2603\u03C6 \u{0000001F4a9}\u{1D306}\u{2603}\u{3c6} 𝌆☃φ""#),
        string_literal_part(r#""\
""#),
        string_literal_part(r"''"),
        string_literal_part(r#"'"'"#),
        string_literal_part(r#"'\'\"\\\b\f\n\r\t\v\0'"#),
        string_literal_part(r#"'\1\00\400\000'"#),
        string_literal_part(r#"'\x01\x23\x45\x67\x89\xAB\xCD\xEF\xab\xcd\xef'"#),
        string_literal_part(r#"'\u0123\u4567\u89AB\uCDEF\u00ab\ucdef'"#),
        string_literal_part(r#"'\uD834\uDF06\u2603\u03C6 \u{0000001F4a9} \u{1D306}\u{2603}\u{3c6} 𝌆☃φ'"#),
        string_literal_part(r#"'\
'"#),
        regex_literal_part(r#"x"#, ""),
        regex_literal_part(r#"|"#, ""),
        regex_literal_part(r#"|||"#, ""),
        regex_literal_part(r#"^$\b\B"#, ""),
        regex_literal_part(r#"(?=(?!(?:(.))))"#, ""),
        regex_literal_part(r#"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000\uD834\uDF06"#,""),
        regex_literal_part(r#"\u{00000001d306}"#,"u"),
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
        template_part(vec![
            template_element("`a`", true)
        ], vec![
        ]),
        template_part(vec![
            template_element("`${", false),
            template_element("}`", true),
        ], vec![
            number_literal_expr("0")
        ]),
        template_part(vec![
            template_element("`0${", false),
            template_element("}2`", true),

        ], vec![
            Expr::Sequence(vec![
                number_literal_expr("0"),
                number_literal_expr("1"),
            ])
        ]),
        template_part(vec![
            template_element("`0${", false),
            template_element("}4`", true),
        ], vec![
            template_expr(vec![
                template_element("`1${", false),
                template_element("}3`", true),
            ], vec![
                number_literal_expr("2")
            ])
        ]),
        template_part(vec![
            template_element(r"`\``", true)
        ], vec![

        ]),
        template_part(vec![
            template_element(r"`a\${b`", true)
        ], vec![

        ]),
        template_part(vec![
            template_element(r"`\0\n\x0A\u000A\u{A}`", true)
        ], vec![

        ]),
        this_part(),
        ProgramPart::Stmt(Stmt::Expr(Expr::Ident("x"))),
        array(vec![]),
        array(vec![None]),
        array(vec![Some(number_literal_expr("0"))]),
        array(vec![Some(number_literal_expr("0"))]),
        array(vec![None, Some(number_literal_expr("0"))]),
        array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]),
        array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]), //60
        array(vec![Some(number_literal_expr("0")), None, Some(number_literal_expr("0"))]),
        array(vec![None, None]),
        obj_literal_part(vec![]),
        obj_literal_part(vec![
            obj_prop(
                obj_key_ident("x"),
                PropertyValue::None,
                PropertyKind::Init,
                false,
                false,
                true,
            )
        ]),
        obj_literal_part(vec![
            obj_prop_ident_number("x", "0")
        ]),
        obj_literal_part(vec![
            obj_prop_ident_number("x", "0"),
            obj_prop_ident_number("y", "0"),
        ]),
        obj_literal_part(vec![
            obj_prop_ident_number("x", "0")
        ]),
        obj_literal_part(vec![
            obj_prop_str_number("'x'", "0"),
            obj_prop_str_number("\"y\"", "0"),
            obj_prop_ident_number("in", "0")
        ]),
        obj_literal_part(vec![
            obj_prop_number_number("0", "0"),
            obj_prop_number_number("0.", "0"),
            obj_prop_number_number("0.0", "0"),
            obj_prop_number_number(".0", "0"),
            obj_prop_number_number("0e0", "0"),
            obj_prop_number_number("0x0", "0"),
            obj_prop(
                obj_key_number("0"),
                obj_value_number("0"),
                PropertyKind::Init,
                false,
                true,
                false,
            ),
            obj_prop_ident_getter("x"),
            obj_prop_ident_setter("x", vec![
                fn_arg_ident_pat("a")
            ]),
            obj_prop_str_getter("'y'"),
            obj_prop_str_setter("\"y\"", vec![
                fn_arg_ident_pat("a")
            ]),
            obj_prop_number_getter("0"),
            obj_prop_number_setter("0", vec![
                fn_arg_ident_pat("a")
            ]),
            obj_prop(
                obj_key_ident("var"),
                obj_value_fn(empty_anon_fn(vec![])),
                PropertyKind::Get,
                false,
                false,
                false,
            ),
            obj_prop(
                obj_key_ident("var"),
                obj_value_fn(empty_anon_fn(vec![
                    fn_arg_ident_pat("a")
                ])),
                PropertyKind::Set,
                false,
                false,
                false,
            ),
            obj_prop(
                obj_key_number("0"),
                obj_value_fn(empty_anon_fn(vec![])),
                PropertyKind::Get,
                false,
                true,
                false,
            ),
            obj_prop(
                obj_key_number("0"),
                obj_value_fn(empty_anon_fn(vec![
                    fn_arg_ident_pat("a")
                ])),
                PropertyKind::Set,
                false,
                true,
                false,
            ),
            obj_prop(
                obj_key_number("1"),
                obj_value_fn(empty_anon_fn(vec![])),
                PropertyKind::Init,
                true,
                true,
                false,
            ),
            obj_prop_ident_fn("a", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_str_fn("'b'", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_str_fn("\"c\"", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn("0", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn(".1", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn("1.", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn("1e1", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_ident_fn("var", empty_anon_fn(
                long_args()
            ), PropertyKind::Init),
            obj_prop_ident_setter("in", vec![
                FunctionArg::Pat(
                    Pat::Array(
                        long_args_array_pat().into_iter().map(|a| Some(ArrayPatPart::Pat(a))).collect()
                    )
                )
            ]),
            empty_generator_prop(obj_key_ident("d")),
            empty_generator_prop(obj_key_string("'e'")),
            empty_generator_prop(obj_key_string("\"f\"")),
            empty_generator_prop(obj_key_number("2")),
            empty_generator_prop(obj_key_number(".2")),
            empty_generator_prop(obj_key_number("3.")),
            empty_generator_prop(obj_key_number("2e2")),
            empty_generator_prop(obj_key_ident("in")),
        ]),
        obj_literal_part(vec![
            obj_prop(
                obj_key_ident("__proto__"),
                PropertyValue::Expr(null_literal_expr()),
                PropertyKind::Init,
                false,
                false,
                false,
            ),
            obj_prop_ident_getter("__proto__"),
            obj_prop_ident_setter("__proto__", vec![fn_arg_ident_pat("a")])
        ]),
        obj_literal_part(vec![
            obj_prop(
                obj_key_string("\"__proto__\""),
                PropertyValue::Expr(null_literal_expr()),
                PropertyKind::Init,
                false,
                false,
                false,
            ),
            obj_prop_ident_fn(
                "__proto__",
                empty_anon_fn(vec![]),
                PropertyKind::Init
            )
        ]),
        member_number_ident_part("0.", "a"),
        member_number_ident_part("0", "a"),
        member_number_ident_part("0", "a"),
        member_number_number_part("0", "0"),
        assign_part(
                assign_left_ident("x"),
                func_expr(
                    "f", vec![], vec![
                        return_ident_part("f")
                    ]
                )
            ),
            assign_part(
                assign_left_expr(
                    member_ident_number_expr("x", "0")
                ),
                Expr::Ident("x")
            ),
            assign_part(
                assign_left_expr(
                    member_ident_ident_expr("x", "a")
                ),
                Expr::Ident("x")
            ),
            new_ident_part("x"),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                Expr::Member(member_ident_number("x", "0")),
                vec![]
            ),
            new_part(
                Expr::Member(member_ident_ident("x", "a")),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        Expr::Member(
                            member_ident_number("x", "0")
                        ),
                        Expr::Ident("a"),
                        false
                    )
                ),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        Expr::Member(
                            member_ident_ident("x", "a")
                        ),
                        number_literal_expr("0"),
                        true,
                    )
                ),
                vec![]
            ),
            new_ident_part("x"),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        new_ident_expr("x", vec![]),
                        Expr::Ident("a"),
                        false
                    )
                ),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        new_ident_expr("x", vec![]),
                        number_literal_expr("0"),
                        true,
                    )
                ),
                vec![]
            ),
            call_ident_part("x", vec![]), //100
            call_part(
                call_ident_expr("x", vec![]),
                vec![],
            ),
            call_ident_part(
                "x", vec![
                    Expr::Ident("x")
                ]
            ),
            call_ident_part(
                "x", vec![
                    Expr::Ident("x"),
                    Expr::Ident("x"),
                ]
            ),
            call_part(
                member_expr(
                    call_expr(
                        member_ident_ident_expr(
                            "x", "a"
                        ),
                        vec![]
                    ),
                    Expr::Ident("a"),
                    false,
                ),
                vec![]
            ),
            call_part(
                member_expr(
                    call_expr(
                        member_ident_number_expr("x", "0"),
                        vec![],
                    ),
                    number_literal_expr("0"),
                    true
                ),
                vec![]
            ),
            call_part(
                member_expr(
                    member_expr(
                        call_expr(
                            Expr::Ident("x"),
                            vec![]
                        ),
                        Expr::Ident("a"),
                        false,
                    ),
                    number_literal_expr("0"),
                    true,
                )
                , vec![]
            ),
            call_part(
                Expr::Ident("x"),
                vec![
                    Expr::Spread(
                        Box::new(Expr::Array(vec![
                            Some(number_literal_expr("0")),
                            Some(number_literal_expr("1")),
                        ]))
                    ),
                    Expr::Spread(
                        Box::new(Expr::Array(vec![]))
                    ),
                    Expr::Spread(
                        Box::new(Expr::Function(
                            Function {
                                id: Some("f"),
                                params: vec![],
                                body: vec![
                                    ProgramPart::Stmt(Stmt::Return(
                                        Some(Expr::Yield(YieldExpr {
                                            argument: Some(Box::new(number_literal_expr("2"))),
                                            delegate: false,
                                        }))
                                    ))
                                ],
                                generator: true,
                                is_async: false,
                            }
                        ))
                    )
                ]
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::TaggedTemplate(
                        TaggedTemplateExpr {
                            tag: Box::new(Expr::Ident("x")),
                            quasi: template(vec![
                                template_element("`a`", true),
                            ], vec![])
                        }
                    )
                )
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::TaggedTemplate(
                        TaggedTemplateExpr {
                            tag: Box::new(Expr::Ident("x")),
                            quasi: template(vec![
                                template_element("`0${", false),
                                template_element("}2`", true),
                            ], vec![
                                number_literal_expr("1"),
                            ])
                        }
                    )
                )
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Increment,
                false
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Decrement,
                false
            ),
            unary_part(
                UnaryOperator::Delete,
                unary_expr(
                    UnaryOperator::Void,
                    unary_expr(
                        UnaryOperator::TypeOf,
                        unary_expr(
                            UnaryOperator::Plus,
                            unary_expr(
                                UnaryOperator::Minus,
                                unary_expr(
                                    UnaryOperator::Tilde,
                                    unary_expr(
                                        UnaryOperator::Not,
                                        Expr::Ident("x"),
                                        true,
                                    ),
                                    true,
                                ),
                                true,
                            ),
                            true,
                        ),
                        true,
                    ),
                    true,
                ),
                true
            ),
            update_part( //110
                Expr::Ident("x"),
                UpdateOperator::Increment,
                true
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Decrement,
                true
            ),
            zero_bin_zero_part(BinaryOperator::Times),
            zero_bin_zero_part(BinaryOperator::Over),
            zero_bin_zero_part(BinaryOperator::Mod),
            zero_bin_zero_part(BinaryOperator::Plus),
            zero_bin_zero_part(BinaryOperator::Minus),
            zero_bin_zero_part(BinaryOperator::LeftShift),
            zero_bin_zero_part(BinaryOperator::RightShift),
            zero_bin_zero_part(BinaryOperator::UnsignedRightShift),
            zero_bin_zero_part(BinaryOperator::LessThan), //120
            zero_bin_zero_part(BinaryOperator::GreaterThan),
            zero_bin_zero_part(BinaryOperator::LessThanEqual),
            zero_bin_zero_part(BinaryOperator::GreaterThanEqual),
            zero_bin_expr_part(
                BinaryOperator::InstanceOf,
                Expr::Function(empty_anon_fn(vec![])),
            ),
            zero_bin_expr_part(
                BinaryOperator::In,
                obj_literal_expr(vec![]),
            ),
            zero_bin_zero_part(BinaryOperator::Equal),
            zero_bin_zero_part(BinaryOperator::NotEqual),
            zero_bin_zero_part(BinaryOperator::StrictEqual),
            zero_bin_zero_part(BinaryOperator::StrictNotEqual),
            zero_bin_zero_part(BinaryOperator::And),//130
            zero_bin_zero_part(BinaryOperator::XOr),
            zero_bin_zero_part(BinaryOperator::Or),
            zero_log_zero(LogicalOperator::And),
            zero_log_zero(LogicalOperator::Or),
            conditional_part(
                number_literal_expr("0"),
                number_literal_expr("0"),
                number_literal_expr("0"),
            ),
            // TODO: Validate this
            conditional_part(
                number_literal_expr("0"),
                conditional_expr(
                    number_literal_expr("0"),
                    number_literal_expr("0"),
                    number_literal_expr("0"),
                ),
                number_literal_expr("0"),
            ),
            conditional_part(
                zero_log_zero_expr(LogicalOperator::Or),
                assign_expr(
                    assign_left_ident("x"),
                    number_literal_expr("0"),
                ),
                assign_expr(
                    assign_left_ident("x"),
                    number_literal_expr("0"),
                ),
            ),
            // ^^^^^^^^^^^^^^^^^^^^^^^^^^
            ident_assign_zero_part("x", AssignmentOperator::Equal),
            ident_assign_zero_part("x", AssignmentOperator::TimesEqual),//140
            ident_assign_zero_part("x", AssignmentOperator::DivEqual),
            ident_assign_zero_part("x", AssignmentOperator::ModEqual),
            ident_assign_zero_part("x", AssignmentOperator::PlusEqual),
            ident_assign_zero_part("x", AssignmentOperator::MinusEqual),
            ident_assign_zero_part("x", AssignmentOperator::LeftShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::RightShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::UnsignedRightShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::AndEqual),
            ident_assign_zero_part("x", AssignmentOperator::XOrEqual),
            ident_assign_zero_part("x", AssignmentOperator::OrEqual), //150
            zero_sequence(2),
            zero_sequence(3),
            sequence(vec![
                ident_assign_zero_expr("x", AssignmentOperator::Equal),
                ident_assign_zero_expr("x", AssignmentOperator::Equal),
            ]),
            block_part(vec![]),
            block_part(vec![ProgramPart::Stmt(Stmt::Empty)]),
            block_part(vec![number_literal_part("0")]),
            block_part(vec![number_literal_part("0")]),
            block_part(vec![
                number_literal_part("0"),
                number_literal_part("0"),
            ]),
            block_part(vec![
                number_literal_part("0"),
                number_literal_part("0"),
            ]),
            variable_decl_part(
                VariableKind::Var,
                vec![
                    variable_decl_none("x") //160
                ]
            ),
            variable_decl_part(
                VariableKind::Var,
                vec![
                variable_decl_none("x"),
                variable_decl_none("y"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_none("x"),
                variable_decl_none("y"),
                variable_decl_none("z"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x"),
                variable_decl_none("y"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_none("x"),
                variable_decl_zero("y"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x"),
                variable_decl_zero("y"),
            ]),
            ProgramPart::Stmt(Stmt::Empty),
            if_zero_empty(),
            if_zero_empty_else(),
            do_while_zero(),
            number_literal_part("0"),
            do_while_zero(),
            do_while_zero(),
            number_literal_part("0"),
            while_zero(),
            for_exprs_part(None, None, None, Stmt::Break(None)),
            for_exprs_part(
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Stmt::Empty,
            ),
            for_exprs_part(
                Some(zero_bin_expr_expr(
                    BinaryOperator::In,
                    array_expr(vec![]),
                )),
                Some(number_literal_expr("0")),
                None,
                Stmt::Empty,
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                var_decl("a")
                ],
                None,
                None,
                Stmt::Break(None),
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                var_decl("a"),
                var_decl("b"),
                ],
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Stmt::Empty,
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                variable_decl_zero("a")
                ],
                None,
                None,
                Stmt::Break(None),
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                    VariableDecl {
                        id: Pat::Identifier("a"),
                        init: Some(
                            zero_bin_expr_expr(
                                BinaryOperator::In,
                                array_expr(vec![])
                            )
                        )
                    }
                ],
                Some(number_literal_expr("0")),
                None,
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Expr(Expr::Ident("x")),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Variable(
                    VariableKind::Var,
                    var_decl("x")
                ),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            ProgramPart::Stmt(
                Stmt::ForOf(
                    ForOfStmt {
                        left: LoopLeft::Expr(
                            Expr::Ident("x")
                        ),
                        right: array_expr(vec![]),
                        body: Box::new(Stmt::Empty),
                        is_await: false
                    }
                )
            ),
            ProgramPart::Stmt(
                Stmt::ForOf(
                    ForOfStmt {
                        left: LoopLeft::Variable(
                            VariableKind::Var,
                            var_decl("x")
                        ),
                        right: array_expr(vec![]),
                        body: Box::new(Stmt::Empty),
                        is_await: false
                    }
                )
            ),
            for_exprs_part(None, Some(number_literal_expr("0")), None, Stmt::Continue(None)),
            labeled_statement_continue("x"),
            for_exprs_part(None, None, None, Stmt::Break(None)),
            preamble_labeled_statement("x"),

            switch_zero_part(vec![
                case_zero(vec![ProgramPart::Stmt(Stmt::Break(None))])
            ]),
            func_part(
                "f",
                vec![],
                vec![ProgramPart::Stmt(Stmt::Return(None))]
            ),
            func_part(
                "f",
                vec![],
                vec![ProgramPart::Stmt(Stmt::Return(
                    Some(number_literal_expr("0"))
                ))]
            ),
            ProgramPart::Stmt(Stmt::With(WithStmt { object: number_literal_expr("0"), body: Box::new(Stmt::Empty)})),
            switch_zero_part(vec![]),
            switch_zero_part(vec![
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![]),
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                default_case(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![]),
                default_case(vec![]),
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![
                    empty_part(),
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                default_case(vec![
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                default_case(vec![
                    empty_part(),
                    empty_part(),
                ]),
            ]),
            labeled_part("x", Stmt::Empty),
            labeled_part(
                "x",
                labeled_stmt("y", Stmt::Empty)
            ),
            try_part(
                vec![
                    throw_part(
                        number_literal_expr("0")
                    )
                ],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                None
            ),
            try_part(
                vec![],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                None
            ),
            try_part(
                vec![],
                None,
                Some(vec![])
            ),
            try_part(
                vec![],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                Some(vec![])
            ),
            ProgramPart::Stmt(Stmt::Debugger),
            func_part("f", vec![], vec![]),
            func_part(
                "f",
                vec![
                    fn_arg_ident_pat("x"),
                ],
                vec![]
            ),
            func_part(
                "f",
                vec![
                    fn_arg_ident_pat("x"),
                    fn_arg_ident_pat("y"),
                ],
                vec![]
            ),
            func_part(
                "f",
                vec![],
                vec![
                    func_part(
                        "f",
                        vec![],
                        vec![]
                    ),
                ]
            ),
        block_part(vec![
            func_part("f", vec![], vec![])
        ]),
        empty_part(),
        func_part("f", long_args(), vec![]),
        func_part(
                "f",
                vec![],
                vec![
                    directive_part("\"use strict\""),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("'use strict'"),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("\"other directive\""),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    directive_part("'other directive'"),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    string_literal_part("\"string\""),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    string_literal_part("'string'"),
                ],
            ),
            func_part(
                "f",
                vec![],
                vec![
                    ProgramPart::Stmt(
                        Stmt::Expr(
                            Expr::Binary(
                                binary(
                                    string_literal_expr("'string'"),
                                    BinaryOperator::Plus,
                                    number_literal_expr("0")
                                )
                            )
                        )
                    ),
                ],
            ),
            ProgramPart::Decl(
                Decl::Function(
                    Function {
                        params: long_args(),
                        id: Some("g"),
                        body: long_gen_body(),
                        generator: true,
                        is_async: false,
                    }
                )
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::Function(
                        Function {
                            params: long_args(),
                            id: Some("g"),
                            body: long_gen_body(),
                            generator: true,
                            is_async: false,
                        }
                    )
                )
            ),
            ident_stmt("yield"),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::Binary(
                        BinaryExpr {
                            left: Box::new(
                                Expr::Ident("yield")
                            ),
                            operator: BinaryOperator::Plus,
                            right: Box::new(
                                number_literal_expr("0")
                            )
                        }
                    )
                )
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::Binary(
                        BinaryExpr {
                            left: Box::new(
                                Expr::Ident("yield")
                            ),
                            operator: BinaryOperator::Times,
                            right: Box::new(
                                number_literal_expr("0")
                            )
                        }
                    )
                )
            ),
            func_expr_part(None, vec![], vec![]),
            func_expr_part(None, vec![
                fn_arg_ident_pat("x")
            ], vec![]),
            func_expr_part(None, vec![
                fn_arg_ident_pat("x"),
                fn_arg_ident_pat("y"),
            ], vec![]),
            func_expr_part(None, vec![], vec![
                func_part("f", vec![], vec![]),
            ]),
            func_expr_part(Some("f"), vec![], vec![]),
            func_expr_part(Some("f"), vec![
                fn_arg_ident_pat("x")
            ], vec![]),
            func_expr_part(Some("f"), vec![
                fn_arg_ident_pat("x"),
                fn_arg_ident_pat("y"),
            ], vec![]),
            func_expr_part(Some("f"), vec![], vec![
                func_part("f", vec![], vec![]),
            ]),
            arrow_expr_part(vec![], number_literal_expr("0")),
            arrow_expr_body_part(vec![],vec![
                empty_part()
            ]),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], Expr::Ident("x")),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], assign_expr(
                assign_left_ident("x"),
                number_literal_expr("0"))),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x")
            ], Expr::ArrowFunction(arrow_expr(vec![
                fn_arg_ident_expr("y")
            ], Expr::Ident("x")))),
            arrow_expr_body_part(vec![
                fn_arg_ident_expr("x")
            ], vec![
                ident_stmt("x")
            ]),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], obj_literal_expr(vec![
                    obj_prop(
                        obj_key_ident("x"),
                        PropertyValue::None,
                        PropertyKind::Init,
                        false,
                        false,
                        true
                    )
                ])
            ),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], Expr::Ident("x")),
            arrow_expr_body_part(vec![
                fn_arg_ident_expr("x"),
            ],vec![
                ProgramPart::Stmt(
                    return_ident_stmt("x")
                )
            ]),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], obj_literal_expr(vec![
                    obj_prop(
                        obj_key_ident("x"),
                        PropertyValue::None,
                        PropertyKind::Init,
                        false,
                        false,
                        true
                    )
                ])
            ),
            arrow_expr_part(vec![
                FunctionArg::Expr(
                    Expr::Object(vec![
                        obj_prop(
                            obj_key_ident("x"),
                            PropertyValue::None,
                            PropertyKind::Init,
                            false,
                            false,
                            true,
                        )
                    ])
                ),
            ], obj_literal_expr(vec![
                    obj_prop(
                        obj_key_ident("x"),
                        PropertyValue::None,
                        PropertyKind::Init,
                        false,
                        false,
                        true
                    )
                ])
            ),
            arrow_expr_body_part(
                long_args_array_expr(),
                vec![
                    empty_part()
                ]
            ),
            assign_part(
                assign_left_expr(
                    array_expr(vec![
                        Some(Expr::Ident("a"))
                    ])
                ),
                array_expr(vec![
                    Some(Expr::Spread(
                        Box::new(
                            array_expr(vec![
                                Some(number_literal_expr("0"))
                            ])
                        )
                    ))
                ])
            ),
            assign_part(
                assign_left_expr(
                    obj_literal_expr(vec![
                        obj_prop(
                            obj_key_ident("a"),
                            PropertyValue::None,
                            PropertyKind::Init,
                            false,
                            false,
                            true,
                        )
                    ])
                ),
                obj_literal_expr(vec![])
            ),
            try_part(vec![], Some(
                CatchClause {
                    param: Some(
                        Pat::Array(vec![
                            Some(ArrayPatPart::Pat(
                                Pat::Identifier("e")
                            ))
                        ]
                        )
                    ),
                    body: vec![]
                }
            ), None),
            try_part(vec![], Some(
                CatchClause {
                    param: Some(
                        Pat::Object(vec![
                            ObjectPatPart::Assignment(
                                Property {
                                    key: obj_key_ident("e"),
                                    value: PropertyValue::None,
                                    kind: PropertyKind::Init,
                                    method: false,
                                    computed: false,
                                    short_hand: true,
                                    is_static: false,
                                }
                            )
                        ]
                        )
                    ),
                    body: vec![]
                }
            ), None),
            class_part(
                "A",
                None,
                vec![],
            ),
            class_part(
                "B",
                Some(new_expr(Expr::Ident("A"), vec![])),
                vec![
                    class_prop(
                        obj_key_ident("constructor"),
                        anon_fn(long_args(), vec![
                            call_part(Expr::Super, vec![
                                Expr::MetaProperty(
                                    MetaProperty {
                                        meta: "new",
                                        property: "target"
                                    }
                                )
                            ]),
                            ProgramPart::Stmt(
                                Stmt::Expr(
                                    Expr::TaggedTemplate(
                                        TaggedTemplateExpr {
                                            tag: Box::new(Expr::Call(
                                                CallExpr {
                                                    callee: Box::new(Expr::Super),
                                                    arguments: vec![]
                                                }
                                            )),
                                            quasi: template(vec![
                                                template_element("`template`", true),
                                            ], vec![])
                                        }
                                    )
                                )
                            ),
                            arrow_expr_part(vec![], call_expr(Expr::Super, vec![
                                Expr::This
                            ])),
                        ],
                        false,
                    ),
                    true,
                    false),
                    class_prop(
                        obj_key_ident("m"),
                        anon_fn(long_args(), vec![
                            call_part(
                                member_expr(
                                    Expr::Super,
                                    Expr::Ident("m"),
                                    false,
                                ),
                                vec![]
                            ),
                            ProgramPart::Stmt(
                                Stmt::Expr(
                                    Expr::TaggedTemplate(
                                        TaggedTemplateExpr {
                                            tag: Box::new(
                                                member_expr(Expr::Super, Expr::Ident("m"), false)
                                            ),
                                            quasi: template(vec![
                                                template_element("`template`", true),
                                            ], vec![])
                                        }
                                    )
                                )
                            ),
                            arrow_expr_part(vec![], call_expr(
                                member_expr(Expr::Super, Expr::Ident("m"), false),
                                vec![
                                    Expr::This
                                ]
                            )),
                        ], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("a"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true,
                    ),
                    class_prop(
                        obj_key_string("'b'"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true,
                    ),
                    class_prop(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true
                    ),
                    class_prop_computed(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_ident("c"),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_string("\"d\""),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop_computed(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_ident("var"),
                        anon_fn(vec![], vec![

                        ], false),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_ident("in"),
                        anon_fn(vec![], vec![
                        ], true),
                        false,
                        true
                    ),
                    class_prop_getter(
                        obj_key_ident("e"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_string("'f'"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        true,
                        true
                    ),
                    class_prop_setter(
                        obj_key_ident("g"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_string("\"h\""),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        true,
                    ),
                    class_prop_getter(
                        obj_key_ident("if"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_ident("if"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("a"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_string("'b'"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_computed(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("c"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_string("\"d\""),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop_computed(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("var"),
                        anon_fn(vec![], vec![
                        ], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("in"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_ident("e"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_string("'f'"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true,
                    ),
                    class_prop_setter(
                        obj_key_ident("g"),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_string("\"h\""),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        true,
                    ),
                    class_prop_getter(
                        obj_key_ident("if"),
                        anon_fn(vec![], vec![
                        ], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_ident("if"),
                        anon_fn(vec![fn_arg_ident_pat("f")], vec![], false),
                        false,
                        false,
                    ),
                ]
            ),
            class_part("C", Some(
                Expr::Ident("B")
            ), vec![
                class_prop(obj_key_string("\"constructor\""), anon_fn(vec![], vec![
                    call_part(Expr::Super, vec![])
                ], false), true, false),
            ])
        // ProgramPart::Stmt(Stmt::Empty)
    ];
}
lazy_static! {
    pub static ref ESMOD: Vec<ProgramPart<'static>> = vec![
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                            ImportSpecifier::Default("i0")
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                            ImportSpecifier::Namespace("i1")
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                            ImportSpecifier::Normal("i2", None),
                            ImportSpecifier::Normal("a", Some("i3")),
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                            ImportSpecifier::Default("i4"),
                            ImportSpecifier::Namespace("i5"),
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                            ImportSpecifier::Default("i6"),
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                            ImportSpecifier::Default("i7"),
                            ImportSpecifier::Normal("i8", None),
                            ImportSpecifier::Normal("var", Some("i9")),
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Import(
                Box::new(
                    ModImport {
                        specifiers: vec![
                        ],
                        source: Literal::String("\"module\"")
                    }
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::All(Literal::String("\"module\""))
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Specifier(
                            vec![],
                            Some(Literal::String("\"module\""))
                        )
                    ),
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Specifier(
                            vec![
                                ExportSpecifier {
                                    local: "i0",
                                    exported: None,
                                },
                                ExportSpecifier {
                                    local: "i1",
                                    exported: Some("a"),
                                },
                                ExportSpecifier {
                                    local: "i2",
                                    exported: Some("var"),
                                },
                            ],
                            Some(Literal::String("\"module\""))
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Specifier(
                            vec![
                            ],
                            None
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Specifier(
                            vec![
                                ExportSpecifier {
                                    local: "i3",
                                    exported: None,
                                },
                                ExportSpecifier {
                                    local: "i4",
                                    exported: Some("in"),
                                },
                            ],
                            None
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            Decl::Variable(
                                VariableKind::Var,
                                vec![
                                    variable_decl_none("e5"),
                                    var_decl_init(
                                        "e6",
                                        number_literal_expr("0")
                                    )
                                ]
                            )
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            Decl::Variable(
                                VariableKind::Let,
                                vec![
                                    variable_decl_none("e7"),
                                    var_decl_init(
                                        "e8",
                                        number_literal_expr("0")
                                    )
                                ]
                            )
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            Decl::Variable(
                                VariableKind::Const,
                                vec![
                                    var_decl_init(
                                        "e9",
                                        number_literal_expr("0")
                                    ),
                                    var_decl_init(
                                        "e10",
                                        number_literal_expr("0")
                                    )
                                ]
                            )
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            Decl::Function(
                                func("e11", vec![], vec![])
                            )
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            Decl::Function(
                                func_generator("e12", vec![], vec![])
                            )
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            class_decl("e13", None, vec![])
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Named(
                        NamedExportDecl::Decl(
                            class_decl("e14", Some(Expr::Ident("e15")), vec![])
                        )
                    )
                )
            )
        ),
        ProgramPart::Decl(
            Decl::Export(
                Box::new(
                    ModExport::Default(
                        DefaultExportDecl::Decl(
                            Decl::Function(
                                func("e16", vec![], vec![])
                            )
                        )
                    )
                )
            )
        ),
        preamble_labeled_statement("tab"),
        preamble_labeled_statement("verticalTab"),
        preamble_labeled_statement("formFeed"),
        preamble_labeled_statement("space"),
        preamble_labeled_statement("nbsp"),
        preamble_labeled_statement("bom"),
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
        var_decl_one_2015(),
        var_decl_two_2015(),
        variable_decl_part(
            VariableKind::Let,
            vec![var_decl("letx")],
        ),
        variable_decl_part(
            VariableKind::Let,
            vec![
                VariableDecl {
                    id: Pat::Array(
                        vec![
                            Some(
                                ArrayPatPart::Pat(
                                    Pat::Identifier(r"x\u0078")
                                )
                            )
                        ]
                    ),
                    init: Some(number_literal_expr("0"))
                }
            ]
        ),
        variable_decl_part(
            VariableKind::Const,
            vec![var_decl_init("constx", number_literal_expr("0"))],
        ),
        block_part(
            vec![
                variable_decl_part(
                    VariableKind::Let,
                    vec![var_decl("x")]
                ),
                variable_decl_part(
                    VariableKind::Let,
                    vec![variable_decl("y", number_literal_expr("0"))],
                ),
                variable_decl_part(
                    VariableKind::Const,
                    vec![variable_decl("z", number_literal_expr("0"))],
                )
            ]
        ),
        null_literal_part(),
        bool_literal_part(true),
        bool_literal_part(false),
        number_literal_part("0"),
        number_literal_part("1234567890"),
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
        number_literal_part("0b0"),
        number_literal_part("0B0"),
        number_literal_part("0b01"),
        number_literal_part("0b10"),
        number_literal_part("0b10101010"),
        number_literal_part("0o0"),
        number_literal_part("0O0"),
        number_literal_part("0o01234567"),
        number_literal_part("2e308"),
        string_literal_part(r#""""#,),
        string_literal_part(r#""'""#,),
        string_literal_part(r#""\'\"\\\b\f\n\r\t\v\0""#,),
        string_literal_part(r#""\0""#,),
        string_literal_part(r#""\x01\x23\x45\x67\x89\xAB\xCD\xEF\xab\xcd\xef""#,),
        string_literal_part(r#""\u0123\u4567\u89AB\uCDEF\u00ab\ucdef""#,),
        string_literal_part(r#""\uD834\uDF06\u2603\u03C6 \u{0000001F4a9}\u{1D306}\u{2603}\u{3c6} 𝌆☃φ""#),
        string_literal_part(r#""\
""#),
        string_literal_part(r"''"),
        string_literal_part(r#"'"'"#),
        string_literal_part(r#"'\'\"\\\b\f\n\r\t\v\0'"#),
        string_literal_part(r#"'\0'"#,),
        string_literal_part(r#"'\x01\x23\x45\x67\x89\xAB\xCD\xEF\xab\xcd\xef'"#),
        string_literal_part(r#"'\u0123\u4567\u89AB\uCDEF\u00ab\ucdef'"#),
        string_literal_part(r#"'\uD834\uDF06\u2603\u03C6 \u{0000001F4a9} \u{1D306}\u{2603}\u{3c6} 𝌆☃φ'"#),
        string_literal_part(r#"'\
'"#),
        regex_literal_part(r#"x"#, ""),
        regex_literal_part(r#"|"#, ""),
        regex_literal_part(r#"|||"#, ""),
        regex_literal_part(r#"^$\b\B"#, ""),
        regex_literal_part(r#"(?=(?!(?:(.))))"#, ""),
        regex_literal_part(r#"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000\uD834\uDF06"#,""),
        regex_literal_part(r#"\u{00000001d306}"#,"u"),
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
        template_part(vec![
            template_element("`a`", true)
        ], vec![
        ]),
        template_part(vec![
            template_element("`${", false),
            template_element("}`", true),
        ], vec![
            number_literal_expr("0")
        ]),
        template_part(vec![
            template_element("`0${", false),
            template_element("}2`", true),

        ], vec![
            Expr::Sequence(vec![
                number_literal_expr("0"),
                number_literal_expr("1"),
            ])
        ]),
        template_part(vec![
            template_element("`0${", false),
            template_element("}4`", true),
        ], vec![
            template_expr(vec![
                template_element("`1${", false),
                template_element("}3`", true),
            ], vec![
                number_literal_expr("2")
            ])
        ]),
        template_part(vec![
            template_element(r"`\``", true)
        ], vec![

        ]),
        template_part(vec![
            template_element(r"`a\${b`", true)
        ], vec![

        ]),
        template_part(vec![
            template_element(r"`\0\n\x0A\u000A\u{A}`", true)
        ], vec![

        ]),
        this_part(),
        ProgramPart::Stmt(Stmt::Expr(Expr::Ident("x"))),
        array(vec![]),
        array(vec![None]),
        array(vec![Some(number_literal_expr("0"))]),
        array(vec![Some(number_literal_expr("0"))]),
        array(vec![None, Some(number_literal_expr("0"))]),
        array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]),
        array(vec![Some(number_literal_expr("0")), Some(number_literal_expr("0"))]), //60
        array(vec![Some(number_literal_expr("0")), None, Some(number_literal_expr("0"))]),
        array(vec![None, None]),
        obj_literal_part(vec![]),
        obj_literal_part(vec![
            obj_prop(
                obj_key_ident("x"),
                PropertyValue::None,
                PropertyKind::Init,
                false,
                false,
                true,
            )
        ]),
        obj_literal_part(vec![
            obj_prop_ident_number("x", "0")
        ]),
        obj_literal_part(vec![
            obj_prop_ident_number("x", "0"),
            obj_prop_ident_number("y", "0"),
        ]),
        obj_literal_part(vec![
            obj_prop_ident_number("x", "0")
        ]),
        obj_literal_part(vec![
            obj_prop_str_number("'x'", "0"),
            obj_prop_str_number("\"y\"", "0"),
            obj_prop_ident_number("in", "0")
        ]),
        obj_literal_part(vec![
            obj_prop_number_number("0", "0"),
            obj_prop_number_number("0.", "0"),
            obj_prop_number_number("0.0", "0"),
            obj_prop_number_number(".0", "0"),
            obj_prop_number_number("0e0", "0"),
            obj_prop_number_number("0x0", "0"),
            obj_prop(
                obj_key_number("0"),
                obj_value_number("0"),
                PropertyKind::Init,
                false,
                true,
                false,
            ),
            obj_prop_ident_getter("x"),
            obj_prop_ident_setter("x", vec![
                fn_arg_ident_pat("a")
            ]),
            obj_prop_str_getter("'y'"),
            obj_prop_str_setter("\"y\"", vec![
                fn_arg_ident_pat("a")
            ]),
            obj_prop_number_getter("0"),
            obj_prop_number_setter("0", vec![
                fn_arg_ident_pat("a")
            ]),
            obj_prop(
                obj_key_ident("var"),
                obj_value_fn(empty_anon_fn(vec![])),
                PropertyKind::Get,
                false,
                false,
                false,
            ),
            obj_prop(
                obj_key_ident("var"),
                obj_value_fn(empty_anon_fn(vec![
                    fn_arg_ident_pat("a")
                ])),
                PropertyKind::Set,
                false,
                false,
                false,
            ),
            obj_prop(
                obj_key_number("0"),
                obj_value_fn(empty_anon_fn(vec![])),
                PropertyKind::Get,
                false,
                true,
                false,
            ),
            obj_prop(
                obj_key_number("0"),
                obj_value_fn(empty_anon_fn(vec![
                    fn_arg_ident_pat("a")
                ])),
                PropertyKind::Set,
                false,
                true,
                false,
            ),
            obj_prop(
                obj_key_number("1"),
                obj_value_fn(empty_anon_fn(vec![])),
                PropertyKind::Init,
                true,
                true,
                false,
            ),
            obj_prop_ident_fn("a", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_str_fn("'b'", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_str_fn("\"c\"", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn("0", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn(".1", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn("1.", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_number_fn("1e1", empty_anon_fn(vec![]), PropertyKind::Init),
            obj_prop_ident_fn("var", empty_anon_fn(
                long_args()
            ), PropertyKind::Init),
            obj_prop_ident_setter("in", vec![
                FunctionArg::Pat(
                    Pat::Array(
                        long_args_array_pat().into_iter().map(|a| Some(ArrayPatPart::Pat(a))).collect()
                    )
                )
            ]),
            empty_generator_prop(obj_key_ident("d")),
            empty_generator_prop(obj_key_string("'e'")),
            empty_generator_prop(obj_key_string("\"f\"")),
            empty_generator_prop(obj_key_number("2")),
            empty_generator_prop(obj_key_number(".2")),
            empty_generator_prop(obj_key_number("3.")),
            empty_generator_prop(obj_key_number("2e2")),
            empty_generator_prop(obj_key_ident("in")),
        ]),
        obj_literal_part(vec![
            obj_prop(
                obj_key_ident("__proto__"),
                PropertyValue::Expr(null_literal_expr()),
                PropertyKind::Init,
                false,
                false,
                false,
            ),
            obj_prop_ident_getter("__proto__"),
            obj_prop_ident_setter("__proto__", vec![fn_arg_ident_pat("a")])
        ]),
        obj_literal_part(vec![
            obj_prop(
                obj_key_string("\"__proto__\""),
                PropertyValue::Expr(null_literal_expr()),
                PropertyKind::Init,
                false,
                false,
                false,
            ),
            obj_prop_ident_fn(
                "__proto__",
                empty_anon_fn(vec![]),
                PropertyKind::Init
            )
        ]),
        member_number_ident_part("0.", "a"),
        member_number_ident_part("0", "a"),
        member_number_ident_part("0", "a"),
        member_number_number_part("0", "0"),
        assign_part(
                assign_left_ident("x"),
                func_expr(
                    "f", vec![], vec![
                        return_ident_part("f")
                    ]
                )
            ),
            assign_part(
                assign_left_expr(
                    member_ident_number_expr("x", "0")
                ),
                Expr::Ident("x")
            ),
            assign_part(
                assign_left_expr(
                    member_ident_ident_expr("x", "a")
                ),
                Expr::Ident("x")
            ),
            new_ident_part("x"),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                Expr::Member(member_ident_number("x", "0")),
                vec![]
            ),
            new_part(
                Expr::Member(member_ident_ident("x", "a")),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        Expr::Member(
                            member_ident_number("x", "0")
                        ),
                        Expr::Ident("a"),
                        false
                    )
                ),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        Expr::Member(
                            member_ident_ident("x", "a")
                        ),
                        number_literal_expr("0"),
                        true,
                    )
                ),
                vec![]
            ),
            new_ident_part("x"),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                new_ident_expr("x", vec![]),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        new_ident_expr("x", vec![]),
                        Expr::Ident("a"),
                        false
                    )
                ),
                vec![]
            ),
            new_part(
                Expr::Member(
                    member(
                        new_ident_expr("x", vec![]),
                        number_literal_expr("0"),
                        true,
                    )
                ),
                vec![]
            ),
            call_ident_part("x", vec![]), //100
            call_part(
                call_ident_expr("x", vec![]),
                vec![],
            ),
            call_ident_part(
                "x", vec![
                    Expr::Ident("x")
                ]
            ),
            call_ident_part(
                "x", vec![
                    Expr::Ident("x"),
                    Expr::Ident("x"),
                ]
            ),
            call_part(
                member_expr(
                    call_expr(
                        member_ident_ident_expr(
                            "x", "a"
                        ),
                        vec![]
                    ),
                    Expr::Ident("a"),
                    false,
                ),
                vec![]
            ),
            call_part(
                member_expr(
                    call_expr(
                        member_ident_number_expr("x", "0"),
                        vec![],
                    ),
                    number_literal_expr("0"),
                    true
                ),
                vec![]
            ),
            call_part(
                member_expr(
                    member_expr(
                        call_expr(
                            Expr::Ident("x"),
                            vec![]
                        ),
                        Expr::Ident("a"),
                        false,
                    ),
                    number_literal_expr("0"),
                    true,
                )
                , vec![]
            ),
            call_part(
                Expr::Ident("x"),
                vec![
                    Expr::Spread(
                        Box::new(Expr::Array(vec![
                            Some(number_literal_expr("0")),
                            Some(number_literal_expr("1")),
                        ]))
                    ),
                    Expr::Spread(
                        Box::new(Expr::Array(vec![]))
                    ),
                    Expr::Spread(
                        Box::new(Expr::Function(
                            Function {
                                id: Some("f"),
                                params: vec![],
                                body: vec![
                                    ProgramPart::Stmt(Stmt::Return(
                                        Some(Expr::Yield(YieldExpr {
                                            argument: Some(Box::new(number_literal_expr("2"))),
                                            delegate: false,
                                        }))
                                    ))
                                ],
                                generator: true,
                                is_async: false,
                            }
                        ))
                    )
                ]
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::TaggedTemplate(
                        TaggedTemplateExpr {
                            tag: Box::new(Expr::Ident("x")),
                            quasi: template(vec![
                                template_element("`a`", true),
                            ], vec![])
                        }
                    )
                )
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::TaggedTemplate(
                        TaggedTemplateExpr {
                            tag: Box::new(Expr::Ident("x")),
                            quasi: template(vec![
                                template_element("`0${", false),
                                template_element("}2`", true),
                            ], vec![
                                number_literal_expr("1"),
                            ])
                        }
                    )
                )
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Increment,
                false
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Decrement,
                false
            ),
            unary_part(
                UnaryOperator::Delete,
                unary_expr(
                    UnaryOperator::Void,
                    unary_expr(
                        UnaryOperator::TypeOf,
                        unary_expr(
                            UnaryOperator::Plus,
                            unary_expr(
                                UnaryOperator::Minus,
                                unary_expr(
                                    UnaryOperator::Tilde,
                                    unary_expr(
                                        UnaryOperator::Not,
                                        Expr::Ident("x"),
                                        true,
                                    ),
                                    true,
                                ),
                                true,
                            ),
                            true,
                        ),
                        true,
                    ),
                    true,
                ),
                true
            ),
            update_part( //110
                Expr::Ident("x"),
                UpdateOperator::Increment,
                true
            ),
            update_part(
                Expr::Ident("x"),
                UpdateOperator::Decrement,
                true
            ),
            zero_bin_zero_part(BinaryOperator::Times),
            zero_bin_zero_part(BinaryOperator::Over),
            zero_bin_zero_part(BinaryOperator::Mod),
            zero_bin_zero_part(BinaryOperator::Plus),
            zero_bin_zero_part(BinaryOperator::Minus),
            zero_bin_zero_part(BinaryOperator::LeftShift),
            zero_bin_zero_part(BinaryOperator::RightShift),
            zero_bin_zero_part(BinaryOperator::UnsignedRightShift),
            zero_bin_zero_part(BinaryOperator::LessThan), //120
            zero_bin_zero_part(BinaryOperator::GreaterThan),
            zero_bin_zero_part(BinaryOperator::LessThanEqual),
            zero_bin_zero_part(BinaryOperator::GreaterThanEqual),
            zero_bin_expr_part(
                BinaryOperator::InstanceOf,
                Expr::Function(empty_anon_fn(vec![])),
            ),
            zero_bin_expr_part(
                BinaryOperator::In,
                obj_literal_expr(vec![]),
            ),
            zero_bin_zero_part(BinaryOperator::Equal),
            zero_bin_zero_part(BinaryOperator::NotEqual),
            zero_bin_zero_part(BinaryOperator::StrictEqual),
            zero_bin_zero_part(BinaryOperator::StrictNotEqual),
            zero_bin_zero_part(BinaryOperator::And),//130
            zero_bin_zero_part(BinaryOperator::XOr),
            zero_bin_zero_part(BinaryOperator::Or),
            zero_log_zero(LogicalOperator::And),
            zero_log_zero(LogicalOperator::Or),
            conditional_part(
                number_literal_expr("0"),
                number_literal_expr("0"),
                number_literal_expr("0"),
            ),
            // TODO: Validate this
            conditional_part(
                number_literal_expr("0"),
                conditional_expr(
                    number_literal_expr("0"),
                    number_literal_expr("0"),
                    number_literal_expr("0"),
                ),
                number_literal_expr("0"),
            ),
            conditional_part(
                zero_log_zero_expr(LogicalOperator::Or),
                assign_expr(
                    assign_left_ident("x"),
                    number_literal_expr("0"),
                ),
                assign_expr(
                    assign_left_ident("x"),
                    number_literal_expr("0"),
                ),
            ),
            // ^^^^^^^^^^^^^^^^^^^^^^^^^^
            ident_assign_zero_part("x", AssignmentOperator::Equal),
            ident_assign_zero_part("x", AssignmentOperator::TimesEqual),//140
            ident_assign_zero_part("x", AssignmentOperator::DivEqual),
            ident_assign_zero_part("x", AssignmentOperator::ModEqual),
            ident_assign_zero_part("x", AssignmentOperator::PlusEqual),
            ident_assign_zero_part("x", AssignmentOperator::MinusEqual),
            ident_assign_zero_part("x", AssignmentOperator::LeftShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::RightShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::UnsignedRightShiftEqual),
            ident_assign_zero_part("x", AssignmentOperator::AndEqual),
            ident_assign_zero_part("x", AssignmentOperator::XOrEqual),
            ident_assign_zero_part("x", AssignmentOperator::OrEqual), //150
            zero_sequence(2),
            zero_sequence(3),
            sequence(vec![
                ident_assign_zero_expr("x", AssignmentOperator::Equal),
                ident_assign_zero_expr("x", AssignmentOperator::Equal),
            ]),
            block_part(vec![]),
            block_part(vec![ProgramPart::Stmt(Stmt::Empty)]),
            block_part(vec![number_literal_part("0")]),
            block_part(vec![number_literal_part("0")]),
            block_part(vec![
                number_literal_part("0"),
                number_literal_part("0"),
            ]),
            block_part(vec![
                number_literal_part("0"),
                number_literal_part("0"),
            ]),
            variable_decl_part(
                VariableKind::Var,
                vec![
                    variable_decl_none("x0") //160
                ]
            ),
            variable_decl_part(
                VariableKind::Var,
                vec![
                variable_decl_none("x1"),
                variable_decl_none("y2"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_none("x3"),
                variable_decl_none("y4"),
                variable_decl_none("z5"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x6"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x7"),
                variable_decl_none("y8"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_none("x9"),
                variable_decl_zero("y10"),
            ]),
            variable_decl_part(VariableKind::Var,vec![
                variable_decl_zero("x11"),
                variable_decl_zero("y12"),
            ]),
            ProgramPart::Stmt(Stmt::Empty),
            if_zero_empty(),
            if_zero_empty_else(),
            do_while_zero(),
            number_literal_part("0"),
            do_while_zero(),
            do_while_zero(),
            number_literal_part("0"),
            while_zero(),
            for_exprs_part(None, None, None, Stmt::Break(None)),
            for_exprs_part(
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Stmt::Empty,
            ),
            for_exprs_part(
                Some(zero_bin_expr_expr(
                    BinaryOperator::In,
                    array_expr(vec![]),
                )),
                Some(number_literal_expr("0")),
                None,
                Stmt::Empty,
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                var_decl("a0")
                ],
                None,
                None,
                Stmt::Break(None),
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                var_decl("a1"),
                var_decl("b2"),
                ],
                Some(number_literal_expr("0")),
                Some(number_literal_expr("0")),
                Stmt::Empty,
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                variable_decl_zero("a3")
                ],
                None,
                None,
                Stmt::Break(None),
            ),
            for_var_part(
                VariableKind::Var,
                vec![
                    VariableDecl {
                        id: Pat::Identifier("a4"),
                        init: Some(
                            zero_bin_expr_expr(
                                BinaryOperator::In,
                                array_expr(vec![])
                            )
                        )
                    }
                ],
                Some(number_literal_expr("0")),
                None,
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Expr(Expr::Ident("x")),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            for_in_loop_part(
                LoopLeft::Variable(
                    VariableKind::Var,
                    var_decl("x12")
                ),
                obj_literal_expr(vec![]),
                Stmt::Empty,
            ),
            ProgramPart::Stmt(
                Stmt::ForOf(
                    ForOfStmt {
                        left: LoopLeft::Expr(
                            Expr::Ident("x")
                        ),
                        right: array_expr(vec![]),
                        body: Box::new(Stmt::Empty),
                        is_await: false
                    }
                )
            ),
            ProgramPart::Stmt(
                Stmt::ForOf(
                    ForOfStmt {
                        left: LoopLeft::Variable(
                            VariableKind::Var,
                            var_decl("x13")
                        ),
                        right: array_expr(vec![]),
                        body: Box::new(Stmt::Empty),
                        is_await: false
                    }
                )
            ),
            for_exprs_part(None, Some(number_literal_expr("0")), None, Stmt::Continue(None)),
            labeled_statement_continue("x"),
            for_exprs_part(None, None, None, Stmt::Break(None)),
            preamble_labeled_statement("x"),
            switch_zero_part(vec![
                case_zero(vec![ProgramPart::Stmt(Stmt::Break(None))])
            ]),
            func_part(
                "f0",
                vec![],
                vec![ProgramPart::Stmt(Stmt::Return(None))]
            ),
            func_part(
                "f1",
                vec![],
                vec![ProgramPart::Stmt(Stmt::Return(
                    Some(number_literal_expr("0"))
                ))]
            ),
            switch_zero_part(vec![]),
            switch_zero_part(vec![
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![]),
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                default_case(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![]),
                default_case(vec![]),
                case_zero(vec![]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                case_zero(vec![
                    empty_part(),
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                default_case(vec![
                    empty_part(),
                ]),
            ]),
            switch_zero_part(vec![
                default_case(vec![
                    empty_part(),
                    empty_part(),
                ]),
            ]),
            labeled_part("x", Stmt::Empty),
            labeled_part(
                "x",
                labeled_stmt("y", Stmt::Empty)
            ),
            try_part(
                vec![
                    throw_part(
                        number_literal_expr("0")
                    )
                ],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                None
            ),
            try_part(
                vec![],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                None
            ),
            try_part(
                vec![],
                None,
                Some(vec![])
            ),
            try_part(
                vec![],
                Some(catch_(Some(Pat::Identifier("x")), vec![])),
                Some(vec![])
            ),
            ProgramPart::Stmt(Stmt::Debugger),
            func_part("f2", vec![], vec![]),
            func_part(
                "f3",
                vec![
                    fn_arg_ident_pat("x"),
                ],
                vec![]
            ),
            func_part(
                "f4",
                vec![
                    fn_arg_ident_pat("x"),
                    fn_arg_ident_pat("y"),
                ],
                vec![]
            ),
            func_part(
                "f5",
                vec![],
                vec![
                    func_part(
                        "f6",
                        vec![],
                        vec![]
                    ),
                ]
            ),
        block_part(vec![
            func_part("f7", vec![], vec![])
        ]),
        empty_part(),
        for_exprs_part(
            None,
            Some(number_literal_expr("0")),
            None,
            Stmt::Expr(
                Expr::Unary(
                    UnaryExpr {
                        operator: UnaryOperator::Plus,
                        prefix: true,
                        argument: Box::new(
                            func_expr("f8", vec![], vec![]),
                        )
                    }
                )
            )
        ),
        number_literal_part("0"),

        ProgramPart::Stmt(
            Stmt::DoWhile(
                DoWhileStmt {
                    body: Box::new(
                        Stmt::Expr(
                            Expr::Unary(
                                UnaryExpr {
                                    operator: UnaryOperator::Plus,
                                    argument: Box::new(func_expr("f9", vec![], vec![])),
                                    prefix: true,
                                }
                            )
                        )
                    ),
                    test: number_literal_expr("0")
                }
            )
        ),
        func_part("f10", long_args(), vec![]),
        func_part(
                "f11",
                vec![],
                vec![
                    directive_part("\"use strict\""),
                ],
            ),
            func_part(
                "f12",
                vec![],
                vec![
                    directive_part("'use strict'"),
                ],
            ),
            func_part(
                "f13",
                vec![],
                vec![
                    directive_part("\"other directive\""),
                ],
            ),
            func_part(
                "f14",
                vec![],
                vec![
                    directive_part("'other directive'"),
                ],
            ),
            func_part(
                "f15",
                vec![],
                vec![
                    string_literal_part("\"string\""),
                ],
            ),
            func_part(
                "f16",
                vec![],
                vec![
                    string_literal_part("'string'"),
                ],
            ),
            func_part(
                "f17",
                vec![],
                vec![
                    ProgramPart::Stmt(
                        Stmt::Expr(
                            Expr::Binary(
                                binary(
                                    string_literal_expr("'string'"),
                                    BinaryOperator::Plus,
                                    number_literal_expr("0")
                                )
                            )
                        )
                    ),
                ],
            ),
            ProgramPart::Decl(
                Decl::Function(
                    Function {
                        params: long_args(),
                        id: Some("g0"),
                        body: long_gen_body(),
                        generator: true,
                        is_async: false,
                    }
                )
            ),
            ProgramPart::Stmt(
                Stmt::Expr(
                    Expr::Call(
                        CallExpr {
                            callee: Box::new(
                                Expr::Function(
                                    Function {
                                        params: long_args(),
                                        id: Some("g1"),
                                        body: long_gen_body(),
                                        generator: true,
                                        is_async: false,
                                    }
                                )
                            ),
                            arguments: vec![
                                Expr::Function(
                                    anon_fn(vec![], vec![], false)
                                )
                            ]
                        }
                    )
                )
            ),
            func_expr_part(None, vec![
                fn_arg_ident_pat("x")
            ], vec![]),
            func_expr_part(None, vec![
                fn_arg_ident_pat("x"),
                fn_arg_ident_pat("y"),
            ], vec![]),
            func_expr_part(None, vec![], vec![
                func_part("f", vec![], vec![]),
            ]),
            func_expr_part(Some("f"), vec![], vec![]),
            func_expr_part(Some("f"), vec![
                fn_arg_ident_pat("x")
            ], vec![]),
            func_expr_part(Some("f"), vec![
                fn_arg_ident_pat("x"),
                fn_arg_ident_pat("y"),
            ], vec![]),
            func_expr_part(Some("f"), vec![], vec![
                func_part("f", vec![], vec![]),
            ]),
            arrow_expr_part(vec![], number_literal_expr("0")),
            arrow_expr_body_part(vec![],vec![
                empty_part()
            ]),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], Expr::Ident("x")),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], assign_expr(
                assign_left_ident("x"),
                number_literal_expr("0"))),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x")
            ], Expr::ArrowFunction(arrow_expr(vec![
                fn_arg_ident_expr("y")
            ], Expr::Ident("x")))),
            arrow_expr_body_part(vec![
                fn_arg_ident_expr("x")
            ], vec![
                ident_stmt("x")
            ]),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], obj_literal_expr(vec![
                    obj_prop(
                        obj_key_ident("x"),
                        PropertyValue::None,
                        PropertyKind::Init,
                        false,
                        false,
                        true
                    )
                ])
            ),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], Expr::Ident("x")),
            arrow_expr_body_part(vec![
                fn_arg_ident_expr("x"),
            ],vec![
                ProgramPart::Stmt(
                    return_ident_stmt("x")
                )
            ]),
            arrow_expr_part(vec![
                fn_arg_ident_expr("x"),
            ], obj_literal_expr(vec![
                    obj_prop(
                        obj_key_ident("x"),
                        PropertyValue::None,
                        PropertyKind::Init,
                        false,
                        false,
                        true
                    )
                ])
            ),
            arrow_expr_part(vec![
                FunctionArg::Expr(
                    Expr::Object(vec![
                        obj_prop(
                            obj_key_ident("x"),
                            PropertyValue::None,
                            PropertyKind::Init,
                            false,
                            false,
                            true,
                        )
                    ])
                ),
            ], obj_literal_expr(vec![
                    obj_prop(
                        obj_key_ident("x"),
                        PropertyValue::None,
                        PropertyKind::Init,
                        false,
                        false,
                        true
                    )
                ])
            ),
            arrow_expr_body_part(
                long_args_array_expr(),
                vec![
                    empty_part()
                ]
            ),
            assign_part(
                assign_left_expr(
                    array_expr(vec![
                        Some(Expr::Ident("a"))
                    ])
                ),
                array_expr(vec![
                    Some(Expr::Spread(
                        Box::new(
                            array_expr(vec![
                                Some(number_literal_expr("0"))
                            ])
                        )
                    ))
                ])
            ),
            assign_part(
                assign_left_expr(
                    obj_literal_expr(vec![
                        obj_prop(
                            obj_key_ident("a"),
                            PropertyValue::None,
                            PropertyKind::Init,
                            false,
                            false,
                            true,
                        )
                    ])
                ),
                obj_literal_expr(vec![])
            ),
            try_part(vec![], Some(
                CatchClause {
                    param: Some(
                        Pat::Array(vec![
                            Some(ArrayPatPart::Pat(
                                Pat::Identifier("e")
                            ))
                        ]
                        )
                    ),
                    body: vec![]
                }
            ), None),
            try_part(vec![], Some(
                CatchClause {
                    param: Some(
                        Pat::Object(vec![
                            ObjectPatPart::Assignment(
                                Property {
                                    key: obj_key_ident("e"),
                                    value: PropertyValue::None,
                                    kind: PropertyKind::Init,
                                    method: false,
                                    computed: false,
                                    short_hand: true,
                                    is_static: false,
                                }
                            )
                        ]
                        )
                    ),
                    body: vec![]
                }
            ), None),
            class_part(
                "A",
                None,
                vec![],
            ),
            class_part(
                "B",
                Some(new_expr(Expr::Ident("A"), vec![])),
                vec![
                    class_prop(
                        obj_key_ident("constructor"),
                        anon_fn(long_args(), vec![
                            call_part(Expr::Super, vec![
                                Expr::MetaProperty(
                                    MetaProperty {
                                        meta: "new",
                                        property: "target"
                                    }
                                )
                            ]),
                            ProgramPart::Stmt(
                                Stmt::Expr(
                                    Expr::TaggedTemplate(
                                        TaggedTemplateExpr {
                                            tag: Box::new(Expr::Call(
                                                CallExpr {
                                                    callee: Box::new(Expr::Super),
                                                    arguments: vec![]
                                                }
                                            )),
                                            quasi: template(vec![
                                                template_element("`template`", true),
                                            ], vec![])
                                        }
                                    )
                                )
                            ),
                            arrow_expr_part(vec![], call_expr(Expr::Super, vec![
                                Expr::This
                            ])),
                        ],
                        false,
                    ),
                    true,
                    false),
                    class_prop(
                        obj_key_ident("m"),
                        anon_fn(long_args(), vec![
                            call_part(
                                member_expr(
                                    Expr::Super,
                                    Expr::Ident("m"),
                                    false,
                                ),
                                vec![]
                            ),
                            ProgramPart::Stmt(
                                Stmt::Expr(
                                    Expr::TaggedTemplate(
                                        TaggedTemplateExpr {
                                            tag: Box::new(
                                                member_expr(Expr::Super, Expr::Ident("m"), false)
                                            ),
                                            quasi: template(vec![
                                                template_element("`template`", true),
                                            ], vec![])
                                        }
                                    )
                                )
                            ),
                            arrow_expr_part(vec![], call_expr(
                                member_expr(Expr::Super, Expr::Ident("m"), false),
                                vec![
                                    Expr::This
                                ]
                            )),
                        ], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("a"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true,
                    ),
                    class_prop(
                        obj_key_string("'b'"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true,
                    ),
                    class_prop(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true
                    ),
                    class_prop_computed(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_ident("c"),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_string("\"d\""),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop_computed(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false),
                        ], true),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_ident("var"),
                        anon_fn(vec![], vec![

                        ], false),
                        false,
                        true
                    ),
                    class_prop(
                        obj_key_ident("in"),
                        anon_fn(vec![], vec![
                        ], true),
                        false,
                        true
                    ),
                    class_prop_getter(
                        obj_key_ident("e"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_string("'f'"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        true,
                        true
                    ),
                    class_prop_setter(
                        obj_key_ident("g"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_string("\"h\""),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        true,
                    ),
                    class_prop_getter(
                        obj_key_ident("if"),
                        anon_fn(vec![], vec![], false),
                        true,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_ident("if"),
                        anon_fn(vec![
                            fn_arg_ident_pat("a")
                        ], vec![], false),
                        true,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("a"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_string("'b'"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_computed(
                        obj_key_number("0"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("c"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_string("\"d\""),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop_computed(
                        obj_key_number("1"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("var"),
                        anon_fn(vec![], vec![
                        ], false),
                        false,
                        false,
                    ),
                    class_prop(
                        obj_key_ident("in"),
                        anon_fn(vec![], vec![
                            yield_part(None, false)
                        ], true),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_ident("e"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_string("'f'"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_getter(
                        obj_key_number("2"),
                        anon_fn(vec![], vec![], false),
                        false,
                        true,
                    ),
                    class_prop_setter(
                        obj_key_ident("g"),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_string("\"h\""),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_number("3"),
                        anon_fn(vec![fn_arg_ident_pat("a")], vec![], false),
                        false,
                        true,
                    ),
                    class_prop_getter(
                        obj_key_ident("if"),
                        anon_fn(vec![], vec![
                        ], false),
                        false,
                        false,
                    ),
                    class_prop_setter(
                        obj_key_ident("if"),
                        anon_fn(vec![fn_arg_ident_pat("f")], vec![], false),
                        false,
                        false,
                    ),
                ]
            ),
            class_part("C", Some(
                Expr::Ident("B")
            ), vec![
                class_prop(obj_key_string("\"constructor\""), anon_fn(vec![], vec![
                    call_part(Expr::Super, vec![])
                ], false), true, false),
            ])
        // ProgramPart::Stmt(Stmt::Empty)
    ];
}

fn yield_part(e: Option<E>, d: bool) -> Part {
    ProgramPart::Stmt(Stmt::Expr(Expr::Yield(YieldExpr {
        delegate: d,
        argument: e.map(Box::new),
    })))
}
type Prop = Property<'static>;
fn class_part(i: &'static str, s: Option<E>, body: Vec<Prop>) -> Part {
    ProgramPart::Decl(class_decl(i, s, body))
}

fn class_decl(i: &'static str, s: Option<E>, body: Vec<Prop>) -> Decl<'static> {
    Decl::Class(Class {
        id: Some(i),
        super_class: s.map(Box::new),
        body,
    })
}

fn class_prop(key: PK, value: F, ctor: bool, is_static: bool) -> Prop {
    Property {
        key,
        value: PropertyValue::Expr(Expr::Function(value)),
        kind: if ctor {
            PropertyKind::Ctor
        } else {
            PropertyKind::Method
        },
        method: true,
        short_hand: false,
        computed: false,
        is_static,
    }
}

fn class_prop_computed(key: PK, value: F, ctor: bool, is_static: bool) -> Property<'static> {
    Property {
        key,
        value: PropertyValue::Expr(Expr::Function(value)),
        kind: if ctor {
            PropertyKind::Ctor
        } else {
            PropertyKind::Method
        },
        method: true,
        short_hand: false,
        computed: true,
        is_static,
    }
}

fn class_prop_getter(key: PK, value: F, is_static: bool, computed: bool) -> Property<'static> {
    Property {
        key,
        value: PropertyValue::Expr(Expr::Function(value)),
        kind: PropertyKind::Get,
        method: false,
        short_hand: false,
        computed,
        is_static,
    }
}
fn class_prop_setter(key: PK, value: F, is_static: bool, computed: bool) -> Property<'static> {
    Property {
        key,
        value: PropertyValue::Expr(Expr::Function(value)),
        kind: PropertyKind::Set,
        method: false,
        short_hand: false,
        computed,
        is_static,
    }
}

fn long_gen_body() -> Vec<Part> {
    vec![ProgramPart::Stmt(Stmt::Return(Some(Expr::Assignment(
        AssignmentExpr {
            left: assign_left_ident("a"),
            right: Box::new(Expr::Yield(YieldExpr {
                delegate: true,
                argument: Some(Box::new(Expr::Assignment(AssignmentExpr {
                    left: assign_left_ident("b"),
                    right: Box::new(Expr::Yield(YieldExpr {
                        delegate: false,
                        argument: Some(Box::new(Expr::Assignment(AssignmentExpr {
                            left: assign_left_ident("c"),
                            operator: AssignmentOperator::Equal,
                            right: Box::new(Expr::Yield(YieldExpr {
                                delegate: false,
                                argument: Some(Box::new(Expr::Yield(YieldExpr {
                                    delegate: false,
                                    argument: None,
                                }))),
                            })),
                        }))),
                    })),
                    operator: AssignmentOperator::Equal,
                }))),
            })),
            operator: AssignmentOperator::Equal,
        },
    ))))]
}

fn empty_generator_prop(key: PK) -> OP {
    obj_prop(
        key,
        PropertyValue::Expr(Expr::Function(Function {
            id: None,
            params: vec![],
            body: vec![],
            generator: true,
            is_async: false,
        })),
        PropertyKind::Init,
        true,
        false,
        false,
    )
}

fn long_args_array_pat() -> Vec<P> {
    vec![
        Pat::Identifier("a"),
        Pat::Assignment(AssignmentPat {
            left: Box::new(Pat::Identifier("b")),
            right: Box::new(number_literal_expr("0")),
        }),
        Pat::Array(vec![
            Some(ArrayPatPart::Pat(Pat::Identifier("c"))),
            None,
            Some(ArrayPatPart::Pat(Pat::Assignment(AssignmentPat {
                left: Box::new(Pat::Identifier("d")),
                right: Box::new(number_literal_expr("0")),
            }))),
            Some(ArrayPatPart::Pat(Pat::RestElement(Box::new(
                Pat::Identifier("e"),
            )))),
        ]),
        Pat::Object(vec![
            ObjectPatPart::Assignment(Property {
                key: obj_key_ident("f"),
                value: PropertyValue::None,
                kind: PropertyKind::Init,
                method: false,
                computed: false,
                short_hand: true,
                is_static: false,
            }),
            // g: h
            ObjectPatPart::Assignment(Property {
                key: obj_key_ident("g"),
                value: PropertyValue::Pat(Pat::Identifier("h")),
                kind: PropertyKind::Init,
                method: false,
                computed: false,
                short_hand: false,
                is_static: false,
            }),
            //i = 0,
            ObjectPatPart::Assignment(Property {
                key: obj_key_ident("i"),
                value: obj_value_number("0"),
                kind: PropertyKind::Init,
                method: false,
                computed: false,
                short_hand: true,
                is_static: false,
            }),
            // i: j = 0
            ObjectPatPart::Assignment(Property {
                key: obj_key_ident("i"),
                value: PropertyValue::Pat(Pat::Assignment(AssignmentPat {
                    left: Box::new(Pat::Identifier("j")),
                    right: Box::new(number_literal_expr("0")),
                })),
                kind: PropertyKind::Init,
                method: false,
                short_hand: false,
                computed: false,
                is_static: false,
            }),
        ]),
        Pat::RestElement(Box::new(Pat::Identifier("k"))),
    ]
}
fn long_args_array_expr() -> Vec<FA> {
    vec![
        FunctionArg::Expr(Expr::Ident("a")),
        FunctionArg::Expr(Expr::Assignment(AssignmentExpr {
            operator: AssignmentOperator::Equal,
            left: AssignmentLeft::Expr(Box::new(Expr::Ident("b"))),
            right: Box::new(number_literal_expr("0")),
        })),
        FunctionArg::Expr(array_expr(vec![
            Some(Expr::Ident("c")),
            None,
            Some(Expr::Assignment(AssignmentExpr {
                operator: AssignmentOperator::Equal,
                left: AssignmentLeft::Expr(Box::new(Expr::Ident("d"))),
                right: Box::new(number_literal_expr("0")),
            })),
            Some(Expr::Spread(Box::new(Expr::Ident("e")))),
        ])),
        FunctionArg::Expr(Expr::Object(vec![
            obj_prop(
                obj_key_ident("f"),
                PropertyValue::None,
                PropertyKind::Init,
                false,
                false,
                true,
            ),
            obj_prop(
                obj_key_ident("g"),
                PropertyValue::Expr(Expr::Ident("h")),
                PropertyKind::Init,
                false,
                false,
                false,
            ),
            obj_prop(
                obj_key_ident("i"),
                obj_value_number("0"),
                PropertyKind::Init,
                false,
                false,
                true,
            ),
            obj_prop(
                obj_key_ident("i"),
                PropertyValue::Expr(Expr::Assignment(AssignmentExpr {
                    operator: AssignmentOperator::Equal,
                    left: AssignmentLeft::Expr(Box::new(Expr::Ident("j"))),
                    right: Box::new(number_literal_expr("0")),
                })),
                PropertyKind::Init,
                false,
                false,
                false,
            ),
        ])),
        FunctionArg::Pat(Pat::RestElement(Box::new(Pat::Identifier("k")))),
    ]
}

fn long_args() -> Vec<FA> {
    long_args_array_pat()
        .into_iter()
        .map(FunctionArg::Pat)
        .collect()
}

fn directive_part(dir: &'static str) -> Part {
    ProgramPart::Dir(Dir {
        expr: string_literal(dir),
        dir: &dir[1..dir.len() - 1],
    })
}

fn empty_part() -> Part {
    ProgramPart::Stmt(Stmt::Empty)
}

fn preamble_labeled_statement(label: &'static str) -> ProgramPart {
    ProgramPart::Stmt(Stmt::Labeled(LabeledStmt {
        label,
        body: Box::new(Stmt::For(ForStmt {
            init: None,
            test: None,
            update: None,
            body: Box::new(Stmt::Break(Some(label))),
        })),
    }))
}
fn labeled_statement_continue(label: &'static str) -> ProgramPart {
    ProgramPart::Stmt(Stmt::Labeled(LabeledStmt {
        label,
        body: Box::new(Stmt::For(ForStmt {
            init: None,
            test: Some(number_literal_expr("0")),
            update: None,
            body: Box::new(Stmt::Continue(Some(label))),
        })),
    }))
}

fn labeled_part(label: &'static str, body: S) -> Part {
    ProgramPart::Stmt(labeled_stmt(label, body))
}

fn labeled_stmt(label: &'static str, body: S) -> S {
    Stmt::Labeled(labeled(label, body))
}

fn labeled(label: &'static str, body: S) -> LabeledStmt {
    LabeledStmt {
        label,
        body: Box::new(body),
    }
}

fn line_term(label: &str) -> ProgramPart {
    ProgramPart::Stmt(Stmt::Labeled(LabeledStmt {
        label,
        body: Box::new(Stmt::Expr(Expr::Literal(Literal::Number("0")))),
    }))
}

fn number_literal_part(number: &'static str) -> ProgramPart {
    ProgramPart::Stmt(number_literal_stmt(number))
}

fn number_literal_stmt(number: &'static str) -> Stmt<'static> {
    Stmt::Expr(number_literal_expr(number))
}

fn number_literal_expr(number: &'static str) -> Expr<'static> {
    Expr::Literal(number_literal(number))
}

fn number_literal(number: &'static str) -> Literal<'static> {
    Literal::Number(number)
}

fn null_literal_part() -> ProgramPart<'static> {
    ProgramPart::Stmt(null_literal_stmt())
}

fn null_literal_stmt() -> Stmt<'static> {
    Stmt::Expr(null_literal_expr())
}

fn null_literal_expr() -> Expr<'static> {
    Expr::Literal(null_literal())
}

fn null_literal() -> Literal<'static> {
    Literal::Null
}

fn bool_literal_part(b: bool) -> ProgramPart<'static> {
    ProgramPart::Stmt(bool_literal_stmt(b))
}

fn bool_literal_stmt(b: bool) -> Stmt<'static> {
    Stmt::Expr(bool_literal_expr(b))
}
fn bool_literal_expr(b: bool) -> Expr<'static> {
    Expr::Literal(bool_literal(b))
}

fn bool_literal(b: bool) -> Literal<'static> {
    Literal::Boolean(b)
}

fn var_decl_one() -> ProgramPart<'static> {
    ProgramPart::Decl(Decl::Variable(
        VariableKind::Var,
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
        ]),
    ))
}

fn var_decl_one_2015() -> Part {
    ProgramPart::Decl(Decl::Variable(
        VariableKind::Var,
        var_decls(&[
            r"$",
            r"_",
            r"\u0078",
            r"\u{2F9F9}",
            r"x$",
            r"x_",
            r"x\u0030",
            r"x\u{e01d5}",
            r"xa",
            r"x0",
            r"x0a",
            r"x0123456789",
            r"qwertyuiopasdfghjklzxcvbnm",
            r"QWERTYUIOPASDFGHJKLZXCVBNM",
        ]),
    ))
}

fn var_decl_two() -> ProgramPart<'static> {
    ProgramPart::Decl(Decl::Variable(
        VariableKind::Var,
        var_decls(&[
            r"œ一",
            r"ǻ둘",
            r"ɤ〩",
            r"φ",
            r"ﬁⅷ",
            r"ユニコード",
            r"x‌‍",
        ]),
    ))
}
fn var_decl_two_2015() -> ProgramPart<'static> {
    ProgramPart::Decl(Decl::Variable(
        VariableKind::Var,
        var_decls(&[
            r"䩶",
            r"x󠇕",
            r"œ一",
            r"ǻ둘",
            r"ɤ〩",
            r"φ",
            r"ﬁⅷ",
            r"ユニコード",
            r"x‌‍",
        ]),
    ))
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

fn var_decl_init(id: &'static str, init: E) -> VariableDecl<'static> {
    VariableDecl {
        id: Pat::Identifier(id),
        init: Some(init),
    }
}

fn string_literal_part(s: &'static str) -> Part {
    ProgramPart::Stmt(string_literal_stmt(s))
}

fn string_literal_stmt(s: &'static str) -> S {
    Stmt::Expr(string_literal_expr(s))
}

fn string_literal_expr(s: &'static str) -> E {
    Expr::Literal(string_literal(s))
}

fn string_literal(s: &'static str) -> L {
    Literal::String(s)
}

fn regex_literal_part(pattern: &'static str, flags: &'static str) -> ProgramPart<'static> {
    ProgramPart::Stmt(regex_literal_stmt(pattern, flags))
}

fn regex_literal_stmt(pattern: &'static str, flags: &'static str) -> Stmt<'static> {
    Stmt::Expr(regex_literal_expr(pattern, flags))
}

fn regex_literal_expr(pattern: &'static str, flags: &'static str) -> Expr<'static> {
    Expr::Literal(regex_literal(pattern, flags))
}

fn regex_literal(pattern: &'static str, flags: &'static str) -> Literal<'static> {
    Literal::RegEx(regex(pattern, flags))
}

fn regex(pattern: &'static str, flags: &'static str) -> RegEx<'static> {
    RegEx { pattern, flags }
}

fn this_part() -> ProgramPart<'static> {
    ProgramPart::Stmt(this_stmt())
}

fn this_stmt() -> Stmt<'static> {
    Stmt::Expr(this_expr())
}

fn this_expr() -> Expr<'static> {
    Expr::This
}

fn ident_stmt(id: &'static str) -> ProgramPart<'static> {
    ProgramPart::Stmt(Stmt::Expr(Expr::Ident(id)))
}

fn array(content: Vec<Option<Expr<'static>>>) -> ProgramPart<'static> {
    ProgramPart::Stmt(Stmt::Expr(array_expr(content)))
}

fn array_expr(content: Vec<Option<E>>) -> E {
    Expr::Array(content)
}

fn obj_literal_part(content: Vec<OP>) -> Part {
    ProgramPart::Stmt(obj_literal_stmt(content))
}

fn obj_literal_stmt(content: Vec<OP>) -> S {
    Stmt::Expr(obj_literal_expr(content))
}

fn obj_literal_expr(content: Vec<OP>) -> E {
    Expr::Object(content)
}

type PK = PropertyKey<'static>;
type PV = PropertyValue<'static>;
fn obj_prop_ident_number(ident: &'static str, number: &'static str) -> OP {
    obj_prop(
        obj_key_ident(ident),
        obj_value_number(number),
        PropertyKind::Init,
        false,
        false,
        false,
    )
}
fn obj_prop_str_number(s: &'static str, number: &'static str) -> OP {
    obj_prop(
        obj_key_string(s),
        obj_value_number(number),
        PropertyKind::Init,
        false,
        false,
        false,
    )
}
fn obj_prop_number_number(n: &'static str, number: &'static str) -> OP {
    obj_prop(
        obj_key_number(n),
        obj_value_number(number),
        PropertyKind::Init,
        false,
        false,
        false,
    )
}
type F = Function<'static>;
type FA = FunctionArg<'static>;
fn empty_anon_fn(args: Vec<FA>) -> F {
    Function {
        id: None,
        generator: false,
        is_async: false,
        body: vec![],
        params: args,
    }
}

fn anon_fn(args: Vec<FA>, body: Vec<Part>, gen: bool) -> F {
    Function {
        id: None,
        generator: gen,
        is_async: false,
        body,
        params: args,
    }
}

fn func_part(id: &'static str, args: Vec<FA>, body: Vec<Part>) -> Part {
    ProgramPart::Decl(Decl::Function(func(id, args, body)))
}

fn func_expr_part(id: Option<&'static str>, args: Vec<FA>, body: Vec<Part>) -> Part {
    let f = if let Some(id) = id {
        func(id, args, body)
    } else {
        Function {
            id: None,
            params: args,
            body,
            generator: false,
            is_async: false,
        }
    };
    ProgramPart::Stmt(Stmt::Expr(Expr::Function(f)))
}

fn func_expr(id: &'static str, args: Vec<FA>, body: Vec<Part>) -> E {
    Expr::Function(func(id, args, body))
}

fn func(id: &'static str, args: Vec<FA>, body: Vec<Part>) -> F {
    Function {
        id: Some(id),
        generator: false,
        is_async: false,
        body,
        params: args,
    }
}
fn func_generator(id: &'static str, args: Vec<FA>, body: Vec<Part>) -> F {
    Function {
        id: Some(id),
        generator: true,
        is_async: false,
        body,
        params: args,
    }
}

fn fn_arg_ident_pat(i: &'static str) -> FA {
    FunctionArg::Pat(Pat::Identifier(i))
}
fn fn_arg_ident_expr(i: &'static str) -> FA {
    FunctionArg::Expr(Expr::Ident(i))
}

fn obj_prop_ident_fn(i: &'static str, f: Function<'static>, kind: PropertyKind) -> OP {
    obj_prop(obj_key_ident(i), obj_value_fn(f), kind, true, false, false)
}
fn obj_prop_str_fn(i: &'static str, f: Function<'static>, kind: PropertyKind) -> OP {
    obj_prop(obj_key_string(i), obj_value_fn(f), kind, true, false, false)
}
fn obj_prop_number_fn(n: &'static str, f: Function<'static>, kind: PropertyKind) -> OP {
    obj_prop(obj_key_number(n), obj_value_fn(f), kind, true, false, false)
}
fn obj_prop_ident_getter(i: &'static str) -> OP {
    obj_prop(
        obj_key_ident(i),
        obj_value_fn(empty_anon_fn(vec![])),
        PropertyKind::Get,
        false,
        false,
        false,
    )
}

fn obj_prop_str_getter(s: &'static str) -> OP {
    obj_prop(
        obj_key_string(s),
        obj_value_fn(empty_anon_fn(vec![])),
        PropertyKind::Get,
        false,
        false,
        false,
    )
}
fn obj_prop_number_getter(n: &'static str) -> OP {
    obj_prop(
        obj_key_number(n),
        obj_value_fn(empty_anon_fn(vec![])),
        PropertyKind::Get,
        false,
        false,
        false,
    )
}

fn obj_prop_ident_setter(i: &'static str, args: Vec<FA>) -> OP {
    obj_prop(
        obj_key_ident(i),
        obj_value_fn(empty_anon_fn(args)),
        PropertyKind::Set,
        false,
        false,
        false,
    )
}
fn obj_prop_str_setter(i: &'static str, args: Vec<FA>) -> OP {
    obj_prop(
        obj_key_string(i),
        obj_value_fn(empty_anon_fn(args)),
        PropertyKind::Set,
        false,
        false,
        false,
    )
}
fn obj_prop_number_setter(i: &'static str, args: Vec<FA>) -> OP {
    obj_prop(
        obj_key_number(i),
        obj_value_fn(empty_anon_fn(args)),
        PropertyKind::Set,
        false,
        false,
        false,
    )
}
fn obj_prop(
    key: PK,
    value: PV,
    kind: PropertyKind,
    method: bool,
    computed: bool,
    short_hand: bool,
) -> OP {
    ObjectProperty::Property(Property {
        key,
        value,
        kind,
        computed,
        method,
        short_hand,
        is_static: false,
    })
}
fn obj_value_number(n: &'static str) -> PV {
    PropertyValue::Expr(number_literal_expr(n))
}
fn obj_value_fn(f: Function<'static>) -> PV {
    PropertyValue::Expr(Expr::Function(f))
}
fn obj_key_ident(i: &'static str) -> PK {
    obj_prop_key_expr(Expr::Ident(i))
}
fn obj_prop_key_expr(expr: E) -> PK {
    PropertyKey::Expr(expr)
}
fn obj_key_number(n: &'static str) -> PK {
    obj_prop_key_lit(number_literal(n))
}
fn obj_key_string(s: &'static str) -> PK {
    obj_prop_key_lit(string_literal(s))
}

fn obj_prop_key_lit(lit: L) -> PK {
    PropertyKey::Literal(lit)
}
type P = Pat<'static>;
fn member_number_ident_part(n: &'static str, i: &'static str) -> Part {
    ProgramPart::Stmt(member_number_ident_stmt(n, i))
}
fn member_number_number_part(n: &'static str, i: &'static str) -> Part {
    ProgramPart::Stmt(member_number_number_stmt(n, i))
}
fn member_number_ident_stmt(n: &'static str, i: &'static str) -> S {
    Stmt::Expr(member_number_ident_expr(n, i))
}
fn member_number_number_stmt(n: &'static str, i: &'static str) -> S {
    Stmt::Expr(member_number_number_expr(n, i))
}
fn member_number_ident_expr(n: &'static str, i: &'static str) -> E {
    Expr::Member(member_number_ident(n, i))
}
fn member_number_number_expr(n: &'static str, i: &'static str) -> E {
    Expr::Member(member_number_number(n, i))
}
fn member_ident_number_expr(i: &'static str, n: &'static str) -> E {
    Expr::Member(member_ident_number(i, n))
}
fn member_ident_ident_expr(i: &'static str, i2: &'static str) -> E {
    Expr::Member(member_ident_ident(i, i2))
}

fn member_number_ident(n: &'static str, i: &'static str) -> MemberExpr<'static> {
    member(number_literal_expr(n), Expr::Ident(i), false)
}
fn member_number_number(n: &'static str, i: &'static str) -> MemberExpr<'static> {
    member(number_literal_expr(n), number_literal_expr(i), true)
}
fn member_ident_number(i: &'static str, n: &'static str) -> MemberExpr<'static> {
    member(Expr::Ident(i), number_literal_expr(n), true)
}
fn member_ident_ident(i: &'static str, i2: &'static str) -> MemberExpr<'static> {
    member(Expr::Ident(i), Expr::Ident(i2), false)
}

fn member_expr(obj: E, prop: E, computed: bool) -> E {
    Expr::Member(member(obj, prop, computed))
}

fn member(obj: E, prop: E, computed: bool) -> MemberExpr<'static> {
    MemberExpr {
        computed,
        object: Box::new(obj),
        property: Box::new(prop),
    }
}

fn assign_part(left: AssignmentLeft<'static>, right: E) -> Part {
    ProgramPart::Stmt(assign_stmt(left, right))
}

fn assign_stmt(left: AssignmentLeft<'static>, right: E) -> S {
    Stmt::Expr(assign_expr(left, right))
}
fn assign_expr(left: AssignmentLeft<'static>, right: E) -> E {
    Expr::Assignment(assign(left, right))
}

fn assign(left: AssignmentLeft<'static>, right: E) -> AssignmentExpr<'static> {
    AssignmentExpr {
        left: left,
        operator: AssignmentOperator::Equal,
        right: Box::new(right),
    }
}

fn assign_left_ident(i: &'static str) -> AssignmentLeft<'static> {
    assign_left_expr(Expr::Ident(i))
}

fn assign_left_expr(e: E) -> AssignmentLeft<'static> {
    AssignmentLeft::Expr(Box::new(e))
}

fn return_ident_part(ident: &'static str) -> Part {
    ProgramPart::Stmt(return_ident_stmt(ident))
}

fn return_ident_stmt(ident: &'static str) -> S {
    Stmt::Return(Some(Expr::Ident(ident)))
}

fn new_ident_part(i: &'static str) -> Part {
    ProgramPart::Stmt(new_stmt(Expr::Ident(i), vec![]))
}

fn new_part(c: E, args: Vec<E>) -> Part {
    ProgramPart::Stmt(new_stmt(c, args))
}

fn new_stmt(c: E, args: Vec<E>) -> S {
    Stmt::Expr(new_expr(c, args))
}
fn new_expr(c: E, args: Vec<E>) -> E {
    Expr::New(new(c, args))
}

fn new(c: E, args: Vec<E>) -> NewExpr<'static> {
    NewExpr {
        callee: Box::new(c),
        arguments: args,
    }
}

fn new_ident_expr(i: &'static str, args: Vec<E>) -> E {
    Expr::New(new_ident(i, args))
}

fn new_ident(i: &'static str, args: Vec<E>) -> NewExpr<'static> {
    NewExpr {
        callee: Box::new(Expr::Ident(i)),
        arguments: args,
    }
}

fn call_ident_part(i: &'static str, args: Vec<E>) -> Part {
    call_part(Expr::Ident(i), args)
}

fn call_part(callee: E, args: Vec<E>) -> Part {
    ProgramPart::Stmt(Stmt::Expr(Expr::Call(call(callee, args))))
}

fn call_expr(callee: E, args: Vec<E>) -> E {
    Expr::Call(call(callee, args))
}

fn call_ident_expr(i: &'static str, args: Vec<E>) -> E {
    Expr::Call(call_ident(i, args))
}

fn call_ident(i: &'static str, args: Vec<E>) -> CallExpr<'static> {
    call(Expr::Ident(i), args)
}

fn call(callee: E, args: Vec<E>) -> CallExpr<'static> {
    CallExpr {
        callee: Box::new(callee),
        arguments: args,
    }
}

fn update_part(e: E, op: UpdateOperator, prefix: bool) -> Part {
    ProgramPart::Stmt(update_stmt(e, op, prefix))
}
fn update_stmt(e: E, op: UpdateOperator, prefix: bool) -> S {
    Stmt::Expr(update_expr(e, op, prefix))
}

fn update_expr(e: E, op: UpdateOperator, prefix: bool) -> E {
    Expr::Update(UpdateExpr {
        argument: Box::new(e),
        operator: op,
        prefix,
    })
}

fn unary_part(op: UnaryOperator, e: E, prefix: bool) -> Part {
    ProgramPart::Stmt(unary_stmt(op, e, prefix))
}

fn unary_stmt(op: UnaryOperator, e: E, prefix: bool) -> S {
    Stmt::Expr(unary_expr(op, e, prefix))
}

fn unary_expr(op: UnaryOperator, e: E, prefix: bool) -> E {
    Expr::Unary(unary(op, e, prefix))
}

fn unary(op: UnaryOperator, e: E, prefix: bool) -> UnaryExpr<'static> {
    UnaryExpr {
        operator: op,
        argument: Box::new(e),
        prefix,
    }
}

fn zero_bin_zero_part(op: BinaryOperator) -> Part {
    ProgramPart::Stmt(Stmt::Expr(Expr::Binary(binary(
        number_literal_expr("0"),
        op,
        number_literal_expr("0"),
    ))))
}

fn ident_assign_zero_part(i: &'static str, op: AssignmentOperator) -> Part {
    ProgramPart::Stmt(Stmt::Expr(ident_assign_zero_expr(i, op)))
}

fn ident_assign_zero_expr(i: &'static str, op: AssignmentOperator) -> Expr {
    Expr::Assignment(ident_assign_zero(i, op))
}

fn ident_assign_zero(i: &'static str, op: AssignmentOperator) -> AssignmentExpr {
    AssignmentExpr {
        left: AssignmentLeft::Expr(Box::new(Expr::Ident(i))),
        operator: op,
        right: Box::new(number_literal_expr("0")),
    }
}

fn zero_bin_expr_part(op: BinaryOperator, e: E) -> Part {
    ProgramPart::Stmt(Stmt::Expr(zero_bin_expr_expr(op, e)))
}

fn zero_bin_expr_expr(op: BinaryOperator, e: E) -> E {
    Expr::Binary(binary(number_literal_expr("0"), op, e))
}

fn binary(l: E, op: BinaryOperator, r: E) -> BinaryExpr<'static> {
    BinaryExpr {
        operator: op,
        left: Box::new(l),
        right: Box::new(r),
    }
}

fn zero_log_zero(op: LogicalOperator) -> Part {
    ProgramPart::Stmt(Stmt::Expr(zero_log_zero_expr(op)))
}
fn zero_log_zero_expr(op: LogicalOperator) -> E {
    Expr::Logical(logical(
        number_literal_expr("0"),
        op,
        number_literal_expr("0"),
    ))
}

fn logical(l: E, op: LogicalOperator, r: E) -> LogicalExpr<'static> {
    LogicalExpr {
        operator: op,
        left: Box::new(l),
        right: Box::new(r),
    }
}

fn conditional_part(t: E, c: E, a: E) -> Part {
    ProgramPart::Stmt(Stmt::Expr(conditional_expr(t, c, a)))
}

fn conditional_expr(t: E, c: E, a: E) -> E {
    Expr::Conditional(conditional(t, c, a))
}

fn conditional(t: E, c: E, a: E) -> ConditionalExpr<'static> {
    ConditionalExpr {
        test: Box::new(t),
        consequent: Box::new(c),
        alternate: Box::new(a),
    }
}

fn zero_sequence(ct: usize) -> Part {
    let mut seq = vec![];
    for _ in 0..ct {
        seq.push(number_literal_expr("0"))
    }
    sequence(seq)
}

fn sequence(seq: Vec<E>) -> Part {
    ProgramPart::Stmt(Stmt::Expr(Expr::Sequence(seq)))
}

fn block_part(body: Vec<Part>) -> Part {
    ProgramPart::Stmt(Stmt::Block(body))
}

fn variable_decl_part(kind: VariableKind, decls: Vec<VariableDecl<'static>>) -> Part {
    ProgramPart::Decl(Decl::Variable(kind, decls))
}

fn variable_decl_none(ident: &'static str) -> VariableDecl<'static> {
    VariableDecl {
        id: Pat::Identifier(ident),
        init: None,
    }
}
fn variable_decl_zero(ident: &'static str) -> VariableDecl<'static> {
    variable_decl(ident, number_literal_expr("0"))
}
fn variable_decl(ident: &'static str, e: E) -> VariableDecl<'static> {
    VariableDecl {
        id: Pat::Identifier(ident),
        init: Some(e),
    }
}

fn if_zero_empty() -> Part {
    ProgramPart::Stmt(Stmt::If(if_stmt(
        number_literal_expr("0"),
        Stmt::Empty,
        None,
    )))
}

fn if_zero_empty_else() -> Part {
    ProgramPart::Stmt(Stmt::If(if_stmt(
        number_literal_expr("0"),
        Stmt::Empty,
        Some(Stmt::Empty),
    )))
}

fn if_stmt(t: E, c: S, a: Option<S>) -> IfStmt<'static> {
    IfStmt {
        test: t,
        consequent: Box::new(c),
        alternate: a.map(|a| Box::new(a)),
    }
}

fn do_while_zero() -> Part {
    ProgramPart::Stmt(Stmt::DoWhile(DoWhileStmt {
        test: number_literal_expr("0"),
        body: Box::new(Stmt::Empty),
    }))
}

fn while_zero() -> Part {
    ProgramPart::Stmt(Stmt::While(WhileStmt {
        test: number_literal_expr("0"),
        body: Box::new(Stmt::Empty),
    }))
}

fn for_exprs_part(init: Option<E>, test: Option<E>, update: Option<E>, body: S) -> Part {
    ProgramPart::Stmt(Stmt::For(for_(
        init.map(LoopInit::Expr),
        test,
        update,
        body,
    )))
}

fn for_var_part(
    kind: VariableKind,
    init: Vec<VariableDecl<'static>>,
    test: Option<E>,
    update: Option<E>,
    body: S,
) -> Part {
    ProgramPart::Stmt(Stmt::For(for_(
        Some(LoopInit::Variable(kind, init)),
        test,
        update,
        body,
    )))
}

fn for_(
    init: Option<LoopInit<'static>>,
    test: Option<E>,
    update: Option<E>,
    body: S,
) -> ForStmt<'static> {
    ForStmt {
        init,
        test,
        update,
        body: Box::new(body),
    }
}

fn for_in_loop_part(left: LoopLeft<'static>, right: E, body: S) -> Part {
    ProgramPart::Stmt(Stmt::ForIn(for_in_loop(left, right, body)))
}

fn for_in_loop(left: LoopLeft<'static>, right: E, body: S) -> ForInStmt<'static> {
    ForInStmt {
        left,
        right,
        body: Box::new(body),
    }
}

fn switch_zero_part(cases: Vec<SwitchCase<'static>>) -> Part {
    ProgramPart::Stmt(Stmt::Switch(switch(number_literal_expr("0"), cases)))
}

fn switch(test: E, body: Vec<SwitchCase<'static>>) -> SwitchStmt<'static> {
    SwitchStmt {
        discriminant: test,
        cases: body,
    }
}

fn case_zero(body: Vec<Part>) -> SwitchCase<'static> {
    switch_case(Some(number_literal_expr("0")), body)
}

fn default_case(body: Vec<Part>) -> SwitchCase<'static> {
    switch_case(None, body)
}

fn switch_case(test: Option<E>, body: Vec<Part>) -> SwitchCase<'static> {
    SwitchCase {
        test,
        consequent: body,
    }
}

fn try_part(
    block: Vec<Part>,
    handler: Option<CatchClause<'static>>,
    finalizer: Option<Vec<Part>>,
) -> Part {
    ProgramPart::Stmt(Stmt::Try(try_(block, handler, finalizer)))
}

fn try_(
    block: Vec<Part>,
    handler: Option<CatchClause<'static>>,
    finalizer: Option<Vec<Part>>,
) -> TryStmt<'static> {
    TryStmt {
        block,
        handler,
        finalizer,
    }
}

fn catch_(param: Option<P>, body: Vec<Part>) -> CatchClause<'static> {
    CatchClause { param, body }
}

fn throw_part(e: E) -> Part {
    ProgramPart::Stmt(throw_stmt(e))
}

fn throw_stmt(e: E) -> S {
    Stmt::Throw(e)
}

fn template_part(quasis: Vec<TemplateElement<'static>>, exprs: Vec<E>) -> Part {
    ProgramPart::Stmt(Stmt::Expr(template_expr(quasis, exprs)))
}

fn template_expr(quasis: Vec<TemplateElement<'static>>, exprs: Vec<E>) -> E {
    Expr::Literal(Literal::Template(template(quasis, exprs)))
}

fn template(quasis: Vec<TemplateElement<'static>>, exprs: Vec<E>) -> TemplateLiteral<'static> {
    TemplateLiteral {
        quasis,
        expressions: exprs,
    }
}

fn template_element(raw: &'static str, tail: bool) -> TemplateElement<'static> {
    let end = if tail { raw.len() - 1 } else { raw.len() - 2 };
    TemplateElement {
        raw,
        cooked: &raw[1..end],
        tail,
    }
}

fn arrow_expr_body_part(args: Vec<FA>, body: Vec<Part>) -> Part {
    ProgramPart::Stmt(Stmt::Expr(Expr::ArrowFunction(arrow_func(
        None,
        args,
        ArrowFunctionBody::FunctionBody(body),
        false,
        false,
        false,
    ))))
}

fn arrow_expr_part(args: Vec<FA>, body: E) -> Part {
    ProgramPart::Stmt(Stmt::Expr(Expr::ArrowFunction(arrow_expr(args, body))))
}

fn arrow_expr(args: Vec<FA>, body: E) -> ArrowFunctionExpr<'static> {
    arrow_func(
        None,
        args,
        ArrowFunctionBody::Expr(Box::new(body)),
        true,
        false,
        false,
    )
}

type FAB = ArrowFunctionBody<'static>;
fn arrow_func(
    id: Option<&'static str>,
    params: Vec<FA>,
    body: FAB,
    expr: bool,
    gen: bool,
    a: bool,
) -> ArrowFunctionExpr<'static> {
    ArrowFunctionExpr {
        id,
        params,
        body,
        expression: expr,
        generator: gen,
        is_async: a,
    }
}
