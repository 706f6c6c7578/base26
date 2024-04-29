# base26
base26 Encoder/Decoder.

Useful when one needs to encode messages in foreign languages, or small binary blobs, for the Diana Cryptosystem or the Dein Star straddling checkerboard.

Another scenario for base26 encoding is to encode small encrypted binary blobs, done with Format Preserving Encryption (FPE) and send them, organized in five letter groups, as text message (SMS), with an old feature phone. 

Usage: base26 [-d] [-l line_length] < infile > outfile

See also: https://pypi.org/project/base26/
