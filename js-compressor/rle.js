// Javascript program to implement run length encoding
	function printRLE(str)
	{
		let n = str.length;
		for (let i = 0; i < n; i++)
		{
			// Count occurrences of current character
			let count = 1;
			while (i < n - 1 && str[i] == str[i+1])
			{
				count++;
				i++;
			}
			
			// Print character and its count
			document.write(str[i]);
			document.write(count);
		}
	}
	
	let str = "wwwwaaadexxxxxxywww";
	printRLE(str);
	
	// This code is contributed by rag2127

