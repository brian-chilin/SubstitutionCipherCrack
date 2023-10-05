import io
import re

texts = [
    #"test.txt"
    "dict.txt",
    "chronicles of fairy land.txt",
    "shakespeare.txt",
    "king james bible.txt"
]

count = 0
trigrams = {}

for text in texts:
    file = open(text, 'rb')
    for line in file:
        count += 1
        if count%1000 == 0:
            print("completed", count, "lines")

        t = re.sub(r'[^a-zA-Z ]', '', line.decode('utf-8').strip())
        words = t.split(' ')
        for word in words:
            word = word.upper()
            if len(word) > 2: #skip any less than 3 length
                for i in range(len(word) - 2):
                    trigram = word[i:i+3]
                    if trigram in trigrams:
                        trigrams[trigram] = trigrams[trigram] + 1
                    else:
                        trigrams[trigram] = 1

print("done reading. beginning writing")
output_filename = 'trigrams.csv'
output_file = open(output_filename, 'w')
buffered_writer = open(output_filename, 'w', buffering=1)  # buffering=1 enables line buffering

data = trigrams.items()
data = sorted(data, key=lambda x: x[1], reverse=True)
for trigram, frequency in data: #trigrams.items():
    # Write data to the buffered writer
    buffered_writer.write(str(frequency) + "," + trigram + "\n")

buffered_writer.flush()
buffered_writer.close()
print("done")