#include <stdlib.h>
#include "func.h"

extern char val[];
extern int n;

void digi(int p, int q)
{
  int m;

  if (p == 0)
    ;
  else if (q == 1) {
         if (p != 1)
           pushi(p);
         val[n++] = 'x';
       } else {
    m = gcd(p, q);
    if (m != 1) {
      p = p/m;
      q = q/m;
    }
    pushi(p);
    if (q != 1) {
      val[n++] = '/';
      pushi(q);
      val[n++] = '*';
    }
    val[n++] = 'x';
  }
}

