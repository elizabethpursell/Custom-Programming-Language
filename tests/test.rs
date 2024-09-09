// Based on homework-5 solutions and my submission
extern crate asalang;
extern crate nom;

use asalang::{program, Value, start_interpreter};

macro_rules! test {
    ($func:ident, $test:tt, $expected:expr) => (
        #[test]
        fn $func() -> Result<(),String> {
            match program($test) {
                Ok((input, p)) => {
                    assert_eq!(input, "");
                    assert_eq!(start_interpreter(&p), $expected);
                    Ok(())
                },
                Err(e) => Err(format!("{:?}",e)),
            }
        }
    )
}

macro_rules! test_parser {
    ($func:ident, $test:tt) => (
        #[test]
        fn $func() -> Result<(),String> {
            match program($test) {
                Ok((input, _)) => {
                    assert_ne!(input, "");
                    Ok(())
                },
                Err(e) => Err(format!("{:?}",e)),
            }
        }
    )
}

test!(numeric, r#"123"#, Ok(Value::Number(123)));
test!(identifier, r#"x"#, Err("Undefined variable"));
test!(string, r#""hello world""#, Ok(Value::String("hello world".to_string())));
test!(bool_true, r#"true"#, Ok(Value::Bool(true)));
test!(bool_false, r#"false"#, Ok(Value::Bool(false)));
test!(function_call, r#"foo()"#, Err("Undefined function"));
test!(function_call_one_arg, r#"foo(a)"#, Err("Undefined function"));
test!(function_call_more_args, r#"foo(a,b,c)"#, Err("Undefined function"));
test!(variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test!(variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test!(variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test!(variable_string, r#"let string = "Hello World";"#, Ok(Value::String("Hello World".to_string())));
test!(variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test!(math, r#"1 + 1"#, Ok(Value::Number(2)));
test!(math_no_space, r#"1+1"#, Ok(Value::Number(2)));
test!(math_subtraction, r#"1 - 1"#, Ok(Value::Number(0)));
test!(math_multiply, r#"2 * 4"#, Ok(Value::Number(8)));
test!(math_divide, r#"6 / 2"#, Ok(Value::Number(3)));
test!(math_exponent, r#"2 ^ 4"#, Ok(Value::Number(16)));
test!(math_more_terms, r#"10 + 2*6"#, Ok(Value::Number(22)));
test!(math_more_terms_paren, r#"((10+2)*6)/4"#, Ok(Value::Number(18)));
test!(assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test!(assign_function, r#"let x = foo();"#, Err("Undefined function"));
test!(assign_function_arguments, r#"let x = foo(a,b,c);"#, Err("Undefined function"));
test!(define_function, r#"fn main(){return foo();} fn foo(){return 5;}"#, Ok(Value::Number(5)));
test!(define_function_args, r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b+c;}"#, Ok(Value::Number(6)));
test!(define_function_more_statement, r#"fn main() {
  return foo();
}
fn foo(){
  let x = 5;
  return x;
}"#, Ok(Value::Number(5)));
test!(define_full_program, r#"fn foo(a,b,c) {
  let x = a + 1;
  let y = bar(c - b);
  return x * y;
}

fn bar(a) {
  return a * 3;
}

fn main() {
  return foo(1,2,3);  
}"#, Ok(Value::Number(6)));

// HW 5 TESTS

test!(function_var_scope, r#"fn main() {
    let y = 1;
    return foo();
  }
  fn foo(){
    return y;
  }"#, Err("Undefined variable"));

test!(multiple_exp_args, r#"fn foo(a,b,c,d) {
    let y = bar(a + b, c + d);
    return y + 1;
  }
  
  fn bar(a, b) {
    return a + b;
  }
  
  fn main() {
    return foo(1,2,3,4);  
  }"#, Ok(Value::Number(11)));

test!(no_main, r#"fn foo(a,b,c,d) {
    let y = bar(a + b, c + d);
    return y + 1;
  }
  
  fn bar(a, b) {
    return a + b;
  }"#, Err("Undefined function"));

// FINAL EXAM TESTS

test!(if_let_multiline, r#"let x = if false {
    return 4;
} else if true {
    let a = 5;
    let b = a ^ 2;
    return b;
};"#, Ok(Value::Number(25)));
test!(if_return_str, r#"if false { return "invalid"; } else { return "valid"; }"#, Ok(Value::String("valid".to_string())));
test!(if_return_var, r#"if false {let z=1; return z * 2;} else if true {let x = 2; let y = x + 1; return y;}"#, Ok(Value::Number(3)));
test!(if_invalid_return_type, r#"if true {let z = 0; return z * 2;} else if false {return "valid";}"#, Err("Inconsistent type used in if expression"));
test!(if_no_condition_met, r#"if false {let z = 0; return z * 2;} else if false {let x = 1; let y = x + 1; return y;}"#, Err("Condition not met"));
test_parser!(if_wrong_order, r#"let x = if false {return 4;} else {return false;} else if true {let a = 5; let b = a ^ 2;};"#);