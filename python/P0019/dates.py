from datetime import date
from datetime import timedelta

a    = date(1901, 1, 1)
delt = timedelta(days = 1)
sum  = 0;

while a < date(2000, 12, 31):
    if a.day == 1 and a.weekday() == 6:
        print(a)
        sum = sum + 1

    a = a+delt

print( f"Sundays on the first of a month from 1901-01-01 to 2000-12-31: {sum}" )
