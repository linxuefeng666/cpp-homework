#include<bits/stdc++.h>
using namespace std;
long long fh(long long n,long long k)
{
k=k-1;
if(k==0) return n%10;
else return fh(n/10,k);
}
int main()
{
long long n,k;
cin>>n>>k;
cout<<fh(n,k);
return 0;
}
