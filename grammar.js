const PREC = {
  app: 4,
  neg: 3,
  prod: 2,
  sum: 1,
  fun: 0,
};

module.exports = grammar({
  name: 'lambda',

  rules: {
    source_file: $ => repeat($._expr),

    _arg: $ => choice(
      $.parens_expr,
      $.identifier,
      $.number,
    ),

    identifier: $ => /[a-zA-Z][a-zA-Z0-9]*/,
    number: $ => /\d+/,
    parens_expr: $ => seq('(',$._expr,')'),

    _expr: $ => choice(
      $._arg,
      $.unary_expr,
      $.binary_expr,
      $.app_expr,
      $.ifz_expr,
      $.fn_expr,
      $.let_expr,
    ),

    unary_expr: $ => prec(PREC.neg, seq('-', $._expr)),
    binary_expr: $ => choice(
      prec.left(PREC.prod, seq($._expr, choice('*', '/'), $._expr)),
      prec.left(PREC.sum,  seq($._expr, choice('+', '-'), $._expr)),
    ),
    ifz_expr: $ => prec.right(PREC.app, seq('ifz', $._arg, $._arg, $._arg)),
    app_expr: $ => prec.left(PREC.app, seq($._expr, repeat1($._arg))),
    fn_expr: $ => prec.right(PREC.fun, seq('fn', $.identifier, '=>', $._expr)),
    let_expr: $ => prec.right(PREC.fun, seq('let', $.identifier, '=', $._expr, 'in', $._expr)),
  }
});
