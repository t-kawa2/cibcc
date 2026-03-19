%{
  #include <stdio.h>
  #include "func.h"
  %}

%union {
  int  Int;
  char Name[100];
}

%token <Int>  DIG
%token <Name> DEMI

%%

line  :
      | line expr
      | line '\n'                { endv(); print(); flash(); }
      | line error '\n'          { flash(); yyerrok; }
      ;
expr  : digi
      ;
digi  : digi DEMI                { push($2); }
| DIG                      { digi($1, 1); }
| DIG '/' DIG              { digi($1, $3); }

%%

main()
{
  yyparse();;
}

yyerror(char *msg)
{
  printf("%s\n", msg);
}

