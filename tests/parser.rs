use asalang::*;
use asalang::Node::*;

macro_rules! test {
  ($func:ident, $input:tt, $combinator:tt, $test:expr) => (
    #[test]
    fn $func() -> Result<(),()> {
      let source = $input;
      let tokens = lex(source);
      let parse_result = $combinator(tokens);
      //println!("{:?}",parse_result );
      match parse_result {
        Ok((tokens,tree)) => {
          assert_eq!(tokens.is_done(),true);
          assert_eq!(tree,$test)
        },
        _ => {assert!(false)},
      }
      Ok(())
    }
  )
}
// test name, test string, combinator,  expected result
test!(parser_ident, r#"hello"#, identifier, Identifier{value: vec![104, 101, 108, 108, 111]});
test!(parser_number, r#"123"#, number, Number{value: 123});
test!(parser_bool, r#"true"#, boolean, Bool{value: true});
test!(parser_string, r#""hello""#, string, String{value: "hello".to_string()});
test!(parser_function_call, r#"foo()"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
  ]}
]});
test!(parser_function_call_one_arg, r#"foo(a)"#, function_call, FunctionCall{name: vec![102, 111, 111], children: vec![
  FunctionArguments{ children: vec![
    Expression { children: vec![Identifier { value: vec![97] }]}
  ]}
]});
test!(parser_variable_define_number, r#"let a = 123"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Number{value: 123 }]}
]});
test!(parser_variable_define_bool, r#"let a = true"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![Bool{value: true}]}
]});
test!(parser_math_expr, r#"1+1"#, math_expression, MathExpression {name: vec![97, 100, 100], children: vec![
  Number{value: 1},
  Number{value: 1}
]});
test!(parser_variable_define_math_expr, r#"let a = 1 + 1"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    MathExpression {name: vec![97, 100, 100], children: vec![
      Number{value: 1},
      Number{value: 1}
    ]}
  ]}
]});
test!(parser_variable_function_call, r#"let a = foo()"#, variable_define, VariableDefine{children: vec![
  Identifier { value: vec![97] },
  Expression { children: vec![
    FunctionCall{name: vec![102, 111, 111], children: vec![
      FunctionArguments{ children: vec![
      ]}
    ]}
  ]}
]});
test!(parser_function_define, r#"fn a(){return 1;}"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Number{value: 1 }]}
      ]}
    ]}
  ]
});
test!(parser_function_define_multi_statements, r#"fn add(a,b){let x=a+b;return x;}"#, function_define, FunctionDefine{
  name: vec![97, 100, 100],
  children: vec![
    FunctionArguments{ children: vec![
      Expression { children: vec![Identifier { value: vec![97] }] },
      Expression { children: vec![Identifier { value: vec![98] }] },
    ] },
    FunctionStatements{ children: vec![
      VariableDefine{children: vec![
        Identifier { value: vec![120] },
        Expression { children: vec![
          MathExpression {name: vec![97, 100, 100], children: vec![
            Identifier{value: vec![97]},
            Identifier{value: vec![98]}
          ]}
        ]}
      ]},
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Identifier{value: vec![120] }]}
      ]}
    ]}
  ]
});


//ADDED TESTS IN PARSER FOR FINAL

//testing conditions
test!(parser_added_test_1, r#"fn a(){let x = 1>2; return x;}"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      VariableDefine{children: vec![
        Identifier { value: vec![120] },
        Expression { children: vec![
          ConditionalExpression {children: vec![
            Number{value: 1},
            GreaterThan{},
            Number{value: 2}
          ]}
        ]}
      ]},
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Identifier { value: vec![120] }]}
      ]}
    ]}
  ]
});

test!(parser_added_test_2, r#"fn a(){return 1 > 2;}"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      FunctionReturn{ children: vec![ 
        Expression { children: vec![ConditionalExpression {children: vec![
          Number{value: 1},
          GreaterThan{},
          Number{value: 2}
        ]}]}
      ]}
    ]}
  ]
});

test!(parser_added_test_3, r#"fn a(){let x = true && false; return x;}"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      VariableDefine{children: vec![
        Identifier { value: vec![120] },
        Expression { children: vec![
          ConditionalExpression {children: vec![
            Bool { value: true }, 
            And{}, 
            Bool { value: false }
          ]}
        ]}
      ]},
      FunctionReturn{ children: vec![ 
        Expression { children: vec![Identifier { value: vec![120] }]}
      ]}
    ]}
  ]
});


