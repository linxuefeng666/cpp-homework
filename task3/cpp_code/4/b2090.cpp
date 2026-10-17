#include<bits/stdc++.h>
using namespace std;
int main()
{
int n,x,a[5]={0};
cin>>n;
for(int i=1; i<=n; i++)
{
cin>>x;
if(x<=18)
{
a[1]++;
}
else if(x>=19&&x<=35)
{
a[2]++;
}
else if(x>=36&&x<=60)
{
a[3]++;
}
else
{
a[4]++;
}
}
printf("%.2lf%%\n",double(a[1])/n*100);
printf("%.2lf%%\n",double(a[2])/n*100);
printf("%.2lf%%\n",double(a[3])/n*100);
printf("%.2lf%%\n",double(a[4])/n*100);
return 0;
}
