#include<bits/stdc++.h>
using namespace std;
int main()
{
int L,R,x,sum=0;
cin>>L>>R;
for(int i=L; i<=R; i++)
{
x=i;
while(x!=0)
{
if(x%10==2)
{
sum++;
}
x=x/10;
}
}
cout<<sum<<endl;
return 0;
}
