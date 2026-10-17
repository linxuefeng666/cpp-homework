#include<bits/stdc++.h>
using namespace std;
int main()
{
int n;
cin>>n;
double sum;
for(int i=1; i<=n; i++)
{
int x;
cin>>x;
sum+=x;
}
printf("%.2lf",sum/n);
return 0;
}
