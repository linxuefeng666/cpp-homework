#include<bits/stdc++.h>
using namespace std;
int main()
{
int n,a[101],m,sum=0;
cin>>n;
for(int i=1; i<=n; i++)
{
cin>>a[i];
}
cin>>m;
for(int i=1; i<=n; i++)
{
if(a[i]==m)
{
sum++;
}
}
cout<<sum<<endl;
return 0;
}
