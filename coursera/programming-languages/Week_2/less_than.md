# Less Than

## Syntax

~e1~ < ~e2~
where:
  ~e1~, ~e2~ are subexpressions.

## Type-checking

If ~e1~ and ~e2~ have type ~int~,
then the result has type ~bool~,
else does not type-check.

## Evaluation

First evaluate ~e1~ to ~v1~ and ~e2~ to ~v2~,
then:
  if ~v1~ < ~v2~, evaluates to ~true~,
  else evaluates to ~false~.
