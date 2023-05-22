use tree_sitter::{Parser, Node};
use std::io::{self, BufRead};
use std::rc::Rc;
use rpds::HashTrieMap;

fn main() {
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let code = line.unwrap();
    let source = code.as_bytes().to_vec();
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_lambda::language()).expect("Error loading lambda grammar");
    let tree = parser.parse(code, None).unwrap();
    let mut cursor = tree.walk();
    for expr in tree.root_node().children(&mut cursor) {
      match eval(&source, &HashTrieMap::new(), expr) {
        Val::Num(n)     => { print!("{}\n", n); }
        Val::Clo(_,_,_) => { print!("Closure\n"); }
      }
    }
  }
}

#[derive(Debug,Clone)]
enum Val<'a> {
  Num(f32),
  Clo(HashTrieMap<String,Rc<Val<'a>>>, String, Node<'a>)
}

impl<'a> Val<'a> {
  fn as_num(self: &Val<'a>) -> f32 {
    return match self {
      Val::Num(n) => *n,
      _ => panic!("{:?}", self),
    }
  }
}

fn eval<'a>(source: &[u8], env: &HashTrieMap<String,Rc<Val<'a>>>, expr: Node<'a>) -> Val<'a> {
  return match expr.kind() {
    "number" => Val::Num(expr
          .utf8_text(source).unwrap()
          .parse::<f32>().unwrap()),
    "identifier" => {
      let x = expr.utf8_text(&source).unwrap();
      return (**env.get(x).unwrap()).clone();
    }
    "parens_expr" => eval(&source, &env, expr.child(1).unwrap()),
    "binary_expr" => {
      let left_num = eval(&source, &env, expr.child(0).unwrap()).as_num();
      let right_num = eval(&source, &env, expr.child(2).unwrap()).as_num();
      let op = expr.child(1).unwrap().utf8_text(&source).unwrap();
      return match op {
        "+" => Val::Num(left_num + right_num),
        "-" => Val::Num(left_num - right_num),
        "*" => Val::Num(left_num * right_num),
        "/" => Val::Num(left_num / right_num),
        x   => { panic!("{}", x); }
      }
    }
    "unary_expr" => {
      let op = expr.child(0).unwrap().utf8_text(&source).unwrap();
      let num = eval(&source, &env, expr.child(1).unwrap()).as_num();
      return match op {
        "-" => Val::Num(-num),
        x   => { panic!("{}", x); }
      }
    }
    "let_expr" => {
      let x = expr.child(1).unwrap().utf8_text(&source).unwrap().to_string();
      let rhs = eval(&source, &env, expr.child(3).unwrap());
      return eval(&source, &env.insert(x,Rc::new(rhs)), expr.child(5).unwrap());
    }
    "fn_expr" => {
      let param = expr.child(1).unwrap().utf8_text(&source).unwrap().to_string();
      let body = expr.child(3).unwrap();
      return Val::Clo(env.clone(), param, body);
    }
    "app_expr" => {
      let fun = eval(&source, &env, expr.child(0).unwrap());
      let arg = eval(&source, &env, expr.child(1).unwrap());
      return match fun {
        Val::Clo(fun_env, param, body) => eval(&source, &fun_env.insert(param,Rc::new(arg)), body),
        x => panic!("Type error {:?}", x)
      }
    }
    "ifz_expr" => {
      return match eval(&source, &env, expr.child(1).unwrap()) {
        Val::Num(0.0) => eval(&source, &env, expr.child(2).unwrap()),
        _             => eval(&source, &env, expr.child(3).unwrap()),
      }
    }
    x => { panic!("{}", x); }
  }
}
