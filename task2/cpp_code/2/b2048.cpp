#include<bits/stdc++.h>
using namespace std;
int main()
{
int a,sum=8; char s;
cin>>a>>s;
if(a>1000)
{
a=a-1000;
sum=sum+(a/500)*4;
if(a%500!=0)
{
sum=sum+4;
}
}
if(s=='y')
{
sum=sum+5;
}
cout<<sum<<endl;
return 0;
}
