#include<bits/stdc++.h>
using namespace std;
int main ()
{
int n,x,a[10001];
cin>>n;
for(int i=1; i<=n; i++)
{
cin>>a[n];
}
cin>>x;
for(int i=1; i<=n; i++)
{
if(a[i]=x)
{
cout<<"1"<<endl;
return 0;
}
}
cout<<"-1"<<endl;
return 0;
}
