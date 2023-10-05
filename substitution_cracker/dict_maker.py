import io


bw = io.BufferedWriter(open('dict.txt', 'wb'))

# Open a source for reading
with open('american-englishOld', 'r') as source:
    for line in source:
        l = line.strip()\
            .replace('\'', '')\
            .replace(',', '')\
            .replace('.', '')\
            .upper() # Use strip() to remove leading/trailing whitespace
        if len(l) > 2:
            l += '\n'
            bw.write(l.encode('utf-8'))
        
        
bw.flush()
bw.close()
print("Data has been written")
