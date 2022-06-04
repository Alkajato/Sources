# Sources
CLI Program for parsing specially formatted text files based on inputted tags.

File format of a library below:

[tags, tag1, tag tag tag another_tag more tags The entire string will show up who cares how its separated]
sd{ source links separated by whatever you want I guess it shows the whole string lol }

Main content--plain text--with potentially indefinite number of newlines.
An entry only "ends" because end of file or another entry began.
The syntax is very forgiving just have the right starting and ending characters per line.

# Include 
[dependencies]

library_reader = { git = "https://github.com/Alkajato/Sources", branch = "main" }
