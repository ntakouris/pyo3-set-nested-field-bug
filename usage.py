import testlib

if __name__=="__main__":
    beta = testlib.Beta(field=1)
    alpha = testlib.Alpha(beta)

    print(alpha.beta.field) # prints 1

    alpha.beta.field = 2

    print(alpha.beta.field) # prints 1
