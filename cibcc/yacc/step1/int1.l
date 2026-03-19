%{
#include <stdio.h>
#include <string.h>
/* #include <stdlib.h> */

#include "y.tab.h"
#include "func.h"

char pal[100];
int pn = 0;

extern char t[];

%}

ws     [ \t]
digit  [0-9]
demi   [-+]
symbol [*/()\n]
other  .

%%

{ws}+
{digit}+    { sscanf(yytext, "%d", &yylval.Int);  ppushi(yylval.Int);
                                                    return(DIG); }
{demi}      { strcpy(yylval.Name, yytext); ppush(yylval.Name); return(DEMI); }
{symbol}    { strcpy(yylval.Name, yytext); ppush(yylval.Name);
                                                    return(yytext[0]); }
{other}     { fprintf(stderr, "Illegl char '%c'\n", yytext[0]); }

%%

void ppush(char *s)
{
  pal[pn++] = *s;
}

void ppushi(int p)
{
  itoa_(p, t);
  ppushd(t);
}

void ppushd(char *s)
{
int i, j;

  i = strlen(s);
  for (j = 0; j < i; j++)
	 pal[pn++] = *s++;
}


       
