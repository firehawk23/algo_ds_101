# This algorithm finds the shortest common supersequence of 2 strings - 
# the shortest string that has both str1 and str2 as subsequences. 
# If 'm' is the length of the first string and 'n' is the length of 
# the second string, then this algo takes O(m * n) time and O(m * n) space.

# Returns length of the shortest supersequence of X and Y 
def superSeq(X, Y, m, n): 
	dp = [[0] * (n + 2) for i in range(m + 2)] 

	# Fill table in bottom up manner 
	for i in range(m + 1): 
		for j in range(n + 1): 
			
			#Below steps follow above recurrence 
			if (not i): dp[i][j] = j 
			elif (not j): dp[i][j] = i 
			
			elif (X[i - 1] == Y[j - 1]): 
				dp[i][j] = 1 + dp[i - 1][j - 1] 
				
			else: dp[i][j] = 1 + min(dp[i - 1][j], 
									dp[i][j - 1]) 
			
	return dp[m][n] 

# Driver Code 
X = "AGGTAB"
Y = "GXTXAYB"
print("Length of the shortest supersequence is %d"
	% superSeq(X, Y, len(X), len(Y))) 

# This code is contributed by Ansu Kumari 
