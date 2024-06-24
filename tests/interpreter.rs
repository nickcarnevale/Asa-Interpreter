extern crate asalang;
extern crate nom;
use std::io::Write;

use asalang::*;
use nom::IResult;

macro_rules! test_fragment {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),AsaErrorKind> {
      let tokens = lex($test);
      match program(tokens) {
        Ok((tokens, tree)) => {
          assert_eq!(tokens.is_done(), true); // Check that input token stream is fully parsed
          let mut interpreter = Interpreter::new();
          let result = interpreter.exec(&tree);
          std::io::stdout().flush();
          assert_eq!(result, $expected);
          Ok(())
        },
        Err(e) => Err(AsaErrorKind::Generic(format!("{:?}",e))),
      }
    }
  )
}

macro_rules! test_program {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),AsaErrorKind> {
      let tokens = lex($test);
      match program(tokens) {
        Ok((tokens, tree)) => {
          assert_eq!(tokens.is_done(), true); // Check that input token stream is fully parsed
          let mut interpreter = Interpreter::new();
          let compile_result = interpreter.exec(&tree)?;
          let main_result = interpreter.start_main(vec![]);
          assert_eq!(main_result, $expected);
          Ok(())
        },
        Err(e) => Err(AsaErrorKind::Generic(format!("{:?}",e))),
      }
    }
  )
}

// Test interpreter fragments (no main function)
test_fragment!(interpreter_numeric, r#"123"#, Ok(Value::Number(123)));
test_fragment!(interpreter_string, r#""hello world""#, Ok(Value::String("helloworld".to_string())));
test_fragment!(interpreter_bool_true, r#"true"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_bool_false, r#"false"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_identifier, r#"x"#, Err(AsaErrorKind::UndefinedFunction));
test_fragment!(interpreter_function_call, r#"foo()"#, Err(AsaErrorKind::UndefinedFunction));
test_fragment!(interpreter_function_call_one_arg, r#"foo(a)"#, Err(AsaErrorKind::UndefinedFunction));
test_fragment!(interpreter_function_call_more_args, r#"foo(a,b,c)"#, Err(AsaErrorKind::UndefinedFunction));
test_fragment!(interpreter_variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test_fragment!(interpreter_variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_variable_string, r#"let string = "Hello World";"#, Ok(Value::String("HelloWorld".to_string())));
test_fragment!(interpreter_variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_math, r#"1 + 1"#, Ok(Value::Number(2)));
test_fragment!(interpreter_math_no_space, r#"1-1"#, Ok(Value::Number(0)));
test_fragment!(interpreter_math_multiply, r#"2 + 4"#, Ok(Value::Number(6)));
test_fragment!(interpreter_assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test_fragment!(interpreter_assign_function, r#"let x = foo();"#, Err(AsaErrorKind::UndefinedFunction));
test_fragment!(interpreter_assign_function_arguments, r#"let x = foo(a,b,c);"#, Err(AsaErrorKind::UndefinedFunction));

// Test full programs
test_program!(interpreter_define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test_program!(interpreter_define_function_args, r#"fn main(){return foo(1,2);} fn foo(a,b){return a+b;}"#, Ok(Value::Number(3)));
test_program!(interpreter_define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test_program!(interpreter_define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;     
  let y = bar(c + b); 
  return x + y;
}

fn bar(a) {
  return a + 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(10)));


//additional tests from homework 5
test_program!(hw5_test_1, r#"fn main() { return 10 + 5; }"#, Ok(Value::Number(15)));
test_program!(hw5_test_2, r#"fn main() { return 20 - 7; }"#, Ok(Value::Number(13)));
test_program!(hw5_test_3, r#"fn main() { let x = 30; let y = 15; return x - y; }"#, Ok(Value::Number(15)));
test_program!(hw5_test_4, r#"fn main() { let x = 100; let y = 50; let z = x + y; return z; }"#, Ok(Value::Number(150)));
test_program!(hw5_test_5, r#"fn main() { let x = 50; let y = 20; let z = 10; return 65; }"#, Ok(Value::Number(65)));
test_program!(hw5_test_6, r#"fn main() { let x = 50; return x; }"#, Ok(Value::Number(50)));



//final tests

//testing conditional
test_program!(final_test_1, r#"fn main() { return 1>2; }"#, Ok(Value::Bool(false)));
test_program!(final_test_2, r#"fn main() { return a(); } fn a(){let x = true && false; return x;}"#, Ok(Value::Bool(false)));
test_program!(final_test_3, r#"fn main() { let x = 2; let y = 3; let z = x <= y; let ab = a(); let zz = z || ab; return zz; } fn a(){let c = true && false; return c;}"#, Ok(Value::Bool(true)));

//test errors for conditionals
test_program!(final_test_4, r#"fn main() { return true>2; }"#, Err(AsaErrorKind::TypeMismatch));
test_program!(final_test_5, r#"fn main() { return 1||2; }"#, Err(AsaErrorKind::TypeMismatch));

//testing if statements
test_program!(final_test_6, r#"fn main() { if (1<2) {return 3;} else{return 5;}; }"#, Ok(Value::Number(3)));
test_program!(final_test_7, r#"fn main() { if (1<2) {let x = 1; return x;} else{return 5;}; }"#, Ok(Value::Number(1)));

//test else
test_program!(final_test_8, r#"fn main() { if (1>2) {return 3;} else{return 5;}; }"#, Ok(Value::Number(5)));

//test else if
test_program!(final_test_9, r#"fn main() { if (1>2) {return 3;} else if (1 < 2) {return 6;} else{return 5;}; }"#, Ok(Value::Number(6)));

//test multiple else ifs
test_program!(final_test_10, r#"fn main() { if (1>2) {return 3;} else if (1 == 2){return 2;} else if (1 < 2) {return 6;} else{return 5;}; }"#, Ok(Value::Number(6)));

