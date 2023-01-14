# Sources
CLI Program for parsing specially formatted text files based on inputted tags.                       
File format of a library below:

[tags tag1 tag another_tag more_tags duplicates do not add extra to scores for sorting entries]                         
{ source links separated by whitespace }

Main content--plain text--with potentially indefinite number of newlines.
An entry only "ends" because end of file or another entry began.
The syntax is very forgiving just have the right starting and ending characters per line.
