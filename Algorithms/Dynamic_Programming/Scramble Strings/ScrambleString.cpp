
#include<bits/stdc++.h>
using namespace std;
unordered_map<string,int>mp;
    bool scramble(string s1,string s2)
    {
      
        if(s1==s2)
            return true;
        if(s1.length()<=1)
            return false;
        string key=s1+" "+s2;
        if(mp.find(key)!=mp.end())
            return mp[key];
        int n=s1.length();
      
        bool flag=false;
        for(int i=1;i<=n-1;i++)
        {
           
          
            if((scramble(s1.substr(0,i),s2.substr(n-i,i)) && 
              scramble(s1.substr(i,n-i),s2.substr(0,n-i)) )||
               (scramble(s1.substr(0,i),s2.substr(0,i)) && 
              scramble(s1.substr(i,n-i),s2.substr(i,n-i)) )
              )
            { flag=true;
                 break;
            }
        }
        return mp[key]=flag;
    }
    bool isScramble(string s1, string s2) {
        if(s1.length()!=s2.length())
            return false;
        if(s1.length()==0 && s2.length()==0)
            return true;
        mp.clear();
        return scramble(s1,s2);
        
        
    }
    
    
    int main()
    {
        string s1,s2;
        cin>>s1>>s2;
        bool ans=isScramble(s1,s2);
        cout<<ans<<endl;
        return 0;
    }