//testing if statement without else if
test!(parser_added_test_4, r#"if(1 > 2){let x = 1;} else {let x = 2;}"#, if_statement, 
  IfStatement { case: vec![
    ConditionalExpression { children: vec![
      Number { value: 1 }, 
      GreaterThan{}, 
      Number { value: 2 }] }], 
    statements: vec![
      VariableDefine { children: vec![
        Identifier { value: vec![120] }, 
        Expression { children: vec![Number { value: 1 }] }] }], 
    else_statements: vec![
      ElseStatement { 
        statements: vec![
          VariableDefine { children: vec![
            Identifier { value: vec![120] }, 
              Expression { children: vec![
                Number { value: 2 }] 
              }]
          }]  
      }] 
  }
);

//testing if statement with 1 else if
test!(parser_added_test_5, r#"if(1 > 2){let x = 1;} else if (1<=2) {let x = 2;} else {let x = 3;}"#, if_statement, 
  IfStatement { 
    case: vec![
      ConditionalExpression { children: vec![
        Number { value: 1 }, 
        GreaterThan{}, 
        Number { value: 2 }] }], 
    statements: vec![
      VariableDefine { children: vec![
        Identifier { value: vec![120] }, 
        Expression { children: vec![Number { value: 1 }] }] }], 
    else_statements: vec![
      ElseIfStatement{
      case: vec![
        ConditionalExpression { children: vec![
          Number { value: 1 }, 
          LessThanOrEqualTo{}, 
          Number { value: 2 }] }
      ], 
      statements: vec![
        VariableDefine { children: vec![
          Identifier { value: vec![120] }, 
          Expression { children: vec![Number { value: 2 }] }] }
      ]},
      ElseStatement { 
        statements: vec![
          VariableDefine { children: vec![
            Identifier { value: vec![120] }, 
              Expression { children: vec![
                Number { value: 3 }] 
              }]
          }]  
      }
      ] 
  }
);

//testing if statement with multiple else ifs
test!(parser_added_test_6, r#"if(1 > 2){let x = 1;} else if (1<=2) {let x = 2;} else if (1<0) {let x = 4;} else {let x = 3;}"#, if_statement, 
  IfStatement { 
    case: vec![
      ConditionalExpression { children: vec![
        Number { value: 1 }, 
        GreaterThan{}, 
        Number { value: 2 }] }], 
    statements: vec![
      VariableDefine { children: vec![
        Identifier { value: vec![120] }, 
        Expression { children: vec![Number { value: 1 }] }] }], 
    else_statements: vec![
      ElseIfStatement{
      case: vec![
        ConditionalExpression { children: vec![
          Number { value: 1 }, 
          LessThanOrEqualTo{}, 
          Number { value: 2 }] }
      ], 
      statements: vec![
        VariableDefine { children: vec![
          Identifier { value: vec![120] }, 
          Expression { children: vec![Number { value: 2 }] }] }
      ]},
      ElseIfStatement{
        case: vec![
          ConditionalExpression { children: vec![
            Number { value: 1 }, 
            LessThan{}, 
            Number { value: 0 }] }
        ], 
        statements: vec![
          VariableDefine { children: vec![
            Identifier { value: vec![120] }, 
            Expression { children: vec![Number { value: 4 }] }] }
        ]},
      ElseStatement { 
        statements: vec![
          VariableDefine { children: vec![
            Identifier { value: vec![120] }, 
              Expression { children: vec![
                Number { value: 3 }] 
              }]
          }]  
      }
      ] 
  }
);


//testing if statements within a function
test!(parser_added_test_7, r#"fn a(){let x = true && false; if (x || true) {return x;} else { return false; }; }"#, function_define, FunctionDefine{
  name: vec![97],
  children: vec![
    FunctionArguments{ children: vec![] },
    FunctionStatements{ children: vec![
      VariableDefine{children: vec![
        Identifier { value: vec![120] },
        Expression { children: vec![
          ConditionalExpression {children: vec![
            Bool { value: true }, 
            And{}, 
            Bool { value: false }
          ]}
        ]}
      ]},
      IfStatement { case: vec![
        ConditionalExpression { children: vec![
          Identifier { value: vec![120] },
          Or{}, 
          Bool { value: true }] }], 
        statements: vec![
          FunctionReturn{ children: vec![ 
            Expression { children: vec![Identifier { value: vec![120] }]}
          ]}], 
        else_statements: vec![
          ElseStatement { 
            statements: vec![
              FunctionReturn{ children: vec![ 
                Expression { children: vec![Bool { value: false }]}
              ]}]  
          }
        ] 
      }
    ]}
  ]
});


