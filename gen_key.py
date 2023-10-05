import random;
letters = [c for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ"]
#print(letters)
while letters:
    c = random.choice(letters)
    letters.remove(c)
    print(c, end="");
print();