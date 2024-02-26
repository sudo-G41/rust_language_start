from rustpy import dict_make, hash2dict

a = ["q","w","e","r"]
b = [0,1,2,3,4]

qwer = dict_make(a,b)
print(qwer)

print(hash2dict(a,b))