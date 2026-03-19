#include <stdio.h>
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

void xpow(int q, int q0, char *d, int p, int p0)
{
  int m, m0;

  if (*d != '-') {
    if (p0 == 1)
      m = p + 1;
    else
      m = p + p0;

    if (p == 0) {
      if (q0 == 1) {
	if (q != 1)
	  pushi(q);
      } else {
	pushi(q);
	val[n++] = '/';
	pushi(q0);
	val[n++] = '*';
      }
      val[n++] = 'x';
    } else {
      q = q*p0;
      q0 = q0*m;
      m0 = gcd(q, q0);
      q = q/m0;
      q0 = q0/m0;
      if (q0 == 1) {
	if (q != 1)
	  pushi(q);
      } else {
	pushi(q);
	val[n++] = '/';
	pushi(q0);
	val[n++] = '*';
      }
      val[n++] = 'x';
      val[n++] = '^';
      val[n++] = '(';
      pushi(m);
      if (p0 != 1) {
	val[n++] = '/';
	pushi(p0);
      }
      val[n++] = ')';
    }
  } else {
    if (p0 == 1)
      m = 1 - p;
    else

      m = p0 - p;

    if (p == 1 && p0 == 1) {
      if (q0 == 1) {
	if (q != 1)
	  pushi(q);
      }else {
	pushi(q);
	val[n++] = '/';
	pushi(q0);
	val[n++] = '*';
      }
      push_log();
      val[n++] = '|';
      val[n++] = 'X';
      val[n++] = '|';
    } else {
      if (p == 0) {
	if (q0 == 1) {
	  if (q != 1)
	    pushi(q);
	} else {
	  pushi(q);
	  val[n++] = '/';
	  pushi(q0);
	  val[n++] = '*';
	}
	val[n++] = 'x';
      } else {
	if (m < 0) {
	  if (n == 0)
	    val[n++] = '-';
	  else if (val[n-1] == '-') {
	    if (n == 1)
	      n--;
	    else
	      val[n-1] = '+';
	  } else
	    val[n-1] = '-';
	}
	q = q*p0;
	q0 = q0*m;
	if (q0 < 0)
	  q0 = -q0;
	m0 = gcd(q, q0);
	q = q/m0;
	q0 = q0/m0;
	if (q0 == 1) {
	  if (q != 1)
	    pushi(q);
	} else {
	  pushi(q);
	  val[n++] = '/';
	  pushi(q0);
	  val[n++] = '*';
	}
	val[n++] = 'x';
	val[n++] = '^';
	val[n++] = '(';
	if (m <= -1) {
	  val[n++] = '-';
	  pushi(-m);
	} else
	  pushi(m);
	if (p0 != 1) {
	  val[n++] = '/';
	  pushi(p0);
	}
	val[n++] = ')';
      }
    }
  }
}



      
