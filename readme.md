

## Parition Mapping
Each sector is encrypted with an aspect-specific key. Upon decrpytion, there is
a header. The header contains a pointer to the next sector.

## Initial Partition Mapping

# Decoding Process
To decode a aspect, a keyphrase is supplied. From the keyphrase, a secure key is
generated. Then, the size of the disk is used to infer the number of sectors
(size / sector_size). The secure key is divded by the number of sectors, and the
remainder is a pointer to the 'head' sector of the aspect. Because of the
reductive nature of modulus, it is possible for multiple aspects to share a head
sector. This is handled.

With a head sector now identified, the secure key is extended to match the
sector size, and the entire sector and secure key buffer are XOR'd together. In
the resulting buffer, at a random offset, is two 32-bit integers, immediatley
adjacent. They are 0xffffffff, and the offset of the first data sector of your
aspect. Because the offsets are random, other aspects can have their offsets
encoded into the same sector, and there's no way to tell, provided that the
secure key is at least 64 bits (the size of the flag and pointer).

With the first data sector now identified, it is navigated to and decrypted,
using the secure key. The sector begins with a 32bit integer pointing to the
next sector, as well as a 32bit integer representing the index within the
aspect. The aspect index
