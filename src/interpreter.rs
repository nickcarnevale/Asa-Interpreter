use crate::parser::Node;
use std::collections::HashMap;
use crate::error::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}

type Frame = HashMap<String, Value>;
type Arguments = Node;
type Statements = Node;

#[derive(Debug)]
pub struct Interpreter {
  // Function Table:
  // Key - Function name
  // Value - Vec<Node> arguments, statements
  functions: HashMap<String, (Arguments,Statements)>,
  // Stack:
  // Each element in the stack is a function stack frame.
  // Crate a new stack frame on function entry.
  // Pop stack frame on function return.
  // Key - Variable name
  // Value - Variable value
  stack: Vec<Frame>,
}


impl Interpreter {

  pub fn new() -> Interpreter {
    let mut interpreter = Interpreter {
      functions: HashMap::new(),
      stack: Vec::new(),
    };
    interpreter.push_new_frame();
    interpreter
  }  

  pub fn push_new_frame(&mut self) {
    self.stack.push(Frame::new());
  }

  pub fn exec(&mut self, node: &Node) -> Result<Value,AsaErrorKind> {
    match node {
      Node::Program{children} => {
        
        let mut return_val = Value::Bool(true);
        
        //needed to change
        for n in children {
          match self.exec(n){
            Ok(value) => return_val = value,
            Err(e) => {
              self.stack.pop(); 
              return Err(e); 
            }
          }
        }
        self.stack.pop();
        Ok(return_val)
      },


      // Evaluates a mathematical expression based on the elements in the children argument. If the expression is valid, the code evaluates it and returns a new Value object with the resulting value. If the expression is not valid, the code returns an error message.
      Node::MathExpression{name, children} => {
        match name.as_slice() {
          b"add" | b"sub" => {
            if children.len() == 2 {
                let val1 = self.exec(&children[0])?;
                let val2 = self.exec(&children[1])?;
                match (val1, val2) {
                  (Value::Number(val1), Value::Number(val2)) => {
                    match name.as_slice() {
                        b"add" => Ok(Value::Number(val1 + val2)),
                        b"sub" => Ok(Value::Number(val1 - val2)),
                        _ => Err(AsaErrorKind::Generic("1. Cannot solve mathematical expression".to_string())),
                    }
                  }
                  _ => Err(AsaErrorKind::Generic("2. Cannot solve mathematical expression".to_string())),
                }
            } else {
                Err(AsaErrorKind::Generic("3. Cannot solve mathematical expression".to_string()))
            }
          }
          _ => Err(AsaErrorKind::Generic("4. Cannot solve mathematical expression".to_string())),
        }
      },

      Node::ConditionalExpression{children} => {

        let val1 = self.exec(&children[0])?;
        let val2 = self.exec(&children[2])?;  
        let operation = &children[1].clone();
        //Notes if Val1 and Val2 are boolean, I can only perform and 'and' and 'or' operation
        //if they are numbers then you can perform the greaterthan lessthan operations
        //if they are identifiers, both types can be performed but a type check needs to occur 


        match operation {
          Node::EqualTo {} => {
            if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
              return Ok(Value::Bool(num1 > num2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          Node::NotEqualTo {} => {
            if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
              return Ok(Value::Bool(num1 != num2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          Node::LessThan {} => {
            if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
              return Ok(Value::Bool(num1 < num2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          Node::GreaterThan {} => {
            if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
              return Ok(Value::Bool(num1 > num2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          Node::LessThanOrEqualTo {} => {
            if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
              return Ok(Value::Bool(num1 <= num2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          Node::GreaterThanOrEqualTo {} => {
            if let (Value::Number(num1), Value::Number(num2)) = (val1, val2) {
              return Ok(Value::Bool(num1 >= num2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          Node::And {} => {
            if let (Value::Bool(bool1), Value::Bool(bool2)) = (val1, val2) {
              return Ok(Value::Bool(bool1 && bool2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }          
          }
          Node::Or {} => {
            if let (Value::Bool(bool1), Value::Bool(bool2)) = (val1, val2) {
              return Ok(Value::Bool(bool1 || bool2));
            } else {
                return Err(AsaErrorKind::TypeMismatch);
            }
          }
          _ => unimplemented!() // Handle other operations as needed
        }
      
      },

      Node::IfStatement{case, statements, else_statements} =>{
        //case is a conditional expression
        //if true then evaluate the statements and skip the rest of the code

        let mut new_frame = HashMap::new();
        let mut result: Result<Value, AsaErrorKind> = Err(AsaErrorKind::UndefinedFunction);

        let condition = self.exec(&case[0])?;
        
        if let Value::Bool(true) = condition {
          self.stack.push(new_frame);
          for n in statements {
              result = self.exec(&n);
          }
          self.stack.pop();

        } else {

          for statement in else_statements {
            match statement {
              Node::ElseIfStatement { case, statements } => {
                  let else_condition = self.exec(&case[0])?;
                  if let Value::Bool(true) = else_condition {
                      self.stack.push(new_frame);
                      for statement in statements {
                          result = self.exec(&statement);
                      }
                      self.stack.pop();
                      break;
                  }
              }
              Node::ElseStatement { statements } => {
                  self.stack.push(new_frame);
                  for statement in statements {
                      result = self.exec(&statement);
                  }
                  self.stack.pop();
                  break;
              }
              _ => {
                  return Err(AsaErrorKind::UndefinedFunction);
              }
            }
          }
        }
        return result;
      },

      // Defines a function that takes some arguments and executes a program based on those arguments. The code first checks if the function exists, and if it does, it creates a new scope in which to execute the function's statements (push a new Frame onto the interpreter stack). The code then executes each statement in the function's statements list and returns the result of the function's execution. You will have to correlate each passed value with the apprpriate variable in the called function. If the wrong number or an wrong type of variable is passed, return an error. On success, insert the return value of the function (if any) into the appropriate entry of the caller's stack.
      Node::FunctionCall { name, children } => {

        let mut new_frame = HashMap::new();
        let mut result: Result<Value, AsaErrorKind> = Err(AsaErrorKind::UndefinedFunction);
        let rt = self as *mut Interpreter;
        let func_name = String::from_utf8_lossy(&name).to_string();
        let (arguments, statements) = self.functions.get(&func_name).map(|(args, body)| (args.clone(), body.clone())).ok_or(AsaErrorKind::UndefinedFunction)?;
        //handle arguements

        if let Node::FunctionArguments { children: args_children } = arguments {
          let actual_arguments = if let Some(Node::FunctionArguments { children }) = children.get(0) {children} else {children};
          if args_children.len() != actual_arguments.len() {
            return Err(AsaErrorKind::Generic("Number of arguments does not match expected".to_string()));
          }

          //now that I have all of the arguments, insert each into new frame
          for (arg, value) in args_children.iter().zip(actual_arguments.iter()) {
            if let Node::Expression { children: expr_children } = arg {
              if let Some(Node::Identifier { value: name }) = expr_children.first() {
                let new_value = self.exec(value)?;
                new_frame.insert(String::from_utf8_lossy(&name).to_string(), new_value);
              } else {
                return Err(AsaErrorKind::Generic("Syntax Error".to_string()));
              }
            } else {
              return Err(AsaErrorKind::Generic("Expected argument to be an identifier within an expression".to_string()));
            }
          }
        } else if !children.is_empty() {
          return Err(AsaErrorKind::Generic("No Arguments Expected".to_string()));
        }

        //handle statements
        self.stack.push(new_frame);
        let mut last_value = Value::Bool(false);
        let mut returned = false;

        match statements {
          Node::FunctionStatements { children } => {
              for n in children {
                  result = self.exec(&n);
              }
          }
          _ => {
              todo!();
          }
        }
        self.stack.pop();
        result  
      },

      // Defines a new function based on the elements in the children argument. The name of the function is retrieved from the node struct, the arguments are the first child, and the statements that define the function are the second child. A new key-value pair is then inserted into the functions table of the interprer. If the function was successfully defined, the code returns a Value object with a boolean value of true, otherwise an error is returned.
      Node::FunctionDefine{name, children} => {

        let arguments = &children[0]; 
        let statements = &children[1]; 
        self.functions.insert(String::from_utf8_lossy(name).to_string(), (arguments.clone(), statements.clone()));
        Ok(Value::Bool(true))
      },

      // Calls the exec() method on the first element in the children argument, which recursively evaluates the AST of the program being executed and returns the resulting value or error message.
      Node::FunctionReturn{children} => {
        self.exec(&children[0])
      },
  
      // Retrieves the value of the identifier from the current frame on the stack. If the variable is defined in the current frame, the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
        let last = self.stack.len() - 1;
        let variable_name = String::from_utf8_lossy(&value).to_string();
        match self.stack[last].get(&variable_name) {
            Some(id_value) => Ok(id_value.clone()),
            None => Err(AsaErrorKind::UndefinedFunction),
        }
      },

      // Checks the type of the first element in the children argument and deciding what to do based on that type. If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => {
        match children[0] {
          Node::VariableDefine { .. } |
          Node::FunctionReturn { .. } => {
              self.exec(&children[0])
          },
          _ => Err(AsaErrorKind::Generic("Not a defined statement".to_string())),
      }
      },

      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, and the value is retrieved by running the run method on the second element of the children argument. The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        let name: String = match &children[0] {
          Node::Identifier { value } => String::from_utf8_lossy(&value).to_string(),
          _ => "".to_string(),
        };
        let value = self.exec(&children[1])?;
        let last = self.stack.len() -1 ;
        self.stack[last].insert(name, value.clone());
        Ok(value)
      },

      // Evaluate the child node using the exec() method.
      Node::Expression{children} => {
        self.exec(&children[0])
      }, 
      Node::Number{value} => {
        Ok(Value::Number(*value))
      }, 
      Node::String{value} => {
        Ok(Value::String(value.clone()))
      }, 
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      }, 
      x => {
        unimplemented!();
      },
    }
  }

  pub fn start_main(&mut self, arguments: Vec<Node>) -> Result<Value,AsaErrorKind> {
    // This node is equivalent to the following Asa program source code:
    // "main()"
    // It calls the main function with a FunctionArguments node as input.

    let start_main = Node::FunctionCall{name: "main".into(), children: arguments};
    // Call the main function by running this code through the interpreter. 
    self.exec(&start_main)
  }
  
}