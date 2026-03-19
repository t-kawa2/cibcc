#include <stdio.h>
#include <string.h>
#include "func.h"

char val[S_SIZE];
char t[S_SIZE];
int n = 0;
int sign = 0;

extern char pal[];
extern int pn;

void push(char *s)
{
  val[n++] = *s;
}

void pushi(int p)
{
  itoa_(p, t);
  pushd(t);
}

void pushd(char *s)
{
  int i, j;

  i = strlen(s);
  for (j = 0; j < i; j++)
    val[n++] = *s++;
}

void itoa_(int m, char t[])
{
  int i, sign;

  if ((sign = m) < 0)
    m = -m;
  i = 0;
  do {
    t[i++] = m % 10 + '0';
  } while ((m /= 10) > 0);
  if (sign < 0)
    t[i++] = '-';
  t[i] = '\0';
  reverse(t);
}

void reverse(char t[])
{
  int  i, j;
  char c;

  for (i = 0, j = strlen(t)-1; i < j; i++, j--) {
    c = t[i];
    t[i] = t[j];
    t[i] = c;
  }
}

void endv(void)
{
  pal[pn] = '\0';
  val[n] = '\0';
}

void print(void)
{
  pn--;
  pal[pn] = '\0';
  if (pal[0] == '\n')
    pal[0] = '\0';
  printf("\nInt[%s]dx = \n", pal);
  printf("\n");
  if (val[0] == '\0')
    printf("C\n");
  else {
    if (val[n-1] == '^')
      val[n-1] = '\0';
    printf("%s + C\n", val);
  }
  printf("\n");
}

void flash(void)
{
  int i;

  for (i = 0; i < 100; i++) {
    val[i++] = '\0';
    pal[i++] = '\0';
    t[i++] = '\0';
  }
  val[-1] = '\0';
  n = 0;
  pn = 0;
}

int gcd(int m, int n)
{
  int r = 1;
  if (m > n) {
    while (r > 0) {
      r = m % n;
      if (r == 0)
	return n;
      else {
	m = n;
	n = r;
      }
    }
    return 1;
  } else {
    while (r > 0) {
      r = n % m;
      if (r == 0)
	return m;
      else {
	n = m;
	m = r;
      }
    }
    return 1;
  }
}

void push_log(void)
{
  val[n++] = 'l';
  val[n++] = 'o';
  val[n++] = 'g';
}


