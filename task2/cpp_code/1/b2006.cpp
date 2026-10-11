#include<bits/stdc++.h>
using namespace std;
int main()
{
double x,a,y,b;
cin>>x>>a>>y>>b;
double L=(x*a-y*b)/(a-b);
cout<<fixed<<setprecision(2)<<L<<endl;
return 0;
}
