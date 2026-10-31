#include<bits/stdc++.h>
using namespace std;
double f(double x, int n)
{
if(n>1) return sqrt(n+f(x,n-1));
else return sqrt(1+x);
}
int main()
{
double x;
int n;
cin>>x>>n;
printf("%.2f",f(x,n));
return 0;
}
