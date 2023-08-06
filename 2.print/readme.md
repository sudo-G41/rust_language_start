# ruet print
## print function
rust print is not function and method ㅋㅋㅋ... only macro  
println("string {}", values);

## print format
### align
{:<#} is left align #@    #  
{:>#} is right align #    @#  
{:^#} is center align #  @  #  
{:0align#} is left, right or center align after space -> 0 change #00@00#  

### Dec to ...
Dec 15 -> ...  
{:b} is binary number #1111#  
{:0#b} is similar {:>0#} but binary values #00001111#  
{:o} is binary number #17#  
{:0#o} is similar {:>0#} but octal values #017#  
{:x} is binary number #f#  
{:0#b} is similar {:>0#} but hex values #f#  

### other #
{:.#} is floating number round # #pie is 3.142#  
{:e} is exponential number #1000000 -> 1e6#  

### other format
{{, }} is {, }  
\", \\ is \", \  
{:p} is value pointer(value address)  

### debug print
{:?}, {:#?} is debug print(this # is not number, it's #) #?is align  

### values index
println("{0}, {2}, {1}"); -> 1value, 3value, 2value
values frist index 0 
