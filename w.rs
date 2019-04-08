use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
	let mut s1="ewfdcjzhxnmfcsjzdnc";
	let mut t1 ="sdbzsdhjcsazdhcj";
	let mut s:Vec<char> = s1.chars().collect();
	let mut t:Vec<char> = t1.chars().collect();
	let mut l=s1.len();
	let mut m=t1.len();
	let mut dp = Vec::new();
	let mut prev = Vec::new();
	for i in 0..m+1
	{
		prev.push(0);
	}
	for i in 0..l+1
	{
		let mut vec1 = Vec::new();
		for j in 0..m+1
		{
			if i==0 || j==0
			{
				vec1.push(j+i);
				// prev[j]=0;
			}
			else if s[i-1]==t[j-1]
			{
				vec1.push(prev[j-1]);
				// prev[j]=prev[j-1];
			}
			else
			{
				if prev[j]<vec1[j-1]
				{
					vec1.push(prev[j]+1);
					// prev[j]=prev[j]+1;
				}
				else
				{
					let z=vec1[j-1]+1;
					vec1.push(z);
					// prev[j]=z;
				}
			}

		}
		for j in 0..m+1
		{
			// print!("{} ",dp[i][j]);
			prev[j]=vec1[j];
		}
		dp.push(vec1);
		for j in 0..m+1
		{
			print!("{} ",dp[i][j]);
			// prev[j]=vec1[j];
		}
		print!("\n");
	}
	print!("{}",dp[l][m]);
}
