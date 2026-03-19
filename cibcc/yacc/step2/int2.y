%{
#include <stdio.h>
#include "func.h"

char a[10];

 %}

  %union {
    int  Int;
    char Name[100];
  }

  %token <Int>  DIG
  %token <Name> DEMI VAR POW

%%

line      :
          | line expr                     
	  | line '\n'                   { endv(); print(); flash(); }
          | line error '\n'             { flash(); yyerrok; }  
          ;
expr      : digi
          | xpow
          ;
digi      : digi DEMI                   { push($2); }
          | DIG                         { digi($1, 1); }
          | DIG '/' DIG                 { digi($1, $3); }
          ;
xpow      : xpow DEMI                   { push($2); }
          | VAR                         { xpow(1, 1, a, 1, 1); }
          | DIG VAR                     { xpow($1, 1, a, 1, 1); }
          | DIG '/' DIG '*' VAR         { xpow($1, $3, a, 1, 1); }
          | VAR POW DIG                 { xpow(1, 1, a, $3, 1); }
          | VAR POW '(' DIG ')'         { xpow(1, 1, a, $4, 1); }
          | DIG VAR POW DIG             { xpow($1, 1, a, $4, 1); }
          | DIG VAR POW '(' DIG ')'     { xpow($1, 1, a, $5, 1); }
          | DIG '/' DIG '*' VAR POW DIG   { xpow($1, $3, a, $7, 1); }
          | DIG '/' DIG '*' VAR POW '(' DIG ')'  { xpow($1, $3, a, $8, 1); }
          | VAR POW '(' DIG '/' DIG ')'   { xpow(1, 1, a, $4, $6); }
          | DIG VAR POW '(' DIG '/' DIG ')'      { xpow($1, 1, a, $5, $7); }
          | DIG '/' DIG '*' VAR POW '(' DIG '/' DIG ')'
    				          { xpow($1, $3, a, $8, $10); }
          | VAR POW DEMI DIG            { xpow(1, 1, $3, $4, 1); }
          | VAR POW '(' DEMI DIG ')'    { xpow(1, 1, $4, $5, 1); }
          | DIG VAR POW DEMI DIG        { xpow($1, 1, $4, $5, 1); }
          | DIG VAR POW '(' DEMI DIG ')'  { xpow($1, 1, $5, $6, 1); }
          | DIG '/' DIG '*' VAR POW DEMI DIG     { xpow($1, $3, $7, $8, 1); }
          | DIG '/' DIG '*' VAR POW '(' DEMI DIG ')'
                                          { xpow($1, $3, $8, $9, 1); }
          | VAR POW '(' DEMI DIG '/'  DIG')'     { xpow(1, 1, $4,  $5, $7); }
          | DIG VAR POW '(' DEMI DIG '/' DIG ')' { xpow($1, 1, $5, $6, $8); }
          | DIG '/' DIG '*' VAR POW '(' DEMI DIG '/' DIG ')'
                                                 { xpow($1, $3, $8, $9, $11); }


%%

main()
{
   a[0] = '+';
   a[1] = '\0';

   yyparse();
}

yyerror(char *msg)
{
   printf("%s\n", msg);
}

     
