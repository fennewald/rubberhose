# Sanity checks todo
* seed phrase size < hash length
* net filesystem overhead
* max aspect size

# Block Structure

Each block starts with an 8 byte header. Once the block is decrpyted, the
value may be read. The value is a random number that is equivalent to the block
id of the next sector when taken mod the total number of aspects on disk.
