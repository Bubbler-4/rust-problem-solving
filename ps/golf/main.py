n=int(input())
m=(n%5+n*3%5*3+n)//5
print([-1,m][m<n])