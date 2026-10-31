#include<bits/stdc++.h>
using namespace std;
double MAX(double a, double b, double c)
{
return max(a,max(b,c));
}
int main()
{
double a,b,c;
cin>>a>>b>>c;
double m=MAX(a,b,c)/(MAX(a+b,b,c)*MAX(a,b,b+c));
printf("%.3f",m);
return 0;
}
